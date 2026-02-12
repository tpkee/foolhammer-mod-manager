use crate::{
    dto::mods::{ModRequestDto, ModResponseDto},
    mods::helpers::Pack,
    stores::games,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponseDto {
    pub name: String, // name must be unique across profiles, but not necessarily across games
    pub mods: Vec<ModResponseDto>,
    pub default: bool,
    pub manual_mode: bool,
}

impl ProfileResponseDto {
    pub fn new(profile: games::Profile, mods: &Vec<Pack>) -> Self {
        Self {
            mods: Self::map_mods_to_dto(&profile, &mods),
            name: profile.name,
            default: profile.default,
            manual_mode: profile.manual_mode,
        }
    }

    fn map_mods_to_dto(profile: &games::Profile, mods: &Vec<Pack>) -> Vec<ModResponseDto> {
        profile
            .mods
            .iter()
            .map(|mod_info| {
                ModResponseDto::new(
                    &mod_info,
                    mods.iter()
                        .find(|pack: &&Pack| pack.name == mod_info.name)
                        .cloned(),
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileRequestDto {
    pub game_id: String,
    pub name: String,
    pub default: Option<bool>,
    pub manual_mode: Option<bool>,
    pub mods: Vec<ModRequestDto>,
}
