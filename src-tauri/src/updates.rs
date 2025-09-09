use log::{error, info};
use std::time::SystemTime;

use crate::error::Result as MVResult;
use tauri::{Emitter, Runtime, WebviewWindow};
use tauri_plugin_updater::UpdaterExt;

const MAX_UPDATE_CHECK_HOURS: u64 = 12;

pub(crate) struct MVUpdater {
    last_update_check: SystemTime,
}

#[derive(serde::Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub(crate) enum UpdateProgressEvent {
    Started,
    Downloading {
        progress: usize,
        total: Option<u64>,
        percentage: Option<u64>,
    },
    Installing,
    Completed,
    Failed(String),
}

impl MVUpdater {
    pub(crate) fn new() -> Self {
        Self {
            last_update_check: SystemTime::UNIX_EPOCH,
        }
    }

    pub(crate) fn is_update_check_due(&self) -> bool {
        if self.last_update_check == SystemTime::UNIX_EPOCH {
            // If this is the first check, we consider it due
            return true;
        }

        let update_period_seconds = MAX_UPDATE_CHECK_HOURS * (60 * 60);
        let seconds_since_last_check = self.last_update_check.elapsed().unwrap().as_secs();
        seconds_since_last_check >= update_period_seconds
    }

    pub(crate) async fn check_now<R: Runtime>(
        &mut self,
        window: &WebviewWindow<R>,
    ) -> MVResult<bool> {
        #[cfg(target_os = "linux")]
        {
            if std::env::var("APPIMAGE").is_err() {
                return Ok(false);
            }
        }

        self.last_update_check = SystemTime::now();

        let w = window.clone();
        let update_check_result = w.updater_builder().build()?.check().await;

        let result = match update_check_result? {
            None => false,
            Some(_) => true,
        };

        Ok(result)
    }

    pub(crate) async fn download_and_install_now<R: Runtime>(
        &mut self,
        window: &WebviewWindow<R>,
    ) -> MVResult<bool> {
        use std::sync::{Arc, Mutex};
        use tauri::Manager;
        use tauri_plugin_updater::UpdaterExt;

        self.last_update_check = SystemTime::now();

        let window = Arc::new(window.clone());

        let emit_event = |w: &Arc<WebviewWindow<R>>, event: UpdateProgressEvent, desc: &str| match w
            .emit("update-progress", event)
        {
            Ok(_) => info!("Successfully emitted {desc} event"),
            Err(e) => error!("Failed to emit {desc} event: {e}"),
        };

        let update = match window.updater_builder().build()?.check().await? {
            None => return Ok(false),
            Some(update) => update,
        };

        info!("Starting update download and install process...");
        emit_event(&window, UpdateProgressEvent::Started, "update-started");

        let last_emitted = Arc::new(Mutex::new((0usize, None::<u64>)));
        let total_downloaded = Arc::new(Mutex::new(0usize));

        let w_progress = Arc::clone(&window);
        let w_install = Arc::clone(&window);

        let result = update
            .download_and_install(
                {
                    let last_emitted = Arc::clone(&last_emitted);
                    let total_downloaded = Arc::clone(&total_downloaded);

                    move |progress, total| {
                        let mut downloaded = total_downloaded.lock().unwrap();
                        *downloaded += progress;

                        let percentage = total
                            .filter(|&t| t > 0)
                            .map(|t| ((*downloaded as f64 / t as f64) * 100.0).round() as u64);

                        let mut last = last_emitted.lock().unwrap();
                        if *downloaded > last.0 || percentage != last.1 {
                            *last = (*downloaded, percentage);

                            info!(
                                "Download progress: {} / {:?} ({:?}%)",
                                *downloaded, total, percentage
                            );

                            emit_event(
                                &w_progress,
                                UpdateProgressEvent::Downloading {
                                    progress: *downloaded,
                                    total,
                                    percentage,
                                },
                                "download-progress",
                            );
                        }
                    }
                },
                move || {
                    info!("Starting installation phase...");
                    emit_event(&w_install, UpdateProgressEvent::Installing, "installing");
                },
            )
            .await;

        match result {
            Ok(_) => {
                info!("Update completed successfully, restarting app...");
                emit_event(&window, UpdateProgressEvent::Completed, "update-completed");
                window.app_handle().restart();
            }
            Err(e) => {
                error!("Update failed: {e}");
                emit_event(&window, UpdateProgressEvent::Failed(e.to_string()), "update-failed");
                Err(e.into())
            }
        }
    }
}
