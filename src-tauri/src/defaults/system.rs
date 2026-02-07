use lazy_static::lazy_static;
use std::path::PathBuf;
lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref PROGRAM_FILES_PATH: PathBuf = match std::env::consts::OS {
        "windows" => std::env::var("FOLDERID_ProgramFiles")
            .expect("Failed to get ProgramFiles environment variable")
            .into(),
        _ => dirs::data_dir().expect("Failed to get data directory"),
    };
}
