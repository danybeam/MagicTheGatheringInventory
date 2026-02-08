use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Card {
    pub id: u64,
    pub name: String,
    pub decks: Vec<Deck>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Deck {
    pub id: u64,
    pub name: String,
    pub cards: Vec<Card>,
}
