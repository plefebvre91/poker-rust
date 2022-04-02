use rand::prelude::*;
use std::{thread, time};

use yacurses::*;

#[path="utils/mod.rs"]
pub mod utils;

#[path="card.rs"]
pub mod card;


const NB_CARDS: usize = 52;
const NB_CHIPS: u32 = 1000;
const BOARD_SIZE: usize = 5;
const NB_MAX_SELECTED: usize = 5;
const EMPTY: i32 = -1;


pub struct Game {
    cards: [card::Card; NB_CARDS],
    chips: u32,
    board: [i32; BOARD_SIZE],
    selected: Vec<u32>,
    window: Curses,
}


impl Default for Game {
    fn default() -> Game {
        Game {
            board: [1,15,28,39,50],
            selected: Vec::new(),
	        chips: 1000,
            window: Curses::init(),
	        cards: [
        		card::Card {color: card::Color::Spade, value: card::Value::Ace },
        		card::Card {color: card::Color::Spade, value: card::Value::Two },
        		card::Card {color: card::Color::Spade, value: card::Value::Three },
        		card::Card {color: card::Color::Spade, value: card::Value::Four },
        		card::Card {color: card::Color::Spade, value: card::Value::Five },
        		card::Card {color: card::Color::Spade, value: card::Value::Six },
        		card::Card {color: card::Color::Spade, value: card::Value::Seven },
        		card::Card {color: card::Color::Spade, value: card::Value::Eight },
        		card::Card {color: card::Color::Spade, value: card::Value::Nine },
        		card::Card {color: card::Color::Spade, value: card::Value::Ten },
        		card::Card {color: card::Color::Spade, value: card::Value::Jack },
        		card::Card {color: card::Color::Spade, value: card::Value::Queen },
        		card::Card {color: card::Color::Spade, value: card::Value::King },

        		card::Card {color: card::Color::Club, value: card::Value::Ace },
        		card::Card {color: card::Color::Club, value: card::Value::Two },
        		card::Card {color: card::Color::Club, value: card::Value::Three },
        		card::Card {color: card::Color::Club, value: card::Value::Four },
        		card::Card {color: card::Color::Club, value: card::Value::Five },
        		card::Card {color: card::Color::Club, value: card::Value::Six },
        		card::Card {color: card::Color::Club, value: card::Value::Seven },
        		card::Card {color: card::Color::Club, value: card::Value::Eight },
        		card::Card {color: card::Color::Club, value: card::Value::Nine },
        		card::Card {color: card::Color::Club, value: card::Value::Ten },
        		card::Card {color: card::Color::Club, value: card::Value::Jack },
        		card::Card {color: card::Color::Club, value: card::Value::Queen },
        		card::Card {color: card::Color::Club, value: card::Value::King },

        		card::Card {color: card::Color::Diamond, value: card::Value::Ace },
        		card::Card {color: card::Color::Diamond, value: card::Value::Two },
        		card::Card {color: card::Color::Diamond, value: card::Value::Three },
        		card::Card {color: card::Color::Diamond, value: card::Value::Four },
        		card::Card {color: card::Color::Diamond, value: card::Value::Five },
        		card::Card {color: card::Color::Diamond, value: card::Value::Six },
        		card::Card {color: card::Color::Diamond, value: card::Value::Seven },
        		card::Card {color: card::Color::Diamond, value: card::Value::Eight },
        		card::Card {color: card::Color::Diamond, value: card::Value::Nine },
        		card::Card {color: card::Color::Diamond, value: card::Value::Ten },
        		card::Card {color: card::Color::Diamond, value: card::Value::Jack },
        		card::Card {color: card::Color::Diamond, value: card::Value::Queen },
        		card::Card {color: card::Color::Diamond, value: card::Value::King },

        		card::Card {color: card::Color::Heart, value: card::Value::Ace },
        		card::Card {color: card::Color::Heart, value: card::Value::Two },
        		card::Card {color: card::Color::Heart, value: card::Value::Three },
        		card::Card {color: card::Color::Heart, value: card::Value::Four },
        		card::Card {color: card::Color::Heart, value: card::Value::Five },
        		card::Card {color: card::Color::Heart, value: card::Value::Six },
        		card::Card {color: card::Color::Heart, value: card::Value::Seven },
        		card::Card {color: card::Color::Heart, value: card::Value::Eight },
        		card::Card {color: card::Color::Heart, value: card::Value::Nine },
        		card::Card {color: card::Color::Heart, value: card::Value::Ten },
        		card::Card {color: card::Color::Heart, value: card::Value::Jack },
        		card::Card {color: card::Color::Heart, value: card::Value::Queen },
        		card::Card {color: card::Color::Heart, value: card::Value::King },
            ],
	    }
    }
}


impl Game {
    fn init(&mut self) -> () {
        self.window.set_echo(false).ok();
        self.window.set_cursor_visibility(CursorVisibility::Invisible).ok();
        self.window.clear().ok();
    }

    fn display(&mut self) -> () {
        let mut counter = 0;
        for i in self.board {
            let id: usize = i as usize;
            let position = utils::types::Vector2d {x: counter*20, y: 10 };
            self.cards[id].draw(&mut self.window, position);
            counter += 1;
        }
    }

    fn is_finished(&self) -> bool {
	       return self.chips == 0;
    }


    pub fn run(&mut self) -> () {
        while !self.is_finished() {
            // match window.poll_events() {
            //     Some(ArrowLeft) => break,
            //     _ => (),
            //  }

            thread::sleep(time::Duration::from_millis(1000));
            self.display();
            self.window.refresh().ok();
        }
    }
}
