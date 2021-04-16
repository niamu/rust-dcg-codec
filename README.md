# rust-dcg-codec

This repository is a [Rust](https://www.rust-lang.org) implementation of the [Digimon Card Game (2020)](https://world.digimoncard.com/) deck codec. The original reference implementation is hosted at [niamu/digimon-card-game](https://github.com/niamu/digimon-card-game).

## Usage

Using the compiled binary:

```
$ dcg-codec --help
dcg-codec 0.4.0
Digimon Card Game 2020 deck codec

USAGE:
    rust-dcg-codec [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --encode <deck>
        --decode <deck-code-str>
```

### Decode

```
$ dcg-codec --decode DCGApQzQlQyIIHBU1QxIEEBQlQxIIQFAsYCQU0QQlQyIIHEBEJUMyCGxALFAYNCwYUNU1QxIEbCwYMBiEUCRGlnaSBCcm9zOiBSYWduYWxvYXJkbW9uIFJlZCAoeW91dHUuYmUvbzBLb1cyd3doUjQp

(
    digi_eggs: [
        (
            number: "BT2-001",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "ST1-01",
            parallel_id: 0,
            count: 1,
        ),
    ],
    deck: [
        (
            number: "BT1-009",
            parallel_id: 0,
            count: 1,
        ),
        (
            number: "BT1-019",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "BT1-020",
            parallel_id: 0,
            count: 2,
        ),
        (
            number: "BT1-085",
            parallel_id: 1,
            count: 2,
        ),
        (
            number: "BT2-016",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "BT3-008",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "BT3-013",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "BT3-016",
            parallel_id: 0,
            count: 3,
        ),
        (
            number: "BT3-018",
            parallel_id: 0,
            count: 2,
        ),
        (
            number: "BT3-019",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "BT3-072",
            parallel_id: 0,
            count: 3,
        ),
        (
            number: "ST1-02",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "ST1-03",
            parallel_id: 0,
            count: 4,
        ),
        (
            number: "ST1-06",
            parallel_id: 0,
            count: 3,
        ),
        (
            number: "ST1-07",
            parallel_id: 0,
            count: 1,
        ),
        (
            number: "ST1-07",
            parallel_id: 1,
            count: 3,
        ),
        (
            number: "ST1-16",
            parallel_id: 0,
            count: 2,
        ),
    ],
    name: "Digi Bros: Ragnaloardmon Red (youtu.be/o0KoW2wwhR4)",
)
```

## License

Copyright © 2021 Brendon Walsh.

Licensed under the EPL (see the file LICENSE).
