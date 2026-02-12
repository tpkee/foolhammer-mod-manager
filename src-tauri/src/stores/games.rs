use std::{collections::HashMap, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Wry;

use crate::{
    defaults::games,
    dto::{mods::ModRequestDto, profiles::ProfileRequestDto},
    mods::pack,
    resolve_existing_path,
    utils::{
        self, ErrorCode,
        path::{retrieve_saves_absolute_path, retrieve_steam_workshop_path},
    },
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    pub name: String,
    pub enabled: bool,
    pub order: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub name: String,
    pub default: bool,
    pub mods: Vec<ModInfo>,
    pub manual_mode: bool,
}

impl Profile {
    pub fn from_dto(dto: ProfileRequestDto) -> Self {
        Self {
            name: dto.name,
            default: dto.default.unwrap_or(false),
            mods: dto
                .mods
                .into_iter()
                .map(|m| ModInfo {
                    name: m.name,
                    enabled: m.enabled,
                    order: m.order,
                })
                .collect(),
            manual_mode: dto.manual_mode.unwrap_or(false),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameStore {
    pub game_id: String,

    pub game_path: PathBuf,
    pub saves_path: Option<PathBuf>,
    pub mods_path: PathBuf,
    pub profiles: Vec<Profile>,
}

impl GameStore {
    pub fn new(
        app_handle: &tauri::AppHandle,
        game_id: &str,
    ) -> Result<Arc<tauri_plugin_store::Store<Wry>>, ErrorCode> {
        let default_game = Self::new_game(game_id)
            .ok_or(ErrorCode::NotFound)?
            .to_hashmap()
            .or(Err(ErrorCode::InternalError))?;

        let game_conf_path =
            utils::path::generate_store_path(app_handle, format!("{}.json", game_id).as_str());

        let store = tauri_plugin_store::StoreBuilder::new(app_handle, game_conf_path)
            .defaults(default_game)
            .build()
            .or(Err(ErrorCode::InternalError))?;

        // TODO: if the store is new should we start the watcher? not sure, probs better to handle this logic separately

        Ok(store)
    }

    fn new_game(game_id: &str) -> Option<Self> {
        let default_game = games::SUPPORTED_GAMES
            .iter()
            .find(|game| game_id == game.game_id)?;

        let game_path = default_game.get_game_path()?;
        let saves_path = retrieve_saves_absolute_path(default_game.game_id);
        let mods_path = resolve_existing_path!(&game_path, default_game.mods_path)?;

        let workshop_path: Option<PathBuf> = retrieve_steam_workshop_path(&default_game.game_id);
        let mods: Vec<ModRequestDto> = pack::Pack::retrieve_mods(&mods_path, &workshop_path)
            .iter()
            .map(|mod_pack| ModRequestDto {
                order: 0,
                enabled: false,
                name: mod_pack.name.clone(),
            })
            .collect();

        let default_profile = Profile::from_dto(ProfileRequestDto {
            game_id: default_game.game_id.to_string(),
            name: "Default".to_string(),
            default: Some(true),
            manual_mode: Some(false),
            mods, // this is the default profile so we should throw all the available mods in it
        });

        Some(Self {
            game_id: default_game.game_id.to_string(),
            game_path,
            saves_path,
            mods_path,
            profiles: vec![default_profile],
        })
    }

    fn to_hashmap(&self) -> Result<HashMap<String, serde_json::Value>, serde_json::Error> {
        // TODO: maybe this should be moved to a trait impl? not sure if we will need this for other structs
        let hm: HashMap<String, Value> =
            serde_json::from_value(self.serialize(serde_json::value::Serializer)?)?;
        Ok(hm)
    }

    pub fn from_entries(entries: Vec<(String, Value)>) -> Result<Self, ErrorCode> {
        let hm: HashMap<String, Value> = entries.into_iter().collect();
        let game_store: Self =
            serde_json::from_value(serde_json::json!(hm)).or(Err(ErrorCode::InternalError))?;
        Ok(game_store)
    }
}
