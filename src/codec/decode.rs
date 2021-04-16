use bytes::Buf;

pub use crate::codec::{Card, Deck, PREFIX, VERSION};

fn decode_b64(deck_code: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(deck_code, base64::URL_SAFE_NO_PAD)
}

fn compute_checksum(total_card_bytes: usize, deck_bytes: &[u8]) -> u32 {
    deck_bytes[..total_card_bytes]
        .iter()
        .map(|&b| b as u32)
        .sum::<u32>()
}

fn get_card_set_from_u32_bytes(card_set_bytes: &[u8]) -> &str {
    &std::str::from_utf8(&card_set_bytes).unwrap().trim()
}

fn is_carry_bit(current_byte: &u8, bits_to_shift: u8) -> bool {
    0 != current_byte & (1 << bits_to_shift)
}

fn read_bits_from_byte(current_byte: &u8, bits_to_shift: u8, delta_shift: u8, out_bits: u8) -> u8 {
    ((current_byte & ((1 << bits_to_shift) - 1)) << delta_shift) | out_bits
}

fn parse_deck(mut deck_bytes: &[u8]) -> Deck {
    let version_and_digi_egg_count = deck_bytes.get_u8();
    let version = version_and_digi_egg_count >> 4;
    assert_eq!(version, VERSION);

    let digi_egg_set_count = (version_and_digi_egg_count & 0x0F) as usize;
    let checksum = deck_bytes.get_u8();
    let deck_name_length = deck_bytes.get_u8() as usize;
    let total_card_bytes = deck_bytes.len() - deck_name_length;

    let computed_checksum = compute_checksum(total_card_bytes, deck_bytes);
    assert_eq!(checksum, computed_checksum as u8);

    let mut cards: Vec<Card> = Vec::new();

    while deck_bytes.remaining() > deck_name_length {
        let mut prev_card_number = 0;
        // Card Set Header
        // - Card Set
        let card_set_bytes = &deck_bytes.get_u32().to_be_bytes();
        let card_set = get_card_set_from_u32_bytes(card_set_bytes);
        // - Card Set Zero Padding and Count
        let padding_and_set_count = &deck_bytes.get_u8();
        let card_set_padding = ((padding_and_set_count >> 6) + 1) as usize;
        let card_set_count = padding_and_set_count & 0x3F;

        for _ in 0..card_set_count {
            let card_header = &deck_bytes.get_u8();
            let card_count = (card_header >> 6) + 1;
            let card_parallel_id = card_header >> 3 & 0x07;
            let card_number_offset = read_bits_from_byte(&card_header, 3 - 1, 0, 0);
            let mut card_number = card_number_offset;

            if is_carry_bit(&card_header, 3 - 1) {
                card_number =
                    read_bits_from_byte(&deck_bytes.get_u8(), 8 - 1, 3 - 1, card_number_offset);
            }
            prev_card_number = card_number + prev_card_number;

            let card = Card {
                number: format!(
                    "{s}-{:0>p$}",
                    prev_card_number,
                    s = card_set,
                    p = card_set_padding
                ),
                parallel_id: card_parallel_id,
                count: card_count,
            };
            cards.push(card);
        }
    }

    let deck_name: &str = std::str::from_utf8(&deck_bytes).unwrap();
    return Deck {
        digi_eggs: (&cards[..digi_egg_set_count]).to_vec(),
        deck: (&cards[digi_egg_set_count..]).to_vec(),
        name: deck_name.to_string(),
    };
}

pub fn decode(deck_code_str: &String) -> Deck {
    let (prefix, deck_code) = deck_code_str.split_at(3);
    assert_eq!(PREFIX, prefix);

    let deck_bytes: &[u8] = &decode_b64(deck_code).unwrap();
    return parse_deck(deck_bytes);
}
