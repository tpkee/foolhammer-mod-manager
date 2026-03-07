use std::path::{Path, PathBuf};

use crate::{
    dto::{
        groups::GroupResponseDto, packs::PackResponseDto, profiles::ProfileResponseDto,
        saves::SaveResponseDto,
    },
    mods::pack,
    resolve_existing_path,
    stores::games,
    supported_games::SupportedGames,
    utils::path::retrieve_steam_workshop_path,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameRequestDto {
    pub saves_path: Option<PathBuf>,
    pub mods_path: PathBuf,
    pub game_path: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponseDto {
    pub mods: Vec<PackResponseDto>, // populated at runtime
    pub profiles: Vec<ProfileResponseDto>,
    pub groups: Vec<GroupResponseDto>,
    pub saves: Vec<SaveResponseDto>, // TODO: implement saves
    pub default_profile: Option<uuid::Uuid>,
    pub game_id: SupportedGames,
    pub game_path: PathBuf,
    pub saves_path: Option<PathBuf>,
    pub mods_path: PathBuf,
    pub workshop_path: Option<PathBuf>,
}

impl GameResponseDto {
    pub fn from_store(store: games::GameStore) -> Self {
        let mods_path = resolve_existing_path!(&store.mods_path);
        let workshop_path = retrieve_steam_workshop_path(store.game_id);

        let mods = match mods_path {
            Some(path) => pack::ModPack::retrieve_mods(&path, &workshop_path),
            None => vec![],
        };

        let profiles: Vec<ProfileResponseDto> = store
            .profiles
            .into_iter()
            .map(|profile| ProfileResponseDto::new(profile, &mods))
            .collect();

        let saves = match &store.saves_path {
            Some(path) => Self::get_saves(path),
            None => vec![],
        };

        Self {
            game_id: store.game_id,
            game_path: store.game_path,
            saves_path: store.saves_path,
            mods_path: store.mods_path,
            workshop_path,
            saves,
            mods: Self::mods_to_dto(&mods),
            default_profile: store
                .default_profile
                .or(profiles.first().map(|profile| profile.id)),
            profiles,
            groups: store
                .groups
                .into_iter()
                .map(GroupResponseDto::from)
                .collect(),
        }
    }

    fn get_saves(folder_path: &Path) -> Vec<SaveResponseDto> {
        let Ok(paths) = std::fs::read_dir(folder_path) else {
            eprintln!("Failed to read saves directory: {:?}", folder_path);
            return vec![];
        };

        if let Ok(saves) = paths
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
        {
            return saves.into_iter().map(SaveResponseDto::new).collect();
        }

        vec![]
    }

    fn mods_to_dto(mods: &[pack::ModPack]) -> Vec<PackResponseDto> {
        mods.iter()
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
