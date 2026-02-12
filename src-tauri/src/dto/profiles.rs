use crate::{
    dto::mods::{ModRequestDto, ModResponseDto},
    mods::{pack::Pack, sort::SortMods},
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
    pub fn new(mut profile: games::Profile, game_mods: &Vec<Pack>) -> Self {
        profile.mods.sort_mods(|m| &m.name);

        let mapped_mods = Self::map_mods_to_dto(&mut profile, &game_mods);

        Self {
            mods: mapped_mods,
            name: profile.name,
            default: profile.default,
            manual_mode: profile.manual_mode,
        }
    }

    fn map_mods_to_dto(profile: &mut games::Profile, mods: &Vec<Pack>) -> Vec<ModResponseDto> {
        if !profile.manual_mode {
            for i in 0..profile.mods.len() {
                profile.mods[i].order = (u32::try_from(i)
                    .expect("u32 overflow, it wasn't possible to convert usize to u32"))
                    + 1;
            }
        }

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
