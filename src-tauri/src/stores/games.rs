use crate::{
    defaults::games::DefaultGameInfo,
    dto::{groups::GroupRequestDto, mods::ModRequestDto, profiles::ProfileRequestDto},
    mods::pack,
    resolve_existing_path,
    supported_games::SupportedGames,
    utils::{
        self, ErrorCode,
        path::{retrieve_saves_absolute_path, retrieve_steam_workshop_path},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tauri::Wry;

pub(crate) trait Store<T> {
    fn find_by_id(
        app_handle: &tauri::AppHandle,
        game_id: SupportedGames,
        lookup_id: uuid::Uuid,
    ) -> Option<T>;

    fn id(item: &T) -> uuid::Uuid;
    fn collection_mut(game: &mut GameStore) -> &mut Vec<T>;

    async fn get_all<F, R>(
        app_handle: &tauri::AppHandle,
        game_id: SupportedGames,
        f: F,
    ) -> Result<R, ErrorCode>
    where
        F: FnOnce(&mut Vec<T>) -> Result<R, ErrorCode>,
    {
        GameStore::get(app_handle, game_id, |game| f(Self::collection_mut(game))).await
    }

    async fn get<F, R>(
        app_handle: &tauri::AppHandle,
        game_id: SupportedGames,
        lookup_id: uuid::Uuid,
        f: F,
    ) -> Result<R, ErrorCode>
    where
        F: FnOnce(&mut T) -> Result<R, ErrorCode>,
    {
        Self::get_all(app_handle, game_id, |items| {
            items
                .iter_mut()
                .find(|item| Self::id(item) == lookup_id)
                .ok_or(ErrorCode::NotFound)
                .and_then(f)
        })
        .await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameStore {
    pub game_id: SupportedGames,
    pub game_path: PathBuf,
    pub saves_path: Option<PathBuf>,
    pub mods_path: PathBuf,
    pub profiles: Vec<Profile>,
    pub groups: Vec<Group>,
    pub default_profile: Option<uuid::Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub mods: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: uuid::Uuid,
    pub name: String,
    pub mods: Vec<ProfileModInfo>,
    pub manual_mode: bool,
    pub groups: Vec<uuid::Uuid>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileModInfo {
    pub name: String,
    pub enabled: bool,
    pub groups: Option<Vec<uuid::Uuid>>, // if a mod belongs to a group we need to keep a reference in the profile itself
    pub order: u32,                      // TODO: this should be an option
}

impl From<ModRequestDto> for ProfileModInfo {
    fn from(dto: ModRequestDto) -> Self {
        Self {
            name: dto.name,
            enabled: dto.enabled,
            groups: None,
            order: dto.order.unwrap_or(0),
        }
    }
}

impl From<ProfileRequestDto> for Profile {
    fn from(dto: ProfileRequestDto) -> Self {
        Self {
            id: dto.id.unwrap_or_else(uuid::Uuid::new_v4),
            name: dto.name,
            mods: dto
                .mods
                .into_iter()
                .map(|m| ProfileModInfo {
                    name: m.name,
                    enabled: m.enabled,
                    groups: None,
                    order: m.order.unwrap_or(0),
                })
                .collect(),
            manual_mode: dto.manual_mode.unwrap_or(false),
            groups: dto.groups,
        }
    }
}

impl From<GroupRequestDto> for Group {
    fn from(dto: GroupRequestDto) -> Self {
        Self {
            id: dto.id.unwrap_or_else(uuid::Uuid::new_v4),
            name: dto.name,
            mods: dto.mods,
        }
    }
}

impl Store<Group> for Group {
    fn find_by_id(
        app_handle: &tauri::AppHandle,
        game_id: SupportedGames,
        lookup_id: uuid::Uuid,
    ) -> Option<Group> {
        let store = GameStore::get_store(app_handle, game_id).ok()?;
        let game = GameStore::from_entries(store.entries()).ok()?;

        game.groups.iter().find(|g| g.id == lookup_id).cloned()
    }

    fn id(item: &Group) -> uuid::Uuid {
        item.id
    }

    fn collection_mut(game: &mut GameStore) -> &mut Vec<Group> {
        &mut game.groups
    }
}

impl Store<Profile> for Profile {
    fn find_by_id(
        app_handle: &tauri::AppHandle,
        game_id: SupportedGames,
        lookup_id: uuid::Uuid,
    ) -> Option<Profile> {
        let store = GameStore::get_store(app_handle, game_id).ok()?;
        let game = GameStore::from_entries(store.entries()).ok()?;

        game.profiles.iter().find(|p| p.id == lookup_id).cloned()
    }

    fn id(item: &Profile) -> uuid::Uuid {
        item.id
    }

    fn collection_mut(game: &mut GameStore) -> &mut Vec<Profile> {
        &mut game.profiles
    }
}

impl GameStore {
    pub fn get_store(
        app_handle: &tauri::AppHandle,
        game_id: SupportedGames,
    ) -> Result<Arc<tauri_plugin_store::Store<Wry>>, ErrorCode> {
        let default_game = Self::new_game(game_id)
            .ok_or(ErrorCode::NotFound)?
            .to_hashmap()
            .or(Err(ErrorCode::InternalError))?;

        let game_conf_path =
            utils::path::generate_store_path(app_handle, &format!("{}.json", game_id));

        let store = tauri_plugin_store::StoreBuilder::new(app_handle, game_conf_path)
            .defaults(default_game)
            .build()
            .or(Err(ErrorCode::InternalError))?;

        Ok(store)
    }

    fn new_game(game_id: SupportedGames) -> Option<Self> {
        let default_game = DefaultGameInfo::find_by_id(game_id)?;

        let game_path = default_game.get_game_path()?;
        let saves_path =
            retrieve_saves_absolute_path(default_game.game_id, default_game.saves_path);
        let mods_path = resolve_existing_path!(&game_path, default_game.mods_path)?;

        let workshop_path: Option<PathBuf> = retrieve_steam_workshop_path(default_game.game_id);
        let mods: Vec<ModRequestDto> = pack::ModPack::retrieve_mods(&mods_path, &workshop_path)
            .iter()
            .map(|mod_pack| ModRequestDto {
                order: None,
                enabled: false,
                name: mod_pack.name.clone(),
            })
            .collect();

        let default_profile_name = String::from("Default");

        let default_profile = Profile::from(ProfileRequestDto {
            id: None,
            game_id: default_game.game_id,
            name: default_profile_name.clone(),
            default: Some(true),
            manual_mode: Some(false),
            groups: vec![],
            mods, // this is the default profile so we should throw all the available mods in it
        });

        Some(Self {
            game_id: default_game.game_id,
            game_path,
            saves_path,
            mods_path,
            default_profile: Some(default_profile.id),
            profiles: vec![default_profile],
            groups: vec![],
        })
    }

    pub fn to_hashmap(&self) -> Result<HashMap<String, serde_json::Value>, serde_json::Error> {
        serde_json::from_value(self.serialize(serde_json::value::Serializer)?)
    }

    pub fn from_entries(entries: Vec<(String, Value)>) -> Result<Self, ErrorCode> {
        let hm: HashMap<String, Value> = entries.into_iter().collect();
        serde_json::from_value(serde_json::json!(hm)).or(Err(ErrorCode::InternalError))
    }

    pub async fn get<F, R>(
        app_handle: &tauri::AppHandle,
        game_id: SupportedGames,
        f: F,
    ) -> Result<R, ErrorCode>
    where
        F: FnOnce(&mut GameStore) -> Result<R, ErrorCode>,
    {
        let store = GameStore::get_store(app_handle, game_id)?;
        let mut game = GameStore::from_entries(store.entries())?;

        let result = f(&mut game)?;

        for (k, v) in game.to_hashmap().or(Err(ErrorCode::InternalError))? {
            store.set(k, v);
        }

        store.save().map_err(|e| {
            eprintln!("Failed to save game: {:?}", e);
            ErrorCode::InternalError
        })?;

        Ok(result)
    }
}
