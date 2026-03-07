use std::path::PathBuf;

use crate::{mods::pack::ModPack, stores::games::ProfileModInfo};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModResponseDto {
    pub name: String,
    pub path: Option<PathBuf>,
    pub groups: Option<Vec<uuid::Uuid>>,
    pub enabled: bool,
    pub order: u32,
    pub can_enable: bool,
    pub last_updated: Option<String>,
    pub from_steam_workshop: bool,
    pub image: Option<PathBuf>,
    pub dependencies: Option<Vec<(bool, String)>>, // this will be populated at runtime based on the mod pack's dependencies
}

impl ModResponseDto {
    pub fn new(mod_info: &ProfileModInfo, mod_pack: Option<ModPack>) -> Self {
        let (path, last_updated, from_steam_workshop, image, dependencies) = match mod_pack {
            Some(pack) => (
                Some(pack.path),
                pack.last_updated,
                pack.from_steam_workshop,
                pack.image,
                Some(pack.dependencies),
            ),
            None => (None, None, false, None, None),
        };

        let can_enable = path.is_some(); // this will be updated at runtime based on whether the mod file exists or not

        Self {
            order: mod_info.order,
            name: mod_info.name.clone(),
            path,
            groups: mod_info.groups.clone(),
            enabled: can_enable && mod_info.enabled,
            can_enable,
            last_updated, // this will be updated at runtime based on the mod file's last modified date
            from_steam_workshop, // this will be updated at runtime based on whether the mod is from the steam workshop or not
            image,
            dependencies,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModRequestDto {
    pub name: String,
    pub enabled: bool,
    pub groups: Option<Vec<uuid::Uuid>>,
    pub order: Option<u32>,
}
