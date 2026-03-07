use crate::{
    defaults::games::{DefaultGameInfo, SupportedGames},
    dto::{mods::ModRequestDto, profiles::ProfileRequestDto},
    mods::pack,
    resolve_existing_path,
    utils::{
        self, ErrorCode,
        path::{retrieve_saves_absolute_path, retrieve_steam_workshop_path},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tauri::Wry;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameStore {
    pub game_id: SupportedGames,

    pub game_path: PathBuf,
    pub saves_path: Option<PathBuf>,
    pub mods_path: PathBuf,
    pub profiles: Vec<Profile>,
    pub default_profile: Option<uuid::Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: uuid::Uuid,
    pub name: String,
    pub mods: Vec<ModInfo>,
    pub manual_mode: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    pub name: String,
    pub enabled: bool,
    pub order: u32,
}

impl From<ModRequestDto> for ModInfo {
    fn from(dto: ModRequestDto) -> Self {
        Self {
            name: dto.name,
            enabled: dto.enabled,
            order: dto.order.unwrap_or(0),
        }
    }
}

impl Profile {
    pub fn from_dto(id: Option<uuid::Uuid>, dto: ProfileRequestDto) -> Self {
        Self {
            id: id.unwrap_or_else(uuid::Uuid::new_v4),
            name: dto.name,
            mods: dto
                .mods
                .into_iter()
                .map(|m| ModInfo {
                    name: m.name,
                    enabled: m.enabled,
                    order: m.order.unwrap_or(0),
                })
                .collect(),
            manual_mode: dto.manual_mode.unwrap_or(false),
        }
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

        let default_profile = Profile::from_dto(
            None,
            ProfileRequestDto {
                game_id: default_game.game_id,
                name: default_profile_name.clone(),
                default: Some(true),
                manual_mode: Some(false),
                mods, // this is the default profile so we should throw all the available mods in it
            },
        );

        Some(Self {
            game_id: default_game.game_id,
            game_path,
            saves_path,
            mods_path,
            default_profile: Some(default_profile.id),
            profiles: vec![default_profile],
        })
    }

    pub fn to_hashmap(&self) -> Result<HashMap<String, serde_json::Value>, serde_json::Error> {
        serde_json::from_value(self.serialize(serde_json::value::Serializer)?)
    }

    pub fn from_entries(entries: Vec<(String, Value)>) -> Result<Self, ErrorCode> {
        let hm: HashMap<String, Value> = entries.into_iter().collect();
        serde_json::from_value(serde_json::json!(hm)).or(Err(ErrorCode::InternalError))
    }
}
