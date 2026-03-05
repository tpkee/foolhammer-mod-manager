use crate::{launchers::GameManager, utils::folder_watcher};
use notify::Event;
use tauri::async_runtime::Mutex;

pub struct State {
    pub game_runner: Option<Box<dyn GameManager>>,
    pub folder_watcher: folder_watcher::FolderWatcher,
}

pub type AppState<'a> = tauri::State<'a, Mutex<State>>;

impl State {
    pub fn new() -> Self {
        let watcher = notify::recommended_watcher(move |event| watcher_sentry(event))
            .expect("Failed to create watcher for State");

        Self {
            folder_watcher: folder_watcher::FolderWatcher::new(watcher),
            game_runner: None,
        }
    }
}

fn watcher_sentry(event: Result<Event, notify::Error>) {
    match event {
        Ok(e) => match e.kind {
            notify::EventKind::Create(_)
            | notify::EventKind::Modify(_)
            | notify::EventKind::Remove(_) => {
                folders_governor(e);
            }
            _ => {}
        },
        Err(e) => {
            eprintln!("Error watching folder: {:?}", e);
        }
    }
}

fn folders_governor(event: Event) {
    // TODO: should rescan and do things based on the folder path
    println!("Folder event detected: {:?}", event);
}
