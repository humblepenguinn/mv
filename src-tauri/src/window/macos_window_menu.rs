pub(crate) use tauri::AppHandle;
use tauri::Runtime;
use tauri::menu::{
    AboutMetadata, HELP_SUBMENU_ID, Menu, MenuItemBuilder, PredefinedMenuItem, Submenu,
    WINDOW_SUBMENU_ID,
};

pub(crate) fn app_menu<R: Runtime>(app_handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let pkg_info = app_handle.package_info();
    let config = app_handle.config();

    let about_metadata = AboutMetadata {
        name: Some(pkg_info.name.clone()),
        version: Some(pkg_info.version.to_string()),
        copyright: config.bundle.copyright.clone(),
        authors: config.bundle.publisher.clone().map(|p| vec![p]),
        ..Default::default()
    };

    let window_menu = Submenu::with_id_and_items(
        app_handle,
        WINDOW_SUBMENU_ID,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app_handle, None)?,
            &PredefinedMenuItem::maximize(app_handle, None)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::close_window(app_handle, None)?,
        ],
    )?;

    let help_menu = Submenu::with_id_and_items(
        app_handle,
        HELP_SUBMENU_ID,
        "Help",
        true,
        &[
            &MenuItemBuilder::with_id("open_feedback".to_string(), "Give Feedback")
                .build(app_handle)?,
        ],
    )?;

    let menu = Menu::with_items(
        app_handle,
        &[
            &Submenu::with_items(
                app_handle,
                pkg_info.name.clone(),
                true,
                &[
                    &PredefinedMenuItem::about(app_handle, None, Some(about_metadata))?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::services(app_handle, None)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::hide(app_handle, None)?,
                    &PredefinedMenuItem::hide_others(app_handle, None)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    // NOTE: Replace the predefined quit item with a custom one because, for some
                    //  reason, ExitRequested events are not fired on cmd+Q. Perhaps this will be
                    //  fixed in the future?
                    //  https://github.com/tauri-apps/tauri/issues/9198
                    &MenuItemBuilder::with_id(
                        "hacked_quit".to_string(),
                        format!("Quit {}", app_handle.package_info().name),
                    )
                    .accelerator("CmdOrCtrl+q")
                    .build(app_handle)?,
                ],
            )?,
            &window_menu,
            &help_menu,
        ],
    )?;

    Ok(menu)
}
