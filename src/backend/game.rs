#[derive(Clone)]
struct Card {

}

struct Deck {
    ruler: Vec<Card>,
    main: Vec<Card>,
    magic_stone: Vec<Card>,
    side: Vec<Card>
}

impl Deck {
    fn default() -> Deck {
        Deck {
            ruler: [].to_vec(),
            main: [].to_vec(),
            magic_stone: [].to_vec(),
            side: [].to_vec(),
        }
    }
}

struct Player {
    deck: Deck
}

pub struct Game {
    player1: Player,
    player2: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Player {
                deck: Deck::default()
            },
            player2: Player {
                deck: Deck::default()
            },
        }
    }
}