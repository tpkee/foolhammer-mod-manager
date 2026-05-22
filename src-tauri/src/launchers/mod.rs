use crate::supported_games::SupportedGames;
use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub trait GameManager: Send {
    fn launch_game(
        &mut self,
        game_id: SupportedGames,
        game_path: &Path,
        save_path: Option<&PathBuf>, // the absolute path conteining the savegame file too
    ) -> Result<(), Box<dyn Error>>;
    fn kill_game(&mut self) -> Result<(), Box<dyn Error>>;
    fn get_command(&mut self) -> &mut Command;
}

#[cfg(target_os = "linux")]
pub(crate) type GameLauncher = linux::LinuxLauncher;
#[cfg(target_os = "windows")]
pub(crate) type GameLauncher = windows::WindowsLauncher;

impl GameLauncher {
    pub(crate) async fn create(app_handle: &tauri::AppHandle) -> GameLauncher {
        log::info!("Initializing {} game launcher", std::env::consts::OS);
        GameLauncher::new(app_handle).await
    }
}
