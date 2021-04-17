//! Digimon Card Game 2020 deck codec CLI interface

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]

mod codec;

pub use crate::codec::decode;

/// Digimon Card Game 2020 deck codec
#[derive(structopt::StructOpt)]
struct Cli {
    #[structopt(long = "decode", conflicts_with = "deck")]
    deck_code_str: Option<String>,

    #[structopt(long = "encode", conflicts_with = "deck_code_str")]
    deck: Option<String>,
}

fn main() {
    let args = {
        use structopt::StructOpt as _;
        Cli::from_args()
    };

    if let Some(deck_code_str) = &args.deck_code_str {
        println!(
            "{}",
            ron::ser::to_string_pretty(
                &codec::decode(deck_code_str),
                ron::ser::PrettyConfig::default(),
            )
            .unwrap()
        );
    }

    if let Some(_deck) = &args.deck {
        unimplemented!("Encoding is not implemented.");
    }
}
