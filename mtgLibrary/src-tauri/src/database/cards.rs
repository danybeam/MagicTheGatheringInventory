use rusqlite::named_params;
use rusqlite::Connection;
use specta::specta;

use tauri::AppHandle;

use crate::{data_types::Card, database::get_db};

#[tauri::command]
#[specta]
pub fn create_or_update_card(app_handle: AppHandle, input: Card) -> Card {
    let db = get_db(app_handle).expect("Failed to fetch database");
    match input.id {
        Some(_) => update_card(db, input),
        None => create_card(db, input),
    }
}

fn create_card(db: Connection, card: Card) -> Card {
    // Define query
    let mut query = db
        .prepare("INSERT INTO Cards(name) VALUES (?1)")
        .expect("Could not prepare statement");

    // Insert new ID back into card and return Card data
    match query.insert([card.name.clone()]) {
        Ok(id) => {
            let mut card2 = card;
            card2.id = u32::try_from(id).ok();
            card2
        }
        Err(_) => panic!("Card creation went wrong"),
    }
}

fn update_card(db: Connection, card: Card) -> Card {
    // Define query
    let query = "INSERT OR REPLACE INTO Cards(id,name) VALUES (:id, :name)";
    let mut stmt = db
        .prepare_cached(query)
        .expect("Failed to prepare statemet");

    // Execute query. Because this is update assume card already has source of truth
    stmt.execute(named_params! {":id":card.id,":name":card.name})
        .expect("Could not execute statement");

    // Return card back out
    card
}
