use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

pub struct FolderWatcher {
    watcher: RecommendedWatcher,
    paths: Vec<PathBuf>,
}

impl FolderWatcher {
    pub fn new(watcher: RecommendedWatcher) -> Self {
        Self {
            watcher,
            paths: vec![],
        }
    }

    pub fn set_watchers(&mut self, folders: &[PathBuf]) {
        let mut watcher_paths = self.watcher.paths_mut();
        let mut new_paths: Vec<PathBuf> = Vec::with_capacity(folders.len());
        let old_paths = &self.paths;

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

        self.paths = new_paths;
    }
}
