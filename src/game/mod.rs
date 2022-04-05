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

        // Add labels
        self.ui.add_static_element(Box::new(ui::Label { label: String::from("Actions"), position: utils::types::Vector2d {x: 12, y: 6}}));
        self.ui.add_static_element(Box::new(ui::Number { label: String::from("Bet"), value: 1000, position: utils::types::Vector2d {x: 9, y: 18}}));
        self.ui.add_static_element(Box::new(ui::Number { label: String::from("Win"), value: 12000, position: utils::types::Vector2d {x: 9, y: 19}}));

        // Buttons and checkoxes
        self.ui.add_dynamic_element(Box::new(ui::Button { label: String::from("BET+"), position: utils::types::Vector2d {x: 9, y: 8}, focused: true }));
        self.ui.add_dynamic_element(Box::new(ui::Button { label: String::from("BET-"), position: utils::types::Vector2d {x: 9, y: 11}, focused: false }));
        self.ui.add_dynamic_element(Box::new(ui::Button { label: String::from("BET"), position: utils::types::Vector2d {x: 9, y: 14}, focused: false}));
        self.ui.add_dynamic_element(Box::new(ui::Checkbox { label: String::from("Keep"), position: utils::types::Vector2d {x: 42, y: 17}, selected: false, focused: false }));
        self.ui.add_dynamic_element(Box::new(ui::Checkbox { label: String::from("Keep"), position: utils::types::Vector2d {x: 62, y: 17}, selected: false, focused: false }));
        self.ui.add_dynamic_element(Box::new(ui::Checkbox { label: String::from("Keep"), position: utils::types::Vector2d {x: 82, y: 17}, selected: false, focused: false }));
        self.ui.add_dynamic_element(Box::new(ui::Checkbox { label: String::from("Keep"), position: utils::types::Vector2d {x: 102, y: 17}, selected: false, focused: false }));
        self.ui.add_dynamic_element(Box::new(ui::Checkbox { label: String::from("Keep"), position: utils::types::Vector2d {x: 122, y: 17}, selected: false, focused: false }));
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
        self.init();
        while !self.is_finished() {
            self.display();
            self.window.refresh().ok();
                match self.window.poll_events() {
                Some(CursesKey::ArrowLeft) => self.ui.previous(),
                Some(CursesKey::ArrowRight) => self.ui.next(),
                _ => (),
             }
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
