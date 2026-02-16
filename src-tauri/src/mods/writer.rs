use std::{fs::File, io::Write, path::PathBuf};

use crate::{dto::mods::ModResponseDto, mods::pack::Pack};

#[derive(Debug)]
pub struct Mod {
    pub name: String,
    pub path: PathBuf,
    pub order: u32,
    pub from_steam_workshop: bool,
}

pub(crate) struct ModWriter {
    mods: Vec<Mod>,
}

impl ModWriter {
    pub fn new(
        profile_mods: &Vec<ModResponseDto>,
        mods_path: &PathBuf,
        workshop_path: &Option<PathBuf>,
    ) -> Self {
        let disk_mods = Pack::retrieve_mods(mods_path, workshop_path);

        let mut mods = vec![];

        for m in profile_mods.iter() {
            if !m.enabled {
                continue;
            }

            let disk_mod = match disk_mods.iter().find(|dm| dm.name == m.name) {
                Some(dm) => dm,
                None => {
                    eprintln!(
                        "Warning: Mod '{}' is enabled in the profile but not found on disk. Skipping.",
                        m.name
                    );
                    continue;
                }
            };

            let mod_to_load = Mod {
                name: disk_mod.name.clone(),
                path: disk_mod.path.clone(),
                order: m.order,
                from_steam_workshop: disk_mod.from_steam_workshop,
            };
            mods.push(mod_to_load);
        }

        mods.sort_by(|a, b| a.order.cmp(&b.order));

        Self { mods }
    }

    pub fn write(self, txt_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if txt_path.exists() {
            std::fs::remove_file(&txt_path)?;
        }
        let mut file = File::options().append(true).create(true).open(txt_path)?;

        // add_working_directory "[Z: or C: on Windows]\home\<username>\.local\share\Steam\steamapps\workshop\content\<gameId>\<modId>";
        // mod "<name>.pack";
        let workshop_dirs = self
            .mods
            .iter()
            .filter(|m| m.from_steam_workshop)
            .map(|m| m.path.clone())
            .collect::<Vec<_>>();

        let folder_prefix = if cfg!(target_os = "linux") {
            "Z:"
        } else {
            "" // TODO: test if on windows the scandir returns the path with the drive prefix
        };

        for mut workshop_dir in workshop_dirs {
            workshop_dir.pop(); // we know this is safe since the path has to exist to be a valid mod
            let workshop_line = format!(
                "add_working_directory \"{}{}\";",
                folder_prefix,
                workshop_dir.to_string_lossy().replace("/", "\\") // this sucks but so does windows
            );
            writeln!(&mut file, "{}", workshop_line)?;
        }

        for m in self.mods.iter() {
            let line = format!("mod \"{}.pack\";", m.name);
            writeln!(&mut file, "{}", line)?;
        }

        Ok(())
    }
}
