pub mod decode;

pub use crate::codec::decode::decode;

pub const VERSION: u8 = 0;
pub const PREFIX: &str = "DCG";

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Card {
    number: String,
    parallel_id: u8,
    count: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Deck {
    digi_eggs: Vec<Card>,
    deck: Vec<Card>,
    name: String,
}
