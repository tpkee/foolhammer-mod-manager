use crate::{events::AppEvent, launchers::GameManager, utils::folder_watcher};
use notify::Event;
use tauri::{AppHandle, Emitter, async_runtime::Mutex};

pub struct State {
    pub game_runner: Option<Box<dyn GameManager>>,
    pub folder_watcher: folder_watcher::FolderWatcher,
}

pub type AppState<'a> = tauri::State<'a, Mutex<State>>;

impl State {
    pub fn new(app_handle: AppHandle) -> Self {
        let watcher = notify::recommended_watcher(move |event| watcher_sentry(event, &app_handle))
            .expect("Failed to create watcher for State");

        log::info!("Folder watcher created successfully");

        Self {
            folder_watcher: folder_watcher::FolderWatcher::new(watcher),
            game_runner: None,
        }
    }
}

fn watcher_sentry(event: Result<Event, notify::Error>, app_handle: &AppHandle) {
    match event {
        Ok(e) => match e.kind {
            notify::EventKind::Create(_) | notify::EventKind::Remove(_) => {
                folders_governor(e, app_handle);
            }
            other => {
                log::debug!("Ignoring folder event kind: {:?}", other);
            }
        },
        Err(e) => {
            log::error!("Error watching folder: {:?}", e);
        }
    }
}

fn folders_governor(event: Event, app_handle: &AppHandle) {
    log::info!(
        "Folder event detected (kind={:?}, paths={:?})",
        event.kind,
        event.paths
    );
    app_handle
        .emit(AppEvent::RefreshGame.into(), ())
        .expect("It wasn't possible to emit the event");
}
