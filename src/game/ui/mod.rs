use super::utils::types;
use yacurses::*;
use std::fmt::format;


const LABEL_OFFSET: u32 = 2;
const BUTTON_HEIGHT: u32 = 2;
const BUTTON_WIDTH: u32 = 12;


pub struct Button {
    pub label: String,
    pub position: super::utils::types::Vector2d,
    pub focused: bool,
}

pub struct Checkbox {
    pub label: String,
    pub position: super::utils::types::Vector2d,
    pub selected: bool,
}

pub struct Label {
    pub label: String,
    pub position: super::utils::types::Vector2d,
}

pub struct Number {
    pub label: String,
    pub value: u32,
    pub position: super::utils::types::Vector2d,
}


impl Label {
    pub fn draw(&self, window: &mut Curses) {
        window.set_attributes(Attributes::UNDERLINE, true);
        window.move_cursor(Position {x: self.position.x, y: self.position.y });
        window.print_str(&format!("{:^}", &self.label));
        window.set_attributes(Attributes::UNDERLINE, false);
    }
}


impl Number {
    pub fn draw(&self, window: &mut Curses) {
        window.set_attributes(Attributes::UNDERLINE, true);
        window.move_cursor(Position {x: self.position.x, y: self.position.y });
        window.print_str(&format!("{}:", &self.label));
        window.set_attributes(Attributes::UNDERLINE, false);
        window.set_attributes(Attributes::BOLD, true);
        //window.move_cursor(Position {x: self.position.x+label.len()+4, y: self.position.y });
        window.print_str(&format!(" {:>8}", &self.value));
        window.set_attributes(Attributes::BOLD, false);
    }
}

impl Button {
    pub fn draw(&self, window: &mut Curses) {

        if self.focused {
            window.set_attributes(Attributes::BOLD, true);
        }

        super::utils::rectangle(window, &self.position, &super::utils::types::Size{ w: BUTTON_WIDTH, h: BUTTON_HEIGHT});
        window.move_cursor(Position {x: self.position.x+LABEL_OFFSET, y: self.position.y+1 });
        window.print_str(&format!("{:^10}", &self.label));

        if self.focused {
            window.set_attributes(Attributes::BOLD, false);
        }
    }
}

impl Checkbox {
    pub fn draw(&self, window: &mut Curses) {
        super::utils::rectangle(window, &self.position, &super::utils::types::Size{ w: BUTTON_WIDTH, h: BUTTON_HEIGHT});
        window.move_cursor(Position {x: self.position.x+LABEL_OFFSET, y: self.position.y+1 });
        let mark =  if self.selected { CursesGlyph::from('o') } else { CursesGlyph::from(' ') };
        if self.selected {
            window.set_attributes(Attributes::BOLD, true);
        }
        window.print_ch(mark);
        window.print_str(&format!("{:^9}", &self.label));

        if self.selected {
            window.set_attributes(Attributes::BOLD, false);
        }
    }
}

pub struct Ui {
    

}
