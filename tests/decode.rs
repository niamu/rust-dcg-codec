use dcg_codec::codec::{decode, Card, Deck, Language};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_tests() {
        let st1_deck_encoded = "DCGAREdU1QxIEHBU1QxIE_CwcHBwUHBwUFBwcHBQUFTdGFydGVyIERlY2ssIEdhaWEgUmVkIFtTVC0xXQ";
        let st1_deck: Deck = Deck {
            digi_eggs: [Card {
                number: "ST1-01".to_string(),
                parallel_id: 0,
                count: 4,
            }]
            .to_vec(),
            deck: [
                Card {
                    number: "ST1-02".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-03".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-04".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-05".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-06".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-07".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "ST1-08".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-09".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-10".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "ST1-11".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "ST1-12".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-13".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-14".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-15".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "ST1-16".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
            ]
            .to_vec(),
            sideboard: [].to_vec(),
            icon: None,
            language: None,
            name: "Starter Deck, Gaia Red [ST-1]".to_string(),
        };

        let digi_bros_deck_encoded = "DCGApQzQlQyIIHBU1QxIEEBQlQxIIQFAsYCQU0QQlQyIIHEBEJUMyCGxALFAYNCwYUNU1QxIEbCwYMBiEUCRGlnaSBCcm9zOiBSYWduYWxvYXJkbW9uIFJlZCAoeW91dHUuYmUvbzBLb1cyd3doUjQp";
        let digi_bros_deck: Deck = Deck {
            digi_eggs: [
                Card {
                    number: "BT2-001".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-01".to_string(),
                    parallel_id: 0,
                    count: 1,
                },
            ]
            .to_vec(),
            deck: [
                Card {
                    number: "BT1-009".to_string(),
                    parallel_id: 0,
                    count: 1,
                },
                Card {
                    number: "BT1-019".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT1-020".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "BT1-085".to_string(),
                    parallel_id: 1,
                    count: 2,
                },
                Card {
                    number: "BT2-016".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT3-008".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT3-013".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT3-016".to_string(),
                    parallel_id: 0,
                    count: 3,
                },
                Card {
                    number: "BT3-018".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "BT3-019".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT3-072".to_string(),
                    parallel_id: 0,
                    count: 3,
                },
                Card {
                    number: "ST1-02".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-03".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-06".to_string(),
                    parallel_id: 0,
                    count: 3,
                },
                Card {
                    number: "ST1-07".to_string(),
                    parallel_id: 0,
                    count: 1,
                },
                Card {
                    number: "ST1-07".to_string(),
                    parallel_id: 1,
                    count: 3,
                },
                Card {
                    number: "ST1-16".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
            ]
            .to_vec(),
            sideboard: [].to_vec(),
            icon: None,
            language: None,
            name: "Digi Bros: Ragnaloardmon Red (youtu.be/o0KoW2wwhR4)"
                .to_string(),
        };

        let deck_with_sideboard_encoded = "DCGIkA_B4udAoEDAZydAUEAAYudAYQACQMKAQEBMQSLnQKBAxABi50DhQMIAwUCAwECAwGLnQOBAhgEnJ0BRgMCAwECAwABAiABCV9fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fXw";
        let deck_with_sideboard: Deck = Deck {
            digi_eggs: [
                Card {
                    number: "BT2-001".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-01".to_string(),
                    parallel_id: 0,
                    count: 1,
                },
            ]
            .to_vec(),
            deck: [
                Card {
                    number: "BT1-009".to_string(),
                    parallel_id: 0,
                    count: 1,
                },
                Card {
                    number: "BT1-019".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT1-020".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "BT1-085".to_string(),
                    parallel_id: 1,
                    count: 2,
                },
                Card {
                    number: "BT2-016".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT3-008".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT3-013".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "BT3-016".to_string(),
                    parallel_id: 0,
                    count: 3,
                },
                Card {
                    number: "BT3-018".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
                Card {
                    number: "BT3-019".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
            ]
            .to_vec(),
            sideboard: [
                Card {
                    number: "BT3-072".to_string(),
                    parallel_id: 0,
                    count: 3,
                },
                Card {
                    number: "ST1-02".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-03".to_string(),
                    parallel_id: 0,
                    count: 4,
                },
                Card {
                    number: "ST1-06".to_string(),
                    parallel_id: 0,
                    count: 3,
                },
                Card {
                    number: "ST1-07".to_string(),
                    parallel_id: 0,
                    count: 1,
                },
                Card {
                    number: "ST1-07".to_string(),
                    parallel_id: 1,
                    count: 3,
                },
                Card {
                    number: "ST1-16".to_string(),
                    parallel_id: 0,
                    count: 2,
                },
            ].to_vec(),
            icon: None,
            language: None,
            name: "_______________________________________________________________"
                .to_string(),
        };

        let deck_with_sideboard_and_language_encoded = "DCGOkA_B4udAoEDAZydAUEAAYudAYQACQMKAQEBMQSLnQKBAxABi50DhQMIAwUCAwECAwGLnQOBAhgEnJ0BRgMCAwECAwABAiABCV9fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fXw";
        let mut deck_with_sideboard_and_language = deck_with_sideboard.clone();
        deck_with_sideboard_and_language.language = Some(Language::English);

        let deck_with_sideboard_and_language_and_icon_encoded = "DCGSsA_h4udAoEDAZydAUEAAYudAYQACQMKAQEBMQSLnQKBAxABi50DhQMIAwUCAwECAwGLnQOBAhgEnJ0BRgMCAwECAwABAiABCUJBQ0stMDAxX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fXw";
        let mut deck_with_sideboard_and_language_and_icon =
            deck_with_sideboard_and_language.clone();
        deck_with_sideboard_and_language_and_icon.icon =
            Some("BACK-001".to_string());
        deck_with_sideboard_and_language_and_icon.name =
            deck_with_sideboard_and_language_and_icon
                .name
                .split_at(8)
                .1
                .to_string();

        let deck_with_sideboard_and_language_zh_and_icon_encoded = "DCGUsC_h4udAoEDAZydAUEAAYudAYQACQMKAQEBMQSLnQKBAxABi50DhQMIAwUCAwECAwGLnQOBAhgEnJ0BRgMCAwECAwABAiABCUJBQ0stMDAxX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fXw";
        let mut deck_with_sideboard_and_language_zh_and_icon =
            deck_with_sideboard_and_language_and_icon.clone();
        deck_with_sideboard_and_language_zh_and_icon.language =
            Some(Language::Chinese);

        let deck_with_sideboard_and_language_ko_and_icon_encoded = "DCGUsD_h4udAoEDAZydAUEAAYudAYQACQMKAQEBMQSLnQKBAxABi50DhQMIAwUCAwECAwGLnQOBAhgEnJ0BRgMCAwECAwABAiABCUJBQ0stMDAxX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fX19fXw";
        let mut deck_with_sideboard_and_language_ko_and_icon =
            deck_with_sideboard_and_language_and_icon.clone();
        deck_with_sideboard_and_language_ko_and_icon.language =
            Some(Language::Korean);

        // Deck encoding of v0 deck is stable
        assert_eq!(decode(digi_bros_deck_encoded), digi_bros_deck);

        // Deck decoding of v1 strings is stable
        assert_eq!(decode(st1_deck_encoded), st1_deck);

        // Deck decoding of v2 strings is stable
        assert_eq!(decode(deck_with_sideboard_encoded), deck_with_sideboard);

        // Deck decoding of v3 strings is stable
        assert_eq!(
            decode(deck_with_sideboard_and_language_encoded),
            deck_with_sideboard_and_language
        );

        // Deck decoding of v4 strings is stable
        assert_eq!(
            decode(deck_with_sideboard_and_language_and_icon_encoded),
            deck_with_sideboard_and_language_and_icon
        );

        // Deck decoding of v5 (Chinese) strings is stable
        assert_eq!(
            decode(deck_with_sideboard_and_language_zh_and_icon_encoded),
            deck_with_sideboard_and_language_zh_and_icon
        );

        // Deck decoding of v5 (Korean) strings is stable
        assert_eq!(
            decode(deck_with_sideboard_and_language_ko_and_icon_encoded),
            deck_with_sideboard_and_language_ko_and_icon
        );
    }
}
