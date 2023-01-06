#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use staff::{
    render::staff::{
        measure::{Clef, Measure, MeasureItem, MeasureItemKind},
        renderer::Renderer,
    },
    time::{Duration, DurationKind},
};
use tauri::{api::dialog::FileDialogBuilder, CustomMenuItem, Menu, MenuItem, Submenu};

#[tauri::command]
fn items() -> String {
    let renderer = Renderer::default();
    let mut items = vec![MeasureItem::rest(
        Duration::new(DurationKind::Quarter, false),
        &renderer,
    )];
    let measure = Measure::new(items, &renderer);

    let render_items = measure.items(0., 0., 0., 0, &renderer);
    serde_json::to_string(&render_items).unwrap()
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
                            .emit("openFile", Some(path.to_string_lossy().to_string()))
                            .unwrap(),
                        _ => {}
                    });
            }
            _ => todo!(),
        })
        .invoke_handler(tauri::generate_handler![items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
