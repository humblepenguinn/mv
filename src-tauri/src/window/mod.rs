#[cfg(target_os = "macos")]
mod macos_window;
#[cfg(target_os = "macos")]
mod macos_window_menu;

use log::info;
use rand::random;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindow, WindowEvent};
use tokio::sync::mpsc;

pub(crate) const MAIN_WINDOW_PREFIX: &str = "main_";
pub(crate) const OTHER_WINDOW_PREFIX: &str = "other_";

pub(crate) const DEFAULT_WINDOW_WIDTH: f64 = 1100.0;
pub(crate) const DEFAULT_WINDOW_HEIGHT: f64 = 600.0;

pub(crate) const MIN_WINDOW_WIDTH: f64 = 300.0;
pub(crate) const MIN_WINDOW_HEIGHT: f64 = 300.0;

#[derive(Default, Debug)]
pub(crate) struct CreateWindowConfig<'s> {
    pub(crate) url: &'s str,
    pub(crate) label: &'s str,
    pub(crate) title: &'s str,
    pub(crate) inner_size: Option<(f64, f64)>,
    pub(crate) position: Option<(f64, f64)>,
    pub(crate) navigation_tx: Option<mpsc::Sender<String>>,
    pub(crate) close_tx: Option<mpsc::Sender<()>>,
    pub(crate) hide_titlebar: bool,
}

pub(crate) fn create_window<R: Runtime>(
    handle: &AppHandle<R>,
    config: CreateWindowConfig,
) -> WebviewWindow<R> {
    #[cfg(target_os = "macos")]
    {
        use macos_window_menu::app_menu;

        let menu = app_menu(handle).unwrap();
        handle.set_menu(menu).expect("Failed to set app menu");
    }

    info!("Create new window label={}", config.label);

    let mut win_builder =
        tauri::WebviewWindowBuilder::new(handle, config.label, WebviewUrl::App(config.url.into()))
            .title(config.title)
            .resizable(true)
            .fullscreen(false)
            .disable_drag_drop_handler() // Required for frontend Dnd on windows
            .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);

    if let Some((w, h)) = config.inner_size {
        win_builder = win_builder.inner_size(w, h);
    } else {
        win_builder = win_builder.inner_size(600.0, 600.0);
    }

    if let Some((x, y)) = config.position {
        win_builder = win_builder.position(x, y);
    } else {
        win_builder = win_builder.center();
    }

    if let Some(tx) = config.navigation_tx {
        win_builder = win_builder.on_navigation(move |url| {
            let url = url.to_string();
            let tx = tx.clone();
            tauri::async_runtime::block_on(async move {
                tx.send(url).await.unwrap();
            });
            true
        });
    }

    if config.hide_titlebar {
        #[cfg(target_os = "macos")]
        {
            use tauri::TitleBarStyle;
            win_builder = win_builder.hidden_title(true).title_bar_style(TitleBarStyle::Overlay);
        }
        #[cfg(not(target_os = "macos"))]
        {
            win_builder = win_builder.decorations(false);
        }
    }

    if let Some(w) = handle.webview_windows().get(config.label) {
        info!("Webview with label {} already exists. Focusing existing", config.label);
        w.set_focus().unwrap();
        return w.to_owned();
    }

    let win = win_builder.build().unwrap();

    if let Some(tx) = config.close_tx {
        win.on_window_event(move |event| match event {
            WindowEvent::CloseRequested { .. } => {
                let tx = tx.clone();
                tauri::async_runtime::spawn(async move {
                    tx.send(()).await.unwrap();
                });
            }
            _ => {}
        });
    }

    #[cfg(target_os = "macos")]
    {
        use macos_window;
        macos_window::setup_traffic_light_positioner(&win);

        win.on_menu_event(move |w, event| {
            if !w.is_focused().unwrap() {
                return;
            }

            let event_id = event.id().0.as_str();
            match event_id {
                "hacked_quit" => {
                    // Cmd+Q on macOS doesn't trigger `CloseRequested` so we use a custom Quit menu
                    // and trigger close() for each window.
                    w.webview_windows().iter().for_each(|(_, w)| {
                        info!("Closing window {}", w.label());
                        let _ = w.close();
                    });
                }
                "close" => w.close().unwrap(),
                _ => {}
            }
        });
    }

    win
}

pub(crate) fn create_main_window(
    handle: &AppHandle,
    url: &str,
    size: Option<(f64, f64)>,
) -> WebviewWindow {
    let mut counter = 0;
    let label = loop {
        let label = format!("{MAIN_WINDOW_PREFIX}{counter}");
        match handle.webview_windows().get(label.as_str()) {
            None => break Some(label),
            Some(_) => counter += 1,
        }
    }
    .expect("Failed to generate label for new window");

    let config = CreateWindowConfig {
        url,
        label: label.as_str(),
        title: "MV",
        inner_size: size,
        position: Some((100.0 + random::<f64>() * 20.0, 100.0 + random::<f64>() * 20.0)),
        hide_titlebar: true,
        ..Default::default()
    };

    create_window(handle, config)
}
