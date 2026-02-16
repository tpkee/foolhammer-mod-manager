use std::{error::Error, path::PathBuf, process::Command};

#[cfg(target_os = "linux")]
pub mod linux;

pub trait GameManager: Send {
    fn launch_game(
        &mut self,
        game_id: &str,
        game_path: &PathBuf,
        save_path: Option<&PathBuf>, // the absolute path conteining the savegame file too
    ) -> Result<(), Box<dyn Error>>;
    fn kill_game(&mut self) -> Result<(), Box<dyn Error>>;
    fn get_command(&mut self) -> &mut Command;
}
