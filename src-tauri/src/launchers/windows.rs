use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{error::Error, io::Read};
use tauri::Manager;

use crate::defaults::system::STEAMDIR_INSTANCE;
use crate::{defaults::games::DefaultGameInfo, supported_games::SupportedGames};
use crate::{resolve_existing_path, utils};

const DETACHED_PROCESS: u32 = 0x00000008;
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

#[derive(Debug)]
pub(crate) struct WindowsLauncher {
    command: Command,
    running_exe: Option<&'static str>, // use this to kill
}

impl WindowsLauncher {
    pub async fn new(_app_handler: &tauri::AppHandle) -> Self {
        let command = Command::new("cmd");

        Self {
            command,
            running_exe: None,
        }
    }
}

impl super::GameManager for WindowsLauncher {
    fn launch_game(
        &mut self,
        game_id: SupportedGames,
        game_path: &Path,
        save_path: Option<&PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        if game_path
            .components()
            .into_iter()
            .any(|cmp| cmp.as_os_str() == "Steam")
        {
            // we have to check if there is a steam process running, otherwise umu will fail to launch the game.
            let sys = sysinfo::System::new_all();
            if sys.processes_by_name("steam".as_ref()).count() == 0 {
                let steam_exe = resolve_existing_path!(
                    STEAMDIR_INSTANCE
                        .as_ref()
                        .expect("Steam isn't installed")
                        .path(),
                    "steam.exe"
                )
                .unwrap();

                println!("Launching Steam from path: {}", steam_exe.to_string_lossy());

                let _ = Command::new(steam_exe)
                    .creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP)
                    .spawn()
                    .expect("Failed to launch Steam")
                    .wait();

                // std::thread::sleep(std::time::Duration::from_secs(10)); // steam is eepy, let's wait
            }
        }

        let game_preset =
            DefaultGameInfo::find_by_id(game_id).ok_or("Couldn't find game preset")?;

        let command = self.get_command();
        command.current_dir(game_path);
        println!(
            "Current dir set to: {}",
            command.get_current_dir().unwrap().to_string_lossy()
        );
        command.raw_arg(game_preset.executable_name);
        command.arg("used_mods.txt;");

        if let Some(save_path) = save_path {
            command
                .arg("game_startup_mode")
                .arg("campaign_load")
                .arg(save_path);
        }

        println!("Running command: {:?}", command);

        let _ = command.spawn().expect("Failed to start the game"); // do not wait

        self.running_exe = Some(game_preset.executable_name);
        Ok(())
    }

    fn kill_game(&mut self) -> Result<(), Box<dyn Error>> {
        if self.running_exe.is_none() {
            return Err("No running game found".into());
        }

        let sys = sysinfo::System::new_all();

        let exe_name = self.running_exe.as_ref().unwrap();

        for process in sys.processes_by_name(exe_name.as_ref()) {
            process.kill();
        }

        self.running_exe = None;

        Ok(())
    }

    fn get_command(&mut self) -> &mut Command {
        self.command.args(["/C", "start"])
    }
}
