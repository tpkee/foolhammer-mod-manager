use std::error::Error;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::utils::steam::SteamConfig;
use crate::{defaults::games::DefaultGameInfo, supported_games::SupportedGames};

#[derive(Debug)]
pub(crate) struct WindowsLauncher {
    command: Command,
    steam_config: SteamConfig,
    running_exe: Option<&'static str>, // use this to kill
}

impl WindowsLauncher {
    pub async fn new(app_handler: &tauri::AppHandle) -> Self {
        let command = Command::new("cmd");
        let steam_config = SteamConfig::from_app_handle(app_handler).unwrap_or(SteamConfig {
            steam_path: None,
            steam_library_path: None,
        });

        Self {
            command,
            steam_config,
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
        let game_id_str: String = game_id.into();
        log::info!(
            "WindowsLauncher::launch_game game={}, path={}, save={:?}",
            game_id_str,
            game_path.display(),
            save_path
        );

        if game_path
            .components()
            .into_iter()
            .any(|cmp| cmp.as_os_str() == "Steam")
        {
            // we have to make sure steam is running, otherwise the game will fail to launch.
            self.steam_config
                .run_steam()
                .map_err(|e| format!("Failed to start Steam: {:?}", e))?;
        }

        let game_preset =
            DefaultGameInfo::find_by_id(game_id).ok_or("Couldn't find game preset")?;

        let command = self.get_command();
        command.current_dir(game_path);
        command.raw_arg(game_preset.executable_name);
        command.arg("used_mods.txt;");

        if let Some(save_path) = save_path {
            log::info!("Loading save: {}", save_path.display());
            command
                .arg("game_startup_mode")
                .arg("campaign_load")
                .arg(save_path);
        }

        log::info!("Spawning game process");
        log::debug!("Running command: {:?}", command);

        let _ = command.spawn(); // do not wait

        self.running_exe = Some(game_preset.executable_name);
        Ok(())
    }

    fn kill_game(&mut self) -> Result<(), Box<dyn Error>> {
        if self.running_exe.is_none() {
            log::warn!("WindowsLauncher::kill_game: no running game");
            return Err("No running game found".into());
        }

        let sys = sysinfo::System::new_all();

        let exe_name = self.running_exe.as_ref().unwrap();
        let count = sys.processes_by_name(exe_name.as_ref()).count();
        log::info!("Killing {} process(es) matching '{}'", count, exe_name);

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
