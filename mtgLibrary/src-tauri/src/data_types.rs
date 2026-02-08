use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Card {
    pub id: Option<u32>,
    pub name: String,
    pub decks: Vec<Deck>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Deck {
    pub id: Option<u32>,
    pub name: String,
    pub cards: Vec<Card>,
}
