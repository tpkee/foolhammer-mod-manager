use std::path::PathBuf;

use crate::{
    dto::{packs::PackResponseDto, profiles::ProfileResponseDto},
    mods::helpers,
    resolve_existing_path,
    stores::games,
    utils::path::retrieve_steam_workshop_path,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponseDto {
    pub mods: Vec<PackResponseDto>, // populated at runtime
    pub profiles: Vec<ProfileResponseDto>,
    pub game_id: String,
    pub game_path: PathBuf,
    pub saves_path: Option<PathBuf>,
    pub mods_path: PathBuf,
    pub workshop_path: Option<PathBuf>,
}

impl GameResponseDto {
    pub fn from_store(store: games::GameStore) -> Self {
        let mods_path = resolve_existing_path!(&store.mods_path);
        let workshop_path = retrieve_steam_workshop_path(&store.game_id);

        let mods = match mods_path {
            Some(path) => helpers::retrieve_mods(&path, &workshop_path),
            None => vec![],
        };

        Self {
            game_id: store.game_id,
            game_path: store.game_path,
            saves_path: store.saves_path,
            mods_path: store.mods_path,
            workshop_path: workshop_path,
            mods: Self::mods_to_dto(&mods),
            profiles: store
                .profiles
                .into_iter()
                .map(|profile| ProfileResponseDto::new(profile, &mods))
                .collect(),
        }
    }

    fn mods_to_dto(mods: &Vec<helpers::Pack>) -> Vec<PackResponseDto> {
        mods.into_iter()
            .map(|pack| {
                let pack = pack.clone();
                PackResponseDto {
                    name: pack.name,
                    path: pack.path,
                    image: pack.image,
                    last_updated: pack.last_updated,
                    from_steam_workshop: pack.from_steam_workshop,
                }
            })
            .collect()
    }
}
