use std::path::PathBuf;

use crate::{
    dto::{packs::PackResponseDto, profiles::ProfileResponseDto},
    mods::pack,
    resolve_existing_path,
    stores::games,
    utils::path::retrieve_steam_workshop_path,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponseDto {
    pub mods: Vec<PackResponseDto>, // populated at runtime
    pub profiles: Vec<ProfileResponseDto>,
    pub default_profile: Option<String>,
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

        let mut mods = match mods_path {
            Some(path) => pack::Pack::retrieve_mods(&path, &workshop_path),
            None => vec![],
        };

        let profiles: Vec<ProfileResponseDto> = store
            .profiles
            .into_iter()
            .map(|profile| ProfileResponseDto::new(profile, &mut mods))
            .collect();

        Self {
            game_id: store.game_id,
            game_path: store.game_path,
            saves_path: store.saves_path,
            mods_path: store.mods_path,
            workshop_path: workshop_path,
            mods: Self::mods_to_dto(&mods),
            default_profile: store.default_profile.or(profiles
                .first()
                .and_then(|profile| Some(profile.name.clone()))),
            profiles,
        }
    }

    fn mods_to_dto(mods: &Vec<pack::Pack>) -> Vec<PackResponseDto> {
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
