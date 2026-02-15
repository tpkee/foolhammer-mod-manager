pub mod custom_macro;
pub mod game_launcher;
pub mod path;

#[cfg(target_os = "linux")]
pub mod umu_manager;

#[derive(Debug, serde::Serialize)]
pub enum ErrorCode {
    NotFound = 404,
    InternalError = 500,
    Conflict = 409,
}
