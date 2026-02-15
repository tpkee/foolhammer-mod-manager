use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{error::Error, io::Read};
use tauri::{Emitter, Manager};
use ureq::Body;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UmuRelease {
    release_date: String,
    zipapp_url: String,
}

async fn get_umu_asset() -> Result<UmuRelease, Box<dyn Error>> {
    let res =
        ureq::get("https://api.github.com/repos/Open-Wine-Components/umu-launcher/releases/latest")
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

    Ok(UmuRelease {
        release_date: release_date.to_string(),
        zipapp_url: zipapp_url.to_string(),
    })
}

enum DownloadEvent {
    Error,
    Success,
    Start,
}

impl DownloadEvent {
    fn as_str(&self) -> &'static str {
        match self {
            DownloadEvent::Error => "error",
            DownloadEvent::Success => "success",
            DownloadEvent::Start => "start",
        }
    }
}

async fn download(
    app_handle: &tauri::AppHandle,
    url: &str,
    name: &str,
) -> Result<ureq::http::Response<Body>, ureq::Error> {
    let emit = |e: DownloadEvent| {
        let _ = &app_handle
            .emit(&format!("download/{}", name), e.as_str())
            .expect("Failed to emit download event");
    };

    emit(DownloadEvent::Start);

    let config = ureq::Agent::config_builder()
        .timeout_connect(Some(std::time::Duration::from_secs(10)))
        .build();

    let agent = ureq::Agent::new_with_config(config);

    let res = agent.get(url).call();

    if let Err(e) = res {
        eprintln!("Download error: {:?}", e);
        emit(DownloadEvent::Error);
        return Err(e);
    }

    emit(DownloadEvent::Success);

    Ok(res?)
}

pub enum UmuAvailability {
    Local,
    Global,
    NotAvailable,
}

pub fn generate_umu_command(
    app_handle: &tauri::AppHandle,
) -> Result<Command, Box<dyn std::error::Error>> {
    let err = Err("umu-launcher is not available in the system".into());
    match is_umu_available(app_handle) {
        UmuAvailability::Global => Ok(Command::new("umu-run")),
        UmuAvailability::Local => {
            if let Ok(app_data) = app_handle.path().app_data_dir() {
                let umu_path = app_data.join("umu-launcher/umu/umu-run");

                if umu_path.exists() {
                    let mut cmd = Command::new("python");
                    cmd.arg(umu_path.to_string_lossy().to_string());

                    return Ok(cmd);
                }
            }

            return err;
        }
        UmuAvailability::NotAvailable => return err,
    }
}

/// Check if umu-launcher is available in the system
pub fn is_umu_available(app_handle: &tauri::AppHandle) -> UmuAvailability {
    // Check if in global PATH
    // if Command::new("which").arg("umu-run").spawn().is_ok() {
    //     return UmuAvailability::Global;
    // }

    // Check if in local app data
    if let Ok(app_data) = app_handle.path().app_data_dir() {
        if app_data.join("umu-launcher/umu/umu-run").exists() {
            return UmuAvailability::Local;
        }
    }

    UmuAvailability::NotAvailable
}

pub async fn check_and_update(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn Error>> {
    let umu_dir = &app_handle.path().app_data_dir()?.join("umu-launcher/");

    let umu_meta = get_umu_asset().await.or_else(|e| Err(e))?;

    let mut version_file = File::open(umu_dir.join("version"))?;

    let mut current_version = String::new();

    version_file.read_to_string(&mut current_version)?;

    let current_version_date =
        chrono::DateTime::parse_from_rfc3339(&current_version)?.with_timezone(&chrono::Utc);
    let umu_release_date =
        chrono::DateTime::parse_from_rfc3339(&umu_meta.release_date)?.with_timezone(&chrono::Utc);

    if umu_release_date > current_version_date {
        println!("A new version of umu-launcher is available. Updating...");
        download_and_install(app_handle, umu_meta, umu_dir).await?;
    } else {
        println!("umu-launcher is up to date.");
    }

    Ok(())
}

pub async fn install_umu_launcher(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn Error>> {
    let umu_dir = &app_handle.path().app_data_dir()?.join("umu-launcher/");

    let umu_meta = get_umu_asset().await.or_else(|e| Err(e))?;

    download_and_install(app_handle, umu_meta, umu_dir).await?;

    Ok(())
}

async fn download_and_install(
    app_handle: &tauri::AppHandle,
    umu_meta: UmuRelease,
    umu_dir: &std::path::PathBuf,
) -> Result<(), Box<dyn Error>> {
    println!("Downloading Latest umu-launcher version: {:?}", umu_meta);

    let mut res = download(&app_handle, &umu_meta.zipapp_url, "umu-launcher").await?;

    let mut tar = tar::Archive::new(res.body_mut().as_reader());

    println!("Extracting umu-launcher to {:?}", umu_dir);

    tar.unpack(umu_dir)?;

    // write the release date to a version file
    let mut version_file = File::create(umu_dir.join("version"))?;
    version_file.write_all(umu_meta.release_date.as_bytes())?;

    Ok(())
}
