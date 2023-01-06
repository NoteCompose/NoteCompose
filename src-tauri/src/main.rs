#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::File, io::Read};

use serde::Serialize;
use staff::{
    parse::Parser,
    render::{
        staff::{
            measure::{Clef, Measure, MeasureItem, MeasureItemKind},
            renderer::Renderer,
        },
        Item,
    },
    time::{Duration, DurationKind},
};
use tauri::{api::dialog::FileDialogBuilder, CustomMenuItem, Menu, MenuItem, Submenu};

#[derive(Serialize)]
struct Items {
    file_name: String,
    items: Vec<Item>,
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
                    .pick_file(move |path_buf| {
                        if let Some(path) = path_buf {
                            let mut contents = String::new();
                            let mut f = File::open(&path).unwrap();
                            f.read_to_string(&mut contents).unwrap();

                            let mut parser = Parser::from(contents.as_str());
                            let renderer = Renderer::default();
                            let staff = parser.staff(&renderer);

                            let render_items = staff.items(0., 0., &renderer);
                            let items = serde_json::to_string(&Items {
                                file_name: path.file_name().unwrap().to_string_lossy().to_string(),
                                items: render_items,
                            })
                            .unwrap();
                            event.window().emit("items", Some(items)).unwrap();
                        }
                    });
            }
            _ => todo!(),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
