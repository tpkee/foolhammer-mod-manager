use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::{
    dto::{games::GameResponseDto, mods::ModResponseDto},
    join_path,
    mods::pack::Pack,
    utils::ErrorCode,
};

pub fn launch_game(
    game: &GameResponseDto,
    profile_mods: &Vec<ModResponseDto>,
    save_name: Option<&str>,
) -> Result<(), ErrorCode> {
    let &GameResponseDto {
        game_path,
        saves_path,
        mods_path,
        workshop_path,
        ..
    } = &game;

    if !game_path.exists() {
        return Err(ErrorCode::InternalError);
    }

    let mods = parse_mods(&profile_mods, &mods_path, &workshop_path);

    let txt_path = join_path!(&game_path, "used_mods.txt");

    let mut f = File::options()
        .append(true)
        .open(txt_path)
        .or(Err(ErrorCode::InternalError))?;

    generate_mods_file(&mut f, &mods)?;

    //std::process::Command::new("proton-tricks") // TODO: maybe integrate the proton launcher directly?

    Ok(())
}

fn generate_mods_file(mut file: &mut File, mods: &Vec<ModToLoad>) -> Result<(), ErrorCode> {
    // add_working_directory "Z:/home/<username>/.local/share/Steam/steamapps/workshop/content/<gameId>/<modId>";
    // mod "<name>.pack";
    let workshop_dirs = mods
        .iter()
        .filter(|m| m.from_steam_workshop)
        .map(|m| m.path.clone())
        .collect::<Vec<_>>();

    let folder_prefix = if std::env::consts::OS == "linux" {
        "Z:/"
    } else {
        "" // TODO: test if on windows the scandir returns the path with the drive prefix
    };

    for mut workshop_dir in workshop_dirs {
        workshop_dir.pop(); // we know this is safe since the path has to exist to be a valid mod
        let workshop_line = format!(
            "add_working_directory \"{}{}\";\n",
            folder_prefix,
            workshop_dir.to_string_lossy()
        );
        writeln!(&mut file, "{}", workshop_line).or(Err(ErrorCode::InternalError))?;
    }

    for m in mods.iter() {
        let line = format!("mod \"{}\";\n", m.name);
        writeln!(&mut file, "{}", line).or(Err(ErrorCode::InternalError))?;
    }

    Ok(())
}

#[derive(Debug)]
struct ModToLoad {
    pub name: String,
    pub path: PathBuf,
    pub order: u32,
    pub from_steam_workshop: bool,
}

fn parse_mods(
    profile_mods: &Vec<ModResponseDto>,
    mods_path: &PathBuf,
    workshop_path: &Option<PathBuf>,
) -> Vec<ModToLoad> {
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

        let mod_to_load: ModToLoad = ModToLoad {
            name: disk_mod.name.clone(),
            path: disk_mod.path.clone(),
            order: m.order,
            from_steam_workshop: disk_mod.from_steam_workshop,
        };
        mods.push(mod_to_load);
    }

    mods.sort_by(|a, b| a.order.cmp(&b.order));

    mods
}
