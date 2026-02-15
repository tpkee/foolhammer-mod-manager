use std::{fs::File, io::Write, path::PathBuf};

use crate::utils;
use crate::utils::path::retrieve_wine_pfx_path;
use crate::{defaults, utils::umu_manager::generate_umu_command};
use crate::{
    dto::{games::GameResponseDto, mods::ModResponseDto},
    join_path,
    mods::pack::Pack,
    utils::{
        ErrorCode,
        umu_manager::{check_and_update, install_umu_launcher},
    },
};

pub async fn launch_game(
    app_handle: &tauri::AppHandle,
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

    if txt_path.exists() {
        std::fs::remove_file(&txt_path).expect("Failed to remove existing used_mods.txt file");
    }

    let mut f = File::options()
        .append(true)
        .create(true)
        .open(txt_path)
        .or(Err(ErrorCode::InternalError))?;

    generate_mods_file(&mut f, &mods)?;

    run_game(&app_handle, &game.game_id, &game_path, save_name).await
}

#[cfg(target_os = "linux")]
async fn run_game(
    app_handle: &tauri::AppHandle,
    game_id: &str,
    game_path: &PathBuf,
    save_name: Option<&str>,
) -> Result<(), ErrorCode> {
    use crate::defaults::games::DefaultGameInfo;

    let umu_status = utils::umu_manager::is_umu_available(&app_handle);

    match umu_status {
        utils::umu_manager::UmuAvailability::Local => {
            check_and_update(&app_handle)
                .await
                .expect("It wasn't possible to check for umu updates");
        }
        utils::umu_manager::UmuAvailability::NotAvailable => {
            install_umu_launcher(&app_handle)
                .await
                .expect("It wasn't possible to install umu");
        }
        _ => {}
    }

    let mut command =
        generate_umu_command(&app_handle).expect("Umu command not found, GUARDS GUARDS!");

    command.current_dir(game_path); // umu needs to be run in the game directory to find the used_mods.txt file

    println!("Umu available somewhere");

    if game_path
        .components()
        .into_iter()
        .find(|cmp| cmp.as_os_str() == "Steam")
        .is_some()
    {
        let pfx_path = retrieve_wine_pfx_path(game_id)
            .ok_or(ErrorCode::InternalError)
            .and_then(|pfx_path| {
                if !pfx_path.exists() {
                    return Err(ErrorCode::InternalError);
                }
                Ok(pfx_path)
            })?;

        let proton_version_file = File::open(pfx_path.join("version"));
        let mut proton_version = String::new();

        std::io::Read::read_to_string(
            &mut proton_version_file.expect("Couldn't read the proton version file"),
            &mut proton_version,
        )
        .expect("Couldn't read the proton version file");

        command.env("PROTONPATH", proton_version.trim());
        command.env("WINEPREFIX", pfx_path.join("pfx/"));
        command.env("SteamGameId", game_id);
    }

    let game_preset = DefaultGameInfo::find_by_id(game_id).ok_or(ErrorCode::NotFound)?;

    command.arg(&game_preset.executable_name);
    command.arg("used_mods.txt;");

    println!("Running command: {:?}", command);

    command.spawn().expect("Umu failed");
    Ok(())
}

#[cfg(target_os = "windows")]
fn run_game(
    _app_handle: &tauri::AppHandle,
    game_id: &str,
    game_path: &PathBuf,
    _save_name: Option<&str>,
) -> Result<(), ErrorCode> {
    todo!("Implement game launching on windows");
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
        writeln!(&mut file, "{}", workshop_line).or(Err(ErrorCode::InternalError))?;
    }

    for m in mods.iter() {
        let line = format!("mod \"{}.pack\";", m.name);
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
