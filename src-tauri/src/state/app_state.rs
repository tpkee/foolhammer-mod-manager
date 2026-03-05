use crate::launchers::GameManager;
use notify::{Event, RecursiveMode, Watcher};
use std::path::PathBuf;
use tauri::async_runtime::Mutex;

struct FolderWatcher {
    watcher: notify::RecommendedWatcher,
    paths: Vec<PathBuf>,
}

pub struct State {
    pub game_runner: Option<Box<dyn GameManager>>,
    folder_watcher: FolderWatcher,
}

pub type AppState<'a> = tauri::State<'a, Mutex<State>>;

impl State {
    pub fn set_watchers(&mut self, folders: &[PathBuf]) {
        let mut watcher_paths = self.folder_watcher.watcher.paths_mut();
        let mut new_paths: Vec<PathBuf> = Vec::with_capacity(folders.len());
        let old_paths = &self.folder_watcher.paths;

        for old_path in old_paths {
            if !folders.contains(old_path) || !old_path.exists() {
                let _ = watcher_paths.remove(old_path);
            }
        }

        for folder in folders {
            if !folder.exists() {
                continue;
            }

            if !old_paths.contains(folder) {
                watcher_paths
                    .add(folder, RecursiveMode::Recursive)
                    .expect("Failed to watch folder from State::set_watchers");
            }

            new_paths.push(folder.clone());
        }

        // TODO: this sucks, there should be a retry or smth like that for when it fails
        watcher_paths
            .commit()
            .is_ok()
            .then(|| println!("Watching folders: {:?}", new_paths));

        self.folder_watcher.paths = new_paths;
    }
}

impl Default for State {
    fn default() -> Self {
        let watcher = notify::recommended_watcher(watcher_sentry)
            .expect("Failed to create watcher for State default");

        Self {
            folder_watcher: FolderWatcher {
                watcher,
                paths: vec![],
            },
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
