use sdl2::render::Canvas;

use crate::backend::cards::dante::DanteCard;

// https://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait
pub trait Card {
    fn get_path(&self) -> String;
    fn show(&self, x: i32, y: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>);
} 

pub struct DrawnCard {
    x: u32,
    y: u32,
    sizex: u32,
    sizey: u32,
    card: Box<dyn Card>
}

struct Deck {
    ruler: Vec<Box<dyn Card>>,
    main: Vec<Box<dyn Card>>,
    magic_stone: Vec<Box<dyn Card>>,
    side: Vec<Box<dyn Card>>
}

impl Deck {
    fn default() -> Deck {
        Deck {
            ruler: vec![
                Box::new(DanteCard {}),
            ],
            main: vec![],
            magic_stone: vec![],
            side: vec![],
        }
    }

    pub fn show(&self, is_bot: bool, canvas: &mut Canvas<sdl2::video::Window>) {
        // TODO : SHOW THE REST
        for ruler in &self.ruler {
            ruler.show(
                0,
                (if is_bot {
                    0
                } else {
                    let partial_size: i32 = (canvas.output_size().unwrap().1/2).try_into().unwrap();
                    partial_size + 1i32
                }),
                canvas,
            );
        }
    }
}


struct Player {
    deck: Deck
}

impl Player {
    pub fn show(&self, is_bot: bool, canvas: &mut Canvas<sdl2::video::Window>) {
        self.deck.show(is_bot, canvas)
    }
}

pub struct Game {
    bot: Player,
    player: Player,
    cardarray: Vec<DrawnCard>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            bot: Player {
                deck: Deck::default()
            },
            player: Player {
                deck: Deck::default()
            },
            cardarray: vec![],
        }
    }

    pub fn show(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.bot.show(
            true, 
            canvas,
        );
        self.player.show(
            false, 
            canvas,
        );
    }

    pub fn add_drawn_card(&mut self, drawn_card: DrawnCard) {
        self.cardarray.push(drawn_card);
    }
}