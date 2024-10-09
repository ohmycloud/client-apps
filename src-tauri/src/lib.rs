mod commands;
mod utils;

use tauri::{App, AppHandle, Builder, Runtime, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent, Wry};
use tauri::menu::{CheckMenuItem, MenuItem};
use tauri::menu::SubmenuBuilder;
use tauri::utils::config::TrayIconConfig;
use tauri::Webview;
use tauri::webview::PageLoadPayload;
use tauri_plugin_log::{Target, TargetKind};
use tracing::info;
use commands::{greet, get_app_dir};
use crate::utils::log_dir;

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app!");

    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?
    }
    let mut builder = WebviewWindowBuilder::new(
        app,
        "mainm",
        WebviewUrl::default()
    );

    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .title("Hacker News")
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 600.0)
            .content_protected(true)
            .resizable(true);
    }

    let webview = builder.build()?;

    #[cfg(debug_assertions)]
    webview.open_devtools();

    Ok(())
}
pub fn app() -> anyhow::Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler);

    Ok(builder)
}

fn window_event_handler(window: &Window, event: &WindowEvent) {
    info!("Window event {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap()
        }
    }
}

fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page loaded on {:?}", webview.label());
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder { path: log_dir(), file_name: Some("app.log".to_string()) }),
            Target::new(TargetKind::Stdout),
        ]).level(tracing::log::LevelFilter::Info)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), tauri::Error> {
    // create submenus
    let file_menu = SubmenuBuilder::with_id(app, "file", "File")
        .item(&MenuItem::new(app, "Open", true, Some("CmdOrCtrl+O"))?)
        .item(&MenuItem::new(app, "Save", true, Some("CmdOrCtrl+S"))?)
        .item(&MenuItem::new(app, "Save As", true, Some("CmdOrCtrl+Shift+S"))?)
        .separator()
        .quit();

    let edit_menu = SubmenuBuilder::with_id(app, "edit", "Edit")
        .item(&MenuItem::new(app, "Process", true, Some("CmdOrCtrl+P"))?)
        .separator()
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .separator()
        .select_all()
        .item(&CheckMenuItem::new(app, "Check Me", true, true, None)?);

    let tray_menu = SubmenuBuilder::with_id(app, "tray", "Tray")
        .item(&MenuItem::new(app, "Open", true, None)?)
        .item(&MenuItem::new(app, "Hide", true, None)?)
        .separator()
        .quit();

    let icon_path = app.default_window_icon();
    let mut icon = TrayIconConfig::default();
    icon.id = Some("HackerNews".to_string());
    icon.tooltip = Some("Hacker News".to_string());

    Ok(())

    // create menu, add menu to tray, add menu to window

}