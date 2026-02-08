use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Card {
    pub id: Option<u32>,
    pub name: String,
    pub decks: Vec<String>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct CardQuery {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub potential_decks: Vec<Deck>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Deck {
    pub id: Option<u32>,
    pub name: String,
    pub cards: Vec<String>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct DeckQuery {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub potential_cards: Vec<Card>,
}
