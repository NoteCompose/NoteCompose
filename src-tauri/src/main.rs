#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{api::dialog::FileDialogBuilder, CustomMenuItem, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let open = CustomMenuItem::new("open".to_string(), "Open").accelerator("Cmd+O");
    let submenu = Submenu::new("File", Menu::new().add_item(open));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(move |event| match event.menu_item_id() {
            "open" => {
                FileDialogBuilder::default()
                    .add_filter("LilyPond", &["ly"])
                    .pick_file(move |path_buf| match path_buf {
                        Some(path) => event
                            .window()
                            .emit("openFile", Some(path.to_string_lossy().to_string())).unwrap(),
                        _ => {}
                    });
            }
            _ => todo!(),
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
