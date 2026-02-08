use rusqlite::named_params;
use rusqlite::Connection;
use specta::specta;

use tauri::AppHandle;

use crate::{data_types::Card, data_types::CardQuery, database::get_db};

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

#[tauri::command]
#[specta]
pub fn get_cards(app_handle: AppHandle, input: CardQuery) -> Vec<Card> {
    // Declare control variables
    let mut result: Vec<Card> = vec![];
    let mut valid_query: bool = false;

    // Start query
    let db = get_db(app_handle).expect("Could not fetch database");
    let mut query: String = String::from("SELECT Cards.id, Cards.name, Decks.name FROM Cards LEFT JOIN DeckComposition ON Cards.id=DeckComposition.card_id LEFT JOIN Decks ON DeckComposition.deck_id=Decks.id WHERE");

    // Add parameters to search
    match input.id {
        Some(i) => {
            valid_query = true;
            query.push_str(" Cards.id=");
            query.push_str(&i.to_string());
        }
        _ => println!("Request had no card id"),
    };

    match input.name {
        Some(name) => {
            if valid_query {
                query.push_str(" OR");
            }
            valid_query = true;
            query.push_str(" Cards.name='");
            query.push_str(&name);
            query.push_str("'");
        }
        _ => println!("Request had no card name"),
    }

    for deck in input.potential_decks {
        if valid_query {
            query.push_str(" OR");
        }
        valid_query = true;
        let mut needs_or = false;
        if deck.id.is_some() {
            needs_or = true;
            query.push_str(" Decks.id=");
            query.push_str(&deck.id.unwrap().to_string());
        }

        let trimmed_name = deck.name.trim();

        if !trimmed_name.is_empty() {
            if needs_or {
                query.push_str(" OR");
            }

            query.push_str(" Decks.name='");
            query.push_str(trimmed_name);
            query.push_str("'");
        }
    }

    // If the query is not valid panic
    if !valid_query {
        panic!("Invalid query, not enough data to search");
    }

    query.push_str(" ORDER BY Cards.id;");

    // println!("{}", query);

    let mut stmt = db.prepare(&query).expect("Could not prepare query");
    let mut rows = stmt.query([]).expect("Could not extract rows");

    let mut current_card: Card = Card {
        id: None,
        name: String::from(""),
        decks: Vec::new(),
    };

    while let Some(row) = rows.next().expect("Could not retrieve next row") {
        let current_id: u32 = row.get_unwrap(0);
        if current_card.id.is_none() || current_card.id.unwrap() != current_id {
            if current_card.id.is_some() {
                result.push(current_card);
            }

            current_card = Card {
                id: Some(row.get_unwrap(0)),
                name: row.get_unwrap(1),
                decks: Vec::new(),
            }
        }
        match row.get(2) {
            Ok(value) => current_card.decks.push(value),
            Err(_) => {}
        };
    }

    result.push(current_card);

    result
}
