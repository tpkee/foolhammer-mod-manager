use tauri::Emitter;
use ureq::Body;

pub mod custom_macro;
pub mod path;

#[derive(Debug, serde::Serialize)]
pub enum ErrorCode {
    NotFound = 404,
    InternalError = 500,
    Conflict = 409,
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

pub async fn download(
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
