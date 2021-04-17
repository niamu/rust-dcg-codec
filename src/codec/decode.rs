pub use crate::codec::{Card, Deck, PREFIX, VERSION};

fn decode_b64(deck_code: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(deck_code, base64::URL_SAFE_NO_PAD)
}

fn get_u8(deck_bytes: &mut Vec<u8>) -> u8 {
    *deck_bytes.drain(..1).collect::<Vec<u8>>().first().unwrap()
}

fn get_u32(deck_bytes: &mut Vec<u8>) -> Vec<u8> {
    (*deck_bytes.drain(..4).collect::<Vec<u8>>()).to_vec()
}

fn compute_checksum(total_card_bytes: usize, deck_bytes: &[u8]) -> u8 {
    deck_bytes[..total_card_bytes]
        .iter()
        .map(|&b| b as u32)
        .sum::<u32>() as u8
}

fn get_str_from_bytes(card_set_bytes: &[u8]) -> &str {
    &std::str::from_utf8(&card_set_bytes).unwrap().trim()
}

fn is_carry_bit(current_byte: &u8, mask_bits: u8) -> bool {
    0 != current_byte & (1 << mask_bits)
}

fn read_bits_from_byte(current_byte: &u8, mask_bits: u8, delta_shift: u8, out_bits: u32) -> u32 {
    (((current_byte & ((1 << mask_bits) - 1)) as u32) << delta_shift) | out_bits
}

fn deserialize_card(
    mut deck_bytes: &mut Vec<u8>,
    card_set: &str,
    card_set_padding: &usize,
    prev_card_number: &mut u32,
) -> Card {
    let mut current_byte: u8 = get_u8(&mut deck_bytes);
    let card_count = (&current_byte >> 6) + 1;
    let card_parallel_id = &current_byte >> 3 & 0x07;

    let mut card_number: u32 = read_bits_from_byte(&current_byte, 3 - 1, 0, 0);
    let mut delta_shift: u8 = 3;
    let u8_bits = 8; // u8::BITS in the future
    if is_carry_bit(&current_byte, &delta_shift - 1) {
        loop {
            current_byte = get_u8(&mut deck_bytes);
            card_number =
                read_bits_from_byte(&current_byte, u8_bits - 1, &delta_shift - 1, card_number);
            if !is_carry_bit(&current_byte, u8_bits - 1) {
                break;
            }
            delta_shift = delta_shift + u8_bits - 1;
        }
    }

    *prev_card_number = card_number + *prev_card_number;

    Card {
        number: format!(
            "{s}-{:0>p$}",
            prev_card_number,
            s = card_set,
            p = card_set_padding
        ),
        parallel_id: card_parallel_id,
        count: card_count,
    }
}

fn parse_deck(mut deck_bytes: &mut Vec<u8>) -> Deck {
    let version_and_digi_egg_count = get_u8(&mut deck_bytes);
    let version = version_and_digi_egg_count >> 4;
    assert_eq!(version, VERSION);

    let digi_egg_set_count = (version_and_digi_egg_count & 0x0F) as usize;
    let checksum = get_u8(&mut deck_bytes);
    let deck_name_length = get_u8(&mut deck_bytes) as usize;
    let total_card_bytes = deck_bytes.len() - deck_name_length;

    let computed_checksum = compute_checksum(total_card_bytes, deck_bytes);
    assert_eq!(checksum, computed_checksum);

    let mut cards: Vec<Card> = Vec::new();

    while deck_bytes.len() > deck_name_length {
        // Card Set Header
        // - Card Set
        let card_set_bytes = get_u32(&mut deck_bytes);
        let card_set = get_str_from_bytes(&card_set_bytes);
        // - Card Set Zero Padding and Count
        let padding_and_set_count = &get_u8(&mut deck_bytes);
        let card_set_padding = ((padding_and_set_count >> 6) + 1) as usize;
        let card_set_count = padding_and_set_count & 0x3F;

        let mut prev_card_number: u32 = 0;

        for _ in 0..card_set_count {
            let card = deserialize_card(
                &mut deck_bytes,
                &card_set,
                &card_set_padding,
                &mut prev_card_number,
            );
            cards.push(card);
        }
    }

    let deck_name: &str = get_str_from_bytes(&deck_bytes);
    return Deck {
        digi_eggs: (&cards[..digi_egg_set_count]).to_vec(),
        deck: (&cards[digi_egg_set_count..]).to_vec(),
        name: deck_name.to_string(),
    };
}

pub fn decode(deck_code_str: &String) -> Deck {
    let (prefix, deck_code) = deck_code_str.split_at(3);
    assert_eq!(PREFIX, prefix);

    let mut deck_bytes: Vec<u8> = decode_b64(deck_code).unwrap();
    return parse_deck(&mut deck_bytes);
}
