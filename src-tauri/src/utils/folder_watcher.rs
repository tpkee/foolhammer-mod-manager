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

        log::info!(
            "Updating folder watchers ({} requested folder(s))",
            folders.len()
        );

        for old_path in old_paths {
            if !folders.contains(old_path) || !old_path.exists() {
                log::info!("Removing watch: {}", old_path.display());
                let _ = watcher_paths.remove(old_path);
            }
        }

        for folder in folders {
            if !folder.exists() {
                log::debug!("Skipping non-existent folder: {}", folder.display());
                continue;
            }

            if !old_paths.contains(folder) {
                log::info!("Adding watch: {}", folder.display());
                if let Err(e) = watcher_paths.add(folder, RecursiveMode::NonRecursive) {
                    log::error!(
                        "Failed to watch folder {}: {:?}",
                        folder.display(),
                        e
                    );
                }
            }

            new_paths.push(folder.clone());
        }

        match watcher_paths.commit() {
            Ok(()) => log::info!("Watching folders: {:?}", new_paths),
            Err(e) => log::warn!("Failed to commit folder watcher changes: {:?}", e),
        }

        self.paths = new_paths;
    }
}
