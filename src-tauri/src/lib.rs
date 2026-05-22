use tauri::{Manager, async_runtime::Mutex};
use tauri_plugin_log::{Target, TargetKind};

pub mod commands;
pub mod defaults;
pub mod dto;
pub mod events;
pub mod launchers;
pub mod mods;
pub mod state;
pub mod stores;
pub mod supported_games;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("foolhammer.log".into()),
                    }),
                ])
                .level(log_level)
                .max_file_size(2_000_000)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            let app_handle = app.handle();

            log::info!("Foolhammer {} starting", app.package_info().version);
            log::info!("OS: {} {}", std::env::consts::OS, std::env::consts::ARCH);
            log::info!(
                "Steam directory: {:?}",
                defaults::system::STEAMDIR_INSTANCE
                    .as_ref()
                    .map(|d| d.path().display().to_string())
            );
            log::info!("Config dir: {:?}", app.path().config_dir());
            log::info!("Log dir: {:?}", app.path().app_log_dir());

            let default_state = state::State::new(app_handle.clone());

            stores::settings::SettingsStore::get_store(app_handle)
                .expect("Failed to build settings store");

            app.manage(Mutex::new(default_state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::games::start_game,
            commands::games::stop_game,
            commands::games::get_saves,
            commands::games::update_game,
            commands::games::check_path_exists,
            commands::games::get_supported_games,
            commands::games::get_game,
            commands::games::set_default_profile,
            commands::profiles::create_profile,
            commands::profiles::update_profile,
            commands::profiles::rename_profile,
            commands::profiles::delete_profile,
            commands::profiles::toggle_manual_mode,
            commands::profiles::set_profile_mods,
            commands::profiles::add_profile_mods,
            commands::profiles::remove_profile_mods,
            commands::settings::get_user_settings,
            commands::settings::set_default_game,
            commands::settings::update_settings,
            commands::settings::get_log_directory,
            commands::groups::create_group,
            commands::groups::update_group,
            commands::groups::rename_group,
            commands::groups::delete_group,
            commands::groups::set_group_mods,
            commands::groups::add_group_mods,
            commands::groups::remove_group_mods,
            commands::groups::add_group_profile,
            commands::groups::remove_group_profile,
            commands::groups::set_groups_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
