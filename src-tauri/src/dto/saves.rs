use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveResponseDto {
    pub name: String,
    pub path: PathBuf,
    pub last_updated: Option<String>,
    pub last_accessed: Option<String>,
}

impl SaveResponseDto {
    pub fn new(path: PathBuf) -> Self {
        let meta = std::fs::metadata(&path).expect("Failed to retrieve metadata for save file. This should not happen as the file was just read from the directory.");
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let last_updated = meta
            .modified()
            .ok()
            .map(|t| (chrono::DateTime::<chrono::Utc>::from(t)).to_rfc3339());
        let last_accessed = meta
            .accessed()
            .ok()
            .map(|t| (chrono::DateTime::<chrono::Utc>::from(t)).to_rfc3339());

        Self {
            name,
            path,
            last_updated,
            last_accessed,
        }
    }
}
