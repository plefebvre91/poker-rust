use super::utils::types;
use super::utils;
use yacurses::*;
use std::fmt::format;


const LABEL_OFFSET: u32 = 2;
const BUTTON_HEIGHT: u32 = 2;
const BUTTON_WIDTH: u32 = 12;

pub trait UIElement {
    fn draw(&self, window: &mut Curses);
}

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


impl UIElement for Label {
    fn draw(&self, window: &mut Curses) {
        window.set_attributes(Attributes::UNDERLINE, true);
        window.move_cursor(Position {x: self.position.x, y: self.position.y });
        window.print_str(&format!("{:^}", &self.label));
        window.set_attributes(Attributes::UNDERLINE, false);
    }
}


impl UIElement for Number {
    fn draw(&self, window: &mut Curses) {
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

impl UIElement for Button {
    fn draw(&self, window: &mut Curses) {

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

impl UIElement for Checkbox {
    fn draw(&self, window: &mut Curses) {
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
    elements: Vec<Box<dyn UIElement>>,
}

impl Ui {
    pub fn update(&self, window: &mut Curses) {
        for element in self.elements.iter() {
            element.draw(window);
        }
    }
}

pub fn new() -> Ui {
    Ui {
        elements: vec![
            Box::new(Label { label: String::from("Actions"), position: super::utils::types::Vector2d {x: 12, y: 6}}),
            Box::new(Button { label: String::from("BET+"), position: super::utils::types::Vector2d {x: 9, y: 8}, focused: true }),
            Box::new(Button { label: String::from("BET-"), position: super::utils::types::Vector2d {x: 9, y: 11}, focused: false }),
            Box::new(Button { label: String::from("BET"), position: super::utils::types::Vector2d {x: 9, y: 14}, focused: false}),
            Box::new(Number { label: String::from("Bet"), value: 1000, position: super::utils::types::Vector2d {x: 9, y: 18}}),
            Box::new(Number { label: String::from("Win"), value: 12000, position: super::utils::types::Vector2d {x: 9, y: 19}}),
            Box::new(Checkbox { label: String::from("Keep"), position: super::utils::types::Vector2d {x: 42, y: 17}, selected: true }),
            Box::new(Checkbox { label: String::from("Keep"), position: super::utils::types::Vector2d {x: 62, y: 17}, selected: false }),
            Box::new(Checkbox { label: String::from("Keep"), position: super::utils::types::Vector2d {x: 82, y: 17}, selected: false }),
            Box::new(Checkbox { label: String::from("Keep"), position: super::utils::types::Vector2d {x: 102, y: 17}, selected: false }),
            Box::new(Checkbox { label: String::from("Keep"), position: super::utils::types::Vector2d {x: 122, y: 17}, selected: false }),
        ],
    }
}
