use yacurses::*;

use crate::game::utils;

#[derive(Debug)]
pub enum Value {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub enum Color {
    Club,
    Heart,
    Diamond,
    Spade,
}

pub struct Card {
    pub color: Color,
    pub value: Value,
}

pub fn new() -> Card {
    Card { color: Color::Spade, value: Value::Ace }
}

impl Default for Card {
    fn default() -> Card {
	       Card { color: Color::Spade, value: Value::Ace }
    }
}


impl Card {
    pub fn draw_card(&self, window: &mut Curses, position: utils::types::Vector2d , value: &str, color: &str) {
        let size = utils::types::Size { w: 12, h: 7 };


        utils::rectangle(window, &position, &size);
        window.move_cursor(Position {x: position.x+2, y: position.y+1}).ok();
        window.print_str(value).ok();
        window.move_cursor(Position {x: position.x+size.w-2, y: position.y+size.h-1 }).ok();
        window.print_str(value).ok();

        utils::draw_ascii(window, color, &utils::types::Vector2d{x: position.x + 3, y: position.y+2 });
    }

    pub fn draw(&self, window: &mut Curses, position: utils::types::Vector2d) -> () {
        let value = match &self.value {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight  => "8",
            Value::Nine  => "9",
            Value::Ten  => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
        };

        let color = match self.color {
            Color::Club => utils::ASCII_CLUB,
            Color::Diamond => utils::ASCII_DIAMOND,
            Color::Spade => utils::ASCII_SPADE,
            Color::Heart => utils::ASCII_HEART,
        };

        self.draw_card(window, position, value, color);
    }
}
