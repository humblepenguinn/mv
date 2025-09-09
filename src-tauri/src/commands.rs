use font_kit::source::SystemSource;
use log::{info, warn};
use tauri::{AppHandle, Manager, WebviewWindow, command, is_dev};
use tokio::sync::Mutex;
use webbrowser;

use mv_core::analyzer::Analyzer;
use mv_core::error::Error::{AnalyzerError, ParserError};
use mv_core::parser::Parser;

use crate::AppState;
use crate::desktop_analyzer_state::DesktopAnalyzerState;
use crate::error::{Error, Result as MVResult};
use crate::updates::MVUpdater;
use crate::utils::remove_main_function;

#[derive(serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct AppMetaData {
    is_dev: bool,
    os: String,
    version: String,
    name: String,
    app_data_dir: String,
    app_log_dir: String,
}

#[command]
pub(crate) async fn cmd_metadata(app_handle: AppHandle) -> MVResult<AppMetaData> {
    let app_data_dir = app_handle.path().app_data_dir()?;
    let app_log_dir = app_handle.path().app_log_dir()?;
    Ok(AppMetaData {
        is_dev: is_dev(),
        version: app_handle.package_info().version.to_string(),
        os: tauri_plugin_os::platform().to_string(),
        name: app_handle.package_info().name.to_string(),
        app_data_dir: app_data_dir.to_string_lossy().to_string(),
        app_log_dir: app_log_dir.to_string_lossy().to_string(),
    })
}

#[command]
pub async fn cmd_download_and_install_update(window: WebviewWindow) -> MVResult<bool> {
    let state = window.app_handle().state::<Mutex<MVUpdater>>();
    let mut updater = state.lock().await;

    Ok(updater.download_and_install_now(&window).await?)
}

#[command]
pub async fn cmd_check_for_updates(window: WebviewWindow) -> MVResult<bool> {
    let state = window.app_handle().state::<Mutex<MVUpdater>>();
    let mut updater = state.lock().await;
    let result = updater.check_now(&window).await?;

    Ok(result)
}

#[command]
pub(crate) async fn cmd_analyze_source_code(
    app_handle: AppHandle,
    input: String,
) -> serde_json::Value {
    let sanitized_source_code = remove_main_function(&input);

    let mut parser = Parser::new(&sanitized_source_code);

    match parser.parse() {
        Ok(statements) => {
            info!("{:?}", statements);

            let mut state = DesktopAnalyzerState {
                state: &app_handle.state::<Mutex<AppState>>(),
            };

            match Analyzer::default().analyze_statements(statements, &mut state).await {
                Ok((stack, heap)) => {
                    return serde_json::json!({
                        "stack": stack,
                        "heap": heap,
                    });
                }

                Err(e) => match e {
                    AnalyzerError(_, line_number, column_number) => {
                        return serde_json::json!({
                            "error": {
                                "message": e.to_string(),
                                "line_number": line_number,
                                "column_number": column_number
                            }
                        });
                    }

                    _ => {
                        return serde_json::json!({
                            "error": {
                                "message": e.to_string()
                            }
                        });
                    }
                },
            }
        }

        Err(e) => match e {
            ParserError(_, line_number, column_number) => {
                return serde_json::json!({
                    "error": {
                        "message": e.to_string(),
                        "line_number": line_number,
                        "column_number": column_number
                    }
                });
            }

            _ => {
                return serde_json::json!({
                    "error": {
                        "message": e.to_string()
                    }
                });
            }
        },
    }
}

#[command]
pub(crate) async fn cmd_get_system_fonts() -> MVResult<Vec<String>> {
    let mut fonts = Vec::<String>::new();

    let source = SystemSource::new();

    let font_matches = source.all_fonts().map_err(|e| Error::Msg(e.to_string()))?;

    for handle in font_matches {
        match handle.load() {
            Ok(font) => fonts.push(font.full_name()),
            Err(e) => {
                warn!("Failed to load font: {}", e);
                continue;
            }
        }
    }

    fonts.sort();
    fonts.dedup();

    Ok(fonts)
}

#[command]
pub(crate) async fn cmd_open_url(url: String) -> MVResult<()> {
    webbrowser::open(&url)?;
    Ok(())
}
