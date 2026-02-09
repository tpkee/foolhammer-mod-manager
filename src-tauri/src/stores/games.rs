use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{KeyValueMap, serde_as};

use crate::{defaults::games, resolve_existing_path, utils::retrieve_saves_absolute_path};

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
    #[serde(rename = "$key$")]
    name: String,

    mods: Vec<ModInfo>,
    manual_mode: bool,
}

impl Profile {
    pub fn from_dto(dto: crate::dto::profiles::ProfileRequestDto) -> Self {
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
    #[serde(rename = "$key$")]
    pub game_id: String,

    pub game_path: PathBuf,
    pub saves_path: Option<PathBuf>,
    pub mods_path: PathBuf,
    pub profiles: Vec<Profile>,
}

impl GameStore {
    pub fn new(game_id: &str) -> Option<Self> {
        let default_game = games::SUPPORTED_GAMES
            .iter()
            .find(|game| game_id == game.game_id)?;

        let game_path = default_game.get_game_path()?;
        let saves_path = retrieve_saves_absolute_path(default_game.game_id);
        let mods_path = resolve_existing_path!(&game_path, default_game.mods_path)?;

        Some(Self {
            game_id: default_game.game_id.to_string(),
            game_path,
            saves_path,
            mods_path,
            profiles: vec![],
        })
    }

    pub fn to_hashmap(&self) -> Result<HashMap<String, serde_json::Value>, serde_json::Error> {
        let hm: HashMap<String, Value> =
            serde_json::from_value(self.serialize(serde_json::value::Serializer)?)?;
        Ok(hm)
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct KVMap(#[serde_as(as = "KeyValueMap<_>")] Vec<GameStore>);
