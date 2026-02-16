use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{error::Error, io::Read};
use tauri::Manager;

use crate::defaults::games::DefaultGameInfo;
use crate::utils::{self};

#[derive(Debug, Serialize, Deserialize, Clone)]

struct LauncherRelease {
    release_date: String,
    url: String,
}

#[derive(Debug)]
pub(crate) struct LinuxLauncher {
    runner_path: PathBuf,
    command: Command,
    running_exe: Option<&'static str>, // use this to kill
}

impl LinuxLauncher {
    pub async fn new(app_handler: &tauri::AppHandle) -> Self {
        let launcher_path = Self::update_or_install(app_handler)
            .await
            .expect("Failed to update or install umu-launcher");

        let command = Command::new("python");

        Self {
            runner_path: launcher_path,
            command,
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

        let launcher = Self::get_runner_release().await.or_else(|e| Err(e))?;

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
                // up to date
                return Ok(launcher_dir.join("umu/umu-run"));
            }
        }

        println!(
            "Downloading Latest runner (umu-launcher) version: {:?}",
            launcher
        );

        let mut res = utils::download(&app_handler, &launcher.url, "linux-runner").await?;

        let mut tar = tar::Archive::new(res.body_mut().as_reader());

        println!("Extracting linux runner to {:?}", launcher_dir);

        tar.unpack(&launcher_dir)?;

        let mut version_file = File::create(launcher_dir.join("version"))?; // write the release date to a version file
        version_file.write_all(launcher.release_date.as_bytes())?;

        Ok(launcher_dir.join("umu/umu-run"))
    }

    async fn get_runner_release() -> Result<LauncherRelease, Box<dyn Error>> {
        let res = ureq::get(
            "https://api.github.com/repos/Open-Wine-Components/umu-launcher/releases/latest",
        )
        .call()
        .or_else(|e| return Err(e))?
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
        game_id: &str,
        game_path: &PathBuf,
        save_path: Option<&PathBuf>, //TODO:
    ) -> Result<(), Box<dyn Error>> {
        let command = self.get_command();

        command.current_dir(game_path); // umu needs to be run in the game directory to find the used_mods.txt file

        if game_path
            .components()
            .into_iter()
            .find(|cmp| cmp.as_os_str() == "Steam")
            .is_some()
        {
            let pfx_path = utils::path::retrieve_wine_pfx_path(game_id)
                .ok_or("Failed to find wine prefix path")?;

            let proton_version_file = File::open(pfx_path.join("version"));
            let mut proton_version = String::new();

            std::io::Read::read_to_string(
                &mut proton_version_file.expect("Couldn't read the proton version file"),
                &mut proton_version,
            )
            .expect("Couldn't parse the proton version file");

            command.env("PROTONPATH", proton_version.trim());
            command.env("WINEPREFIX", pfx_path.join("pfx/"));
            command.env("SteamGameId", game_id);

            // we have to check if there is a steam process running, otherwise umu will fail to launch the game.
            let sys = sysinfo::System::new_all();
            if sys.processes_by_name("steam".as_ref()).count() == 0 {
                Command::new("steam")
                    .spawn()
                    .expect("Failed to launch Steam");

                std::thread::sleep(std::time::Duration::from_secs(10)); // steam is eepy, let's wait
            }
        }

        let game_preset =
            DefaultGameInfo::find_by_id(game_id).ok_or("Couldn't find game preset")?;

        command.arg(&game_preset.executable_name);
        command.arg("used_mods.txt;");

        println!("Running command: {:?}", command);

        command.spawn().expect("Umu failed");

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
        self.command.arg(&self.runner_path)
    }
}
