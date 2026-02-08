// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use specta_typescript::Typescript;

use tauri_specta::{collect_commands, Builder};

mod data_types;
mod database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = /*tauri_specta::*/Builder::<tauri::Wry>::new()
        .typ::<data_types::Card>()
        .typ::<data_types::Deck>()
        .typ::<data_types::CardQuery>()
        .typ::<data_types::DeckQuery>()
        .commands(collect_commands![
            database::init_db,
            database::cards::create_or_update_card,
            database::cards::get_cards,
         ]);

    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![
            database::init_db,
            database::cards::create_or_update_card,
            database::cards::get_cards,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
