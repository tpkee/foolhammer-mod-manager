use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{error::Error, io::Read};
use tauri::Manager;

use crate::utils;
use crate::utils::steam::SteamConfig;
use crate::{defaults::games::DefaultGameInfo, supported_games::SupportedGames};

#[derive(Debug, Serialize, Deserialize, Clone)]

struct LauncherRelease {
    release_date: String,
    url: String,
}

#[derive(Debug)]
pub(crate) struct LinuxLauncher {
    runner_path: PathBuf,
    command: Command,
    steam_config: SteamConfig,
    running_exe: Option<&'static str>, // use this to kill
}

impl LinuxLauncher {
    pub async fn new(app_handler: &tauri::AppHandle) -> Self {
        let launcher_path = Self::update_or_install(app_handler)
            .await
            .expect("Failed to update or install umu-launcher");

        let command = Command::new("python");
        let steam_config = SteamConfig::from_app_handle(app_handler).unwrap_or(SteamConfig {
            steam_path: None,
            steam_library_path: None,
        });

        Self {
            runner_path: launcher_path,
            command,
            steam_config,
            running_exe: None,
        }
    }

    fn get_launcher_base_path(app_handler: &tauri::AppHandle) -> Result<PathBuf, Box<dyn Error>> {
        let app_data = app_handler.path().app_data_dir()?;
        Ok(app_data.join("umu-launcher/"))
    }

    // returns the path of the executable itself
    async fn update_or_install(app_handler: &tauri::AppHandle) -> Result<PathBuf, Box<dyn Error>> {
        let launcher_dir = Self::get_launcher_base_path(app_handler)?;

        let launcher = Self::get_runner_release().await?;

        let version_file_path = launcher_dir.join("version");

        if version_file_path.exists() {
            let mut version_file = File::open(launcher_dir.join("version"))?;

            let mut current_version = String::new();

            version_file.read_to_string(&mut current_version)?;

            let current_version_date =
                chrono::DateTime::parse_from_rfc3339(&current_version)?.with_timezone(&chrono::Utc);
            let launcher_release_date =
                chrono::DateTime::parse_from_rfc3339(&launcher.release_date)?
                    .with_timezone(&chrono::Utc);

            if launcher_release_date <= current_version_date {
                log::info!(
                    "umu-launcher up to date (current={}, remote={})",
                    current_version,
                    launcher.release_date
                );
                return Ok(launcher_dir.join("umu/umu-run"));
            }
        }

        log::info!(
            "Downloading latest umu-launcher (release_date={})",
            launcher.release_date
        );

        let mut res = utils::download(app_handler, &launcher.url, "linux-runner").await?;

        let mut tar = tar::Archive::new(res.body_mut().as_reader());

        log::info!("Extracting linux runner to {}", launcher_dir.display());

        tar.unpack(&launcher_dir)?;

        let mut version_file = File::create(launcher_dir.join("version"))?; // write the release date to a version file
        version_file.write_all(launcher.release_date.as_bytes())?;

        Ok(launcher_dir.join("umu/umu-run"))
    }

    async fn get_runner_release() -> Result<LauncherRelease, Box<dyn Error>> {
        let res = ureq::get(
            "https://api.github.com/repos/Open-Wine-Components/umu-launcher/releases/latest",
        )
        .call()?
        .body_mut()
        .read_json::<serde_json::Value>()?;

        let Some(release_date) = res.get("updated_at").and_then(|v| v.as_str()) else {
            return Err("Failed to get updated_at from GitHub API response".into());
        };

        let Some(assets) = res.get("assets").and_then(|v| v.as_array()) else {
            return Err("Failed to get assets from GitHub API response".into());
        };

        let zip: Option<&serde_json::Value> = assets.iter().find(|asset| {
            let Some(o) = asset.as_object() else {
                return false;
            };

            let Some(name) = o.get("name").and_then(|v| v.as_str()) else {
                return false;
            };

            name.contains("zipapp")
        });

        let zipapp_url = zip
            .and_then(|z| z.get("browser_download_url"))
            .and_then(|v| v.as_str())
            .ok_or("Failed to get zipapp URL from GitHub API response")?;

        Ok(LauncherRelease {
            release_date: release_date.to_string(),
            url: zipapp_url.to_string(),
        })
    }
}

impl super::GameManager for LinuxLauncher {
    fn launch_game(
        &mut self,
        game_id: SupportedGames,
        game_path: &Path,
        save_path: Option<&PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        let game_id_str: String = game_id.into();
        log::info!(
            "LinuxLauncher::launch_game game={}, path={}, save={:?}",
            game_id_str,
            game_path.display(),
            save_path
        );
        let steam_config = self.steam_config.clone();

        let command = self.get_command();

        command.current_dir(game_path); // umu needs to be run in the game directory to find the used_mods.txt file

        if game_path
            .components()
            .into_iter()
            .any(|cmp| cmp.as_os_str() == "Steam")
        {
            let pfx_path = steam_config
                .retrieve_wine_pfx_path(game_id)
                .ok_or("Failed to find wine prefix path")?;

            let proton_version_file = File::open(pfx_path.join("version"));
            let mut proton_version = String::new();

            std::io::Read::read_to_string(
                &mut proton_version_file.expect("Couldn't read the proton version file"),
                &mut proton_version,
            )
            .expect("Couldn't parse the proton version file");

            log::info!("Wine prefix: {}", pfx_path.display());
            log::info!("PROTONPATH={}", proton_version.trim());
            command.env("PROTONPATH", proton_version.trim());
            command.env("WINEPREFIX", pfx_path.join("pfx/"));
            command.env("SteamGameId", &game_id_str);

            // we have to make sure steam is running, otherwise umu will fail to launch the game.
            steam_config
                .run_steam()
                .map_err(|e| format!("Failed to start Steam: {:?}", e))?;
        }

        let game_preset =
            DefaultGameInfo::find_by_id(game_id).ok_or("Couldn't find game preset")?;

        command.arg(game_preset.executable_name);
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

        let _ = command.spawn().expect("Umu failed");

        self.running_exe = Some(game_preset.executable_name);
        Ok(())
    }

    fn kill_game(&mut self) -> Result<(), Box<dyn Error>> {
        if self.running_exe.is_none() {
            log::warn!("LinuxLauncher::kill_game: no running game");
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
        self.command.arg(&self.runner_path)
    }
}
