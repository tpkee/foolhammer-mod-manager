use crate::supported_games::SupportedGames;
use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(target_os = "linux")]
mod linux;

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

impl GameLauncher {
    pub(crate) async fn create(app_handle: &tauri::AppHandle) -> GameLauncher {
        // TODO:  how will we handle the specific wine support? Maybe pass a flag or idk
        GameLauncher::new(app_handle).await
    }
}
