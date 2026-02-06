use notify::{Event, RecursiveMode, Watcher};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{
    collections::HashMap,
    path::Path,
    sync::{mpsc, Mutex},
};

pub type AppState<'a> = tauri::State<'a, Mutex<State>>;

#[derive(serde::Serialize, Debug)]
pub struct FolderWatcher {
    pub path: String,
    #[serde(skip)]
    watcher: notify::RecommendedWatcher,
    #[serde(skip)]
    receiver_channel: mpsc::Receiver<notify::Result<Event>>,
}

impl FolderWatcher {
    pub fn new(path: &str) -> Self {
        let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

        let watcher = notify::recommended_watcher(tx).unwrap();

        FolderWatcher {
            path: path.to_string(),
            watcher: watcher,
            receiver_channel: rx,
        }
    }

    pub fn watch(&mut self, callback: fn(event: Event)) {
        self.watcher
            .watch(Path::new(&self.path), RecursiveMode::Recursive)
            .unwrap();

        for res in &self.receiver_channel {
            match res {
                Ok(event) => callback(event),
                Err(e) => println!("watch error: {:?}", e), // we should cleanup or dunno
            }
        }
    }

    pub fn unwatch(mut self) {
        self.watcher.unwatch(Path::new(&self.path)).unwrap();
    }

    pub fn check_path(&self, path: &str) -> bool {
        self.path == path
    }
}

#[derive(serde::Serialize, Debug)]
pub struct State {
    pub game_folder: Option<FolderWatcher>,
    pub game_workshop_folder: Option<FolderWatcher>, // steam's workshop
    pub game_id: String,
}

impl State {
    pub fn from_store(entries: Vec<(String, serde_json::Value)>) -> Self {
        let mut state = State {
            game_folder: None,
            game_workshop_folder: None,
            game_id: String::from("1142710"),
        };

        for (k, v) in entries {
            match k.as_str() {
                "game_folder" => {
                    if let Some(path) = v.as_str() {
                        state.game_folder = Some(initialize_watcher(path));
                    }
                }
                "game_workshop_folder" => {
                    if let Some(path) = v.as_str() {
                        state.game_workshop_folder = Some(initialize_watcher(path));
                    }
                }
                "game_id" => {
                    if let Some(id) = v.as_str() {
                        state.game_id = id.to_string();
                    }
                }
                _ => {}
            }
        }

        state
    }

    pub fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        serde_json::to_value(self)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
    }

    pub fn to_hashmap(&self) -> HashMap<String, serde_json::Value> {
        self.to_json()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

pub fn get_default_state() -> State {
    State {
        game_folder: None,
        game_workshop_folder: None,
        game_id: String::from("1142710"),
    }
}

fn initialize_watcher(path: &str) -> FolderWatcher {
    let mut folder_watcher = FolderWatcher::new(path);
    folder_watcher.watch(|event| {
        println!("Folder event: {:?}", event); // TODO: emit an event? idk
    });
    folder_watcher
}
