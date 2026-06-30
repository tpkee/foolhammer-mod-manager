use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use steamworks::{Client, PublishedFileId};

use crate::supported_games::SupportedGames;

// Steam only allows a single client per process, and its callbacks have to be
// pumped from a long-lived thread. We cache the client here and spawn that pump
// once, on first successful init.
static STEAM_CLIENT: OnceLock<Mutex<Option<Client>>> = OnceLock::new();

// Steam returns at most 50 results per UGC query page, so batch the ids.
const QUERY_PAGE_SIZE: usize = 50;

fn client_cell() -> &'static Mutex<Option<Client>> {
    STEAM_CLIENT.get_or_init(|| Mutex::new(None))
}

/// Lazily initialise the Steam client for the given game, returning a clone on
/// success. Returns `None` if Steam isn't running or the app isn't owned; the
/// caller is expected to degrade gracefully in that case.
fn get_or_init_client(game_id: SupportedGames) -> Option<Client> {
    let mut guard = client_cell().lock().ok()?;

    if let Some(client) = guard.as_ref() {
        return Some(client.clone());
    }

    let app_id: u32 = game_id.into();
    match Client::init_app(app_id) {
        Ok(client) => {
            log::info!("Steam client initialised for app {}", app_id);

            // Pump callbacks for the lifetime of the process. The client is
            // Clone + Send + Sync, so the thread keeps its own handle.
            let pump = client.clone();
            std::thread::spawn(move || {
                loop {
                    pump.run_callbacks();
                    std::thread::sleep(Duration::from_millis(100));
                }
            });

            *guard = Some(client.clone());
            Some(client)
        }
        Err(e) => {
            log::warn!(
                "Failed to initialise Steam client for app {}: {:?}",
                app_id,
                e
            );
            None
        }
    }
}

/// Fetch Steam Workshop titles for the given published file ids, returned as a
/// `published_file_id -> title` map. Best-effort: returns an empty map (or a
/// partial one) if the Steam client is unavailable or a query fails/times out.
pub fn fetch_workshop_titles(game_id: SupportedGames, ids: Vec<u64>) -> HashMap<u64, String> {
    let mut titles = HashMap::new();

    if ids.is_empty() {
        return titles;
    }

    let Some(client) = get_or_init_client(game_id) else {
        return titles;
    };

    for chunk in ids.chunks(QUERY_PAGE_SIZE) {
        let file_ids: Vec<PublishedFileId> = chunk.iter().map(|id| PublishedFileId(*id)).collect();

        let query = match client.ugc().query_items(file_ids) {
            Ok(query) => query,
            Err(e) => {
                log::warn!("Failed to create workshop query: {:?}", e);
                continue;
            }
        };

        let (tx, rx) = std::sync::mpsc::channel();

        query.fetch(move |result| {
            let mut batch = HashMap::new();
            match result {
                Ok(results) => {
                    for i in 0..results.returned_results() {
                        if let Some(item) = results.get(i) {
                            batch.insert(item.published_file_id.0, item.title);
                        }
                    }
                }
                Err(e) => log::warn!("Workshop query failed: {:?}", e),
            }
            let _ = tx.send(batch);
        });

        match rx.recv_timeout(Duration::from_secs(15)) {
            Ok(batch) => titles.extend(batch),
            Err(e) => log::warn!("Timed out waiting for workshop titles: {:?}", e),
        }
    }

    log::info!("Fetched {} workshop title(s) from Steam", titles.len());
    titles
}
