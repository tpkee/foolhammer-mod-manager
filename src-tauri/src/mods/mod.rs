use std::{collections::HashMap, path::PathBuf};

use rpfm_lib::{games::manifest, utils::files_from_subdir};
use std::collections::HashSet;

use crate::join_path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pack {
    pub name: String,
    pub path: PathBuf,
    pub image: Option<PathBuf>,
    pub last_updated: Option<String>,
}

pub fn retrieve_mods(game_mods_path: &PathBuf) -> Result<Vec<Pack>, rpfm_lib::error::RLibError> {
    let manifest_path = join_path!(game_mods_path, "manifest.txt");
    let manifest = manifest::Manifest::read(&manifest_path);

    if manifest.is_err() {
        eprintln!(
            "Failed to read manifest from game mods path {:?}: {:?}",
            game_mods_path,
            manifest.as_ref().err()
        );
        return Err(manifest.err().unwrap());
    }

    let files = match files_from_subdir(game_mods_path, false) {
        Ok(files) => files,
        Err(e) => {
            eprintln!(
                "Failed to read mods from game mods path {:?}: {:?}",
                game_mods_path, e
            );
            return Err(e);
        }
    };

    let mut mods: HashSet<PathBuf> = HashSet::from_iter(files.into_iter());

    manifest.unwrap().0.iter().for_each(|entry| {
        // remove the vanilla packs
        let m = join_path!(game_mods_path, entry.relative_path());
        mods.remove(&m);
    });

    Ok(mods
        .iter()
        .filter_map(|path| match path.extension().and_then(|ext| ext.to_str()) {
            Some("pack") => Some(path_to_pack(path, None)),
            _ => None,
        })
        .collect())
}

fn path_to_pack(path: &PathBuf, image: Option<&PathBuf>) -> Pack {
    let name = path.file_stem().unwrap().to_string_lossy().to_string();
    let metadata: std::fs::Metadata = std::fs::metadata(&path).unwrap(); // we are sure it exists since we just got it
    let last_updated: Option<String> = metadata
        .modified()
        .ok()
        .and_then(|t| Some((chrono::DateTime::<chrono::Utc>::from(t)).to_rfc3339()));

    Pack {
        name,
        path: path.clone(),
        last_updated: last_updated,
        image: image.cloned(),
    }
}

pub fn retrieve_workshop_mods(
    steam_workshop_folder: &PathBuf,
    _game_id: &str, // TODO: we should be the one to actually build the path... somewhere!
) -> Result<Vec<Pack>, rpfm_lib::error::RLibError> {
    //let workshop_path = join_path!(steam_workshop_folder, game_id);
    let workshop_files = match files_from_subdir(&steam_workshop_folder, true) {
        Ok(files) => files,
        Err(e) => {
            eprintln!(
                "Failed to read workshop mods from path {:?}: {:?}",
                steam_workshop_folder, e
            );
            return Err(e);
        }
    };

    let mut map: HashMap<String, (Option<&PathBuf>, Option<&PathBuf>)> = HashMap::new(); // (pack_path, image_path)

    for file in &workshop_files {
        let file_name = file.file_stem().unwrap().to_string_lossy().to_string();
        let ext = file.extension().unwrap_or_default().to_str().unwrap();

        let entry = map.entry(file_name).or_insert(match ext {
            "pack" => (Some(file), None),
            "png" | "jpg" | "jpeg" => (None, Some(file)),
            _ => continue,
        });

        match ext {
            "pack" => {
                entry.0 = Some(file);
            }
            "png" | "jpg" | "jpeg" => {
                entry.1 = Some(file);
            }
            _ => {}
        }
    }

    Ok(map
        .into_values()
        .filter_map(|(pack_path, image_path)| {
            if let Some(pack_path) = pack_path {
                return Some(path_to_pack(pack_path, image_path));
            }

            None
        })
        .collect())
}
