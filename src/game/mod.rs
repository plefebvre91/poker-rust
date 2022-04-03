use rand::prelude::*;
use std::{thread, time};

use yacurses::*;

#[path="utils/mod.rs"]
pub mod utils;

#[path="ui/mod.rs"]
pub mod ui;

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
    ui: ui::Ui,
}


impl Default for Game {
    fn default() -> Game {
        Game {
            board: [1,15,28,39,50],
            selected: Vec::new(),
	        chips: 1000,
            window: Curses::init(),
            ui: ui::new(),
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
        self.window.clear().ok();
        self.window.set_echo(false).ok();
        self.window.set_cursor_visibility(CursorVisibility::Invisible).ok();
    }

    fn display(&mut self) -> () {
        let mut counter = 0;
        utils::rectangle(&mut self.window, &utils::types::Vector2d {x: 0, y: 0}, &utils::types::Size{ w: 31, h: 4 });
        self.window.move_cursor(Position {x: 10, y: 2});
        self.window.print_str("Poker Rust");

        self.ui.update(&mut self.window);
    

        utils::rectangle(&mut self.window, &utils::types::Vector2d {x: 0, y: 5}, &utils::types::Size{ w: 31, h: 16 });
        utils::rectangle(&mut self.window, &utils::types::Vector2d {x: 32, y: 5}, &utils::types::Size{ w: 110, h: 16 });
        for i in self.board {
            let id: usize = i as usize;
            let position = utils::types::Vector2d {x: counter*20 + 42, y: 7 };
            self.cards[id].draw(&mut self.window, position);
            counter += 1;
        }
    }

    fn is_finished(&self) -> bool {
	       return self.chips == 0;
    }


    pub fn run(&mut self) -> () {
        while !self.is_finished() {
            self.display();
            self.window.refresh().ok();
                match self.window.poll_events() {
                Some(ArrowLeft) => break,
                _ => (),
             }
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
