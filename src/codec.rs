//! Codec constants and structs
pub mod decode;

pub use crate::codec::decode::decode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Version of the codec
pub const VERSION: u8 = 2;

/// Deck cods are all prefixed with "DCG"
pub const PREFIX: &str = "DCG";

fn is_zero(n: &u8) -> bool {
    return *n == 0;
}

/// A deck's digi-eggs and main deck is made of Card structs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    /// card number identifier (i.e. "ST1-001")
    pub number: String,
    /// parallel-id values greater than 0 are alternate arts of the card
    #[serde(
        skip_serializing_if = "is_zero",
        rename(serialize = "parallel-id")
    )]
    pub parallel_id: u8,
    /// count of how many cards are in this card group
    pub count: u8,
}

/// Deck language
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Language {
    /// Japanese
    JA,
    /// English
    EN,
}

/// A deck has digi-egg cards (0-5 Cards), a main deck of cards (50 Cards), and a name (0-63 bytes)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Deck {
    #[serde(rename(serialize = "digi-eggs"))]
    /// cards in digi-egg deck
    pub digi_eggs: Vec<Card>,
    /// cards in main deck
    pub deck: Vec<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// cards in sideboard
    pub sideboard: Option<Vec<Card>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// deck language
    pub language: Option<Language>,
    /// deck name
    pub name: String,
}

/// Convert a u8 to a base36 character
pub fn base36_to_char(base36: u8) -> &'static str {
    let lookup: HashMap<u8, &str> = HashMap::from([
        (0, "0"),
        (1, "1"),
        (2, "2"),
        (3, "3"),
        (4, "4"),
        (5, "5"),
        (6, "6"),
        (7, "7"),
        (8, "8"),
        (9, "9"),
        (10, "A"),
        (11, "B"),
        (12, "C"),
        (13, "D"),
        (14, "E"),
        (15, "F"),
        (16, "G"),
        (17, "H"),
        (18, "I"),
        (19, "J"),
        (20, "K"),
        (21, "L"),
        (22, "M"),
        (23, "N"),
        (24, "O"),
        (25, "P"),
        (26, "Q"),
        (27, "R"),
        (28, "S"),
        (29, "T"),
        (30, "U"),
        (31, "V"),
        (32, "W"),
        (33, "X"),
        (34, "Y"),
        (35, "Z"),
    ]);
    lookup.get(&base36).unwrap_or(&"")
}
