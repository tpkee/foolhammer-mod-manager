use crate::launchers::GameManager;
use notify::{Event, RecursiveMode, Watcher};
use std::path::PathBuf;
use tauri::async_runtime::Mutex;

pub struct State {
    pub game_runner: Option<Box<dyn GameManager>>,
    watcher: notify::RecommendedWatcher,
}

pub type AppState<'a> = tauri::State<'a, Mutex<State>>;

impl State {
    pub fn set_watchers(&mut self, folders: &Vec<PathBuf>) {
        let mut watcher_paths = self.watcher.paths_mut();

        for folder in folders {
            if !folder.exists() {
                continue;
            }

            watcher_paths
                .add(folder, RecursiveMode::Recursive)
                .expect("Failed to watch folder from State::set_settings_from_store");
        }

        // TODO: this sucks, there should be a retry or smth like that for when it fails
        watcher_paths
            .commit()
            .is_ok()
            .then(|| println!("Watching folders: {:?}", folders));
    }
}

impl Default for State {
    fn default() -> Self {
        let watcher = notify::recommended_watcher(watcher_sentry)
            .expect("Failed to create watcher for State default");

        Self {
            watcher,
            game_runner: None,
        }
    }
}

fn watcher_sentry(event: Result<Event, notify::Error>) {
    // call the governor if the event was an edit
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
            eprintln!("Error watching folder: {:?}", e); // TODO: uhm, find what to do when this happens...? Stop the watcher or dunno
        }
    }
}

fn folders_governor(event: Event) {
    // TODO: should rescan and do things based on the folder path
    println!("Folder event detected: {:?}", event);
}
