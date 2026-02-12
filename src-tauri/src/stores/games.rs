use std::{collections::HashMap, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Wry;

use crate::{
    defaults::games,
    dto::profiles::ProfileRequestDto,
    resolve_existing_path,
    utils::{self, ErrorCode, path::retrieve_saves_absolute_path},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    #[serde(rename = "$key$")]
    name: String,
    enabled: bool,
    order: Option<u32>, // even if it's Some it will be ignored if the profile's manual mode is enabled
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    name: String,

    mods: Vec<ModInfo>,
    manual_mode: bool,
}

impl Profile {
    pub fn from_dto(dto: ProfileRequestDto) -> Self {
        Self {
            name: dto.name,
            mods: dto
                .mods
                .into_iter()
                .map(|m| ModInfo {
                    name: m.name,
                    enabled: m.enabled,
                    order: Some(m.order),
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
        let default_game = GameStore::new_game(game_id)
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

        let default_profile = Profile::from_dto(ProfileRequestDto {
            game_id: default_game.game_id.to_string(),
            name: "Default".to_string(),
            default: Some(true),
            manual_mode: Some(false),
            mods: vec![],
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
}
