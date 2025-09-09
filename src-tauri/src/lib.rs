mod commands;
mod desktop_analyzer_state;
mod error;
mod updates;
mod utils;
mod window;

use indexmap::IndexMap;
use log::{error, info, warn};

use tauri::{App, Emitter, Manager, RunEvent, State, WindowEvent};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use tokio::sync::Mutex;

use crate::commands::{
    cmd_analyze_source_code, cmd_check_for_updates, cmd_download_and_install_update,
    cmd_get_system_fonts, cmd_metadata, cmd_open_url,
};
use crate::updates::MVUpdater;

#[derive(Default)]
pub(crate) struct AppState {
    pub starting_pointers: Mutex<Option<IndexMap<String, usize>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin({
            #[cfg(debug_assertions)]
            let log_level = log::LevelFilter::Info;
            #[cfg(not(debug_assertions))]
            let log_level = log::LevelFilter::Error;

            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("log".into()),
                }))
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(5))
                .level(log_level)
                .build()
        })
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .setup(|app: &mut App| {
            app.manage(Mutex::new(MVUpdater::new()));
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd_metadata,
            cmd_check_for_updates,
            cmd_download_and_install_update,
            cmd_analyze_source_code,
            cmd_get_system_fonts,
            cmd_open_url
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            match event {
                RunEvent::Ready => {
                    window::create_main_window(
                        &app_handle,
                        "/",
                        Some((window::DEFAULT_WINDOW_WIDTH, window::DEFAULT_WINDOW_HEIGHT)),
                    );
                }

                RunEvent::WindowEvent {
                    event: WindowEvent::Focused(true),
                    label,
                    ..
                } => {
                    if let Some(w) = app_handle.get_webview_window(&label) {
                        let h = app_handle.clone();
                        tauri::async_runtime::spawn(async move {
                            let val: State<'_, Mutex<MVUpdater>> = h.state();

                            if !val.lock().await.is_update_check_due() {
                                return;
                            }

                            let update_available = match val.lock().await.check_now(&w).await {
                                Ok(res) => res,
                                Err(e) => {
                                    error!("Error checking for updates: {}", e);
                                    return;
                                }
                            };

                            if let Err(e) = w.emit("update-available", update_available) {
                                error!("Failed to emit update-available event: {}", e);
                            }
                        });
                    } else {
                        error!("Webview window not found for label: {}", label);
                    }
                }

                RunEvent::WindowEvent {
                    event: WindowEvent::CloseRequested { .. },
                    label,
                    ..
                } => {
                    if !label.starts_with(window::OTHER_WINDOW_PREFIX)
                        && !(app_handle.webview_windows().len() > 1)
                    {
                        if let Err(e) = app_handle.save_window_state(StateFlags::all()) {
                            warn!("Failed to save window state {e:?}");
                        } else {
                            info!("Window state saved successfully");
                        };
                    } else {
                        info!("Skipping window state save for label: {}", label);
                    }
                }
                _ => {}
            };
        })
}
