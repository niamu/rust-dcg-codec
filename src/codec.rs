//! Codec constants and structs

pub mod decode;

pub use crate::codec::decode::decode;

/// Version of the codec
pub const VERSION: u8 = 0;

/// Deck cods are all prefixed with "DCG"
pub const PREFIX: &str = "DCG";

/// A deck's digi-eggs and main deck is made of Card structs
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Card {
    number: String,
    parallel_id: u8,
    count: u8,
}

/// A deck has digi-egg cards (0-5 Cards), a main deck of cards (50 Cards), and a name (0-63 bytes)
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Deck {
    digi_eggs: Vec<Card>,
    deck: Vec<Card>,
    name: String,
}
