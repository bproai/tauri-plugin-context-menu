use serde::Deserialize;
use tauri::{plugin::Builder, plugin::TauriPlugin, Runtime, Window};

mod keymap;
mod menu_item;
mod theme;

use menu_item::MenuItem;
use theme::Theme;

#[cfg(target_os = "windows")]
mod win_image_handler;

#[cfg(target_os = "windows")]
#[path = "win.rs"]
mod os;

#[cfg(target_os = "macos")]
mod macos_window_holder;

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod os;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod os;

#[derive(Clone, Deserialize)]
pub struct Position {
    x: f64,
    y: f64,
    is_absolute: Option<bool>,
}

use tauri::{plugin::Builder, plugin::TauriPlugin, Runtime, State};
use std::sync::Mutex;

#[tauri::command]
fn show_context_menu<R: Runtime>(
    window: Window<R>,
    pos: Option<Position>,
    items: Option<Vec<MenuItem>>,
    theme: Option<String>,
) {
    let theme = theme.and_then(|s| Theme::from_str(&s));

    // Use provided items, or fallback to default
    let menu_items = items.or_else(|| {
        DEFAULT_MENU_ITEMS.get().map(|mutex| mutex.lock().unwrap().clone())
    });

    os::show_context_menu(window, pos, menu_items, theme);
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("context-menu")
        .setup(|app, api| {
            // Any plugin setup (optional)
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![show_context_menu])
        .build()
}



// Global fallback to store default items (used when `items` arg is None)
static DEFAULT_MENU_ITEMS: OnceCell<Mutex<Vec<MenuItem>>> = OnceCell::new();

pub fn init_with_menu<R: Runtime>(default_items: Vec<MenuItem>) -> TauriPlugin<R> {
    DEFAULT_MENU_ITEMS.set(Mutex::new(default_items)).ok();

    Builder::new("context-menu")
        .setup(|_app, _api| Ok(()))
        .invoke_handler(tauri::generate_handler![show_context_menu])
        .build()
}

