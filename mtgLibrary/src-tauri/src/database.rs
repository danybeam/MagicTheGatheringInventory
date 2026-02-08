use rusqlite::{Connection, Result};
use std::fs;
use tauri::{AppHandle, Manager};

const DB_VERSION: u32 = 1;

#[tauri::command]
#[specta::specta]
pub fn init_db(app_handle: AppHandle) -> String {
    let db = get_db(app_handle).expect("[Get database] database should be accessible");
    println!("Database fetched");
    upgrade_database_if_needed(db).expect("[Upgrade database] Database should be upgraded");
    format!("Success!")
}

fn get_db(app_handle: AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("MTGLibrary.sqlite");
    let path_string = sqlite_path.to_string_lossy().to_string();
    println!("App dir: {}", path_string);

    let db = Connection::open(sqlite_path)?;

    Ok(db)
}

fn upgrade_database_if_needed(mut db: Connection) -> Result<(), rusqlite::Error> {
    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    if existing_version >= DB_VERSION {
        println!("Upgrade not needed");
        return Ok(());
    }

    println!("entering upgrade mode");

    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", DB_VERSION)?;

    tx.execute_batch(
        r#"
            CREATE TABLE IF NOT EXISTS "Cards"(
              "id" INTEGER PRIMARY KEY,
              "name" TEXT  
            );

            CREATE TABLE IF NOT EXISTS "Decks"(
                "id" INTEGER PRIMARY KEY,
                "name" TEXT
            );

            CREATE TABLE IF NOT EXISTS "DeckComposition"(
                "id" INTEGER PRIMARY KEY,
                "card_id" INTEGER NOT NULL,
                "deck_id" INTEGER NOT NULL,
                "card_copies" INTEGER,
                FOREIGN KEY ("card_id") REFERENCES "Cards"("id"),
                FOREIGN KEY ("deck_id") REFERENCES "Decks"("id")
            );
        "#,
    )?;

    tx.commit()?;

    Ok(())
}

pub mod cards;
