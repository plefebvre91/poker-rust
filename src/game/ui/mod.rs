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

pub trait UIFocusable {
    fn set_focus(&mut self, focus: bool) -> ();
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
    pub focused: bool,
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


impl UIFocusable for Button {
    fn set_focus(&mut self, focus: bool) -> () {
        self.focused = focus;
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


impl UIFocusable for Checkbox {
    fn set_focus(&mut self, focus: bool) -> () {
        self.focused = focus;
    }
}

impl UIElement for Checkbox {
    fn draw(&self, window: &mut Curses) {
        super::utils::rectangle(window, &self.position, &super::utils::types::Size{ w: BUTTON_WIDTH, h: BUTTON_HEIGHT});
        window.move_cursor(Position {x: self.position.x+LABEL_OFFSET, y: self.position.y+1 });
        let mark =  if self.selected { CursesGlyph::from('o') } else { CursesGlyph::from(' ') };

        if self.focused {
            window.set_attributes(Attributes::BOLD, true);
        }

        if self.selected {
            window.set_attributes(Attributes::ITALIC, true);
        }
        window.print_ch(mark);
        window.print_str(&format!("{:^9}", &self.label));

        if self.selected {
            window.set_attributes(Attributes::ITALIC, false);
        }

        if self.focused {
            window.set_attributes(Attributes::BOLD, false);
        }
    }
}

pub trait UIFocusableElement: UIElement + UIFocusable {}
impl UIFocusableElement for Checkbox {}
impl UIFocusableElement for Button {}

pub struct Ui {
    static_elements: Vec<Box<dyn UIElement>>,
    dynamic_elements: Vec<Box<dyn UIFocusableElement>>,
    current_element_id: usize,
}

enum UIEvent {
    PressLeft,
    PressRight,
    PressUp,
    PressDown,
    PressEnter,
}

enum UIState {
    UIBet,
    UIDraw,
}

impl Ui {
    pub fn update(&self, window: &mut Curses) {
        for element in self.static_elements.iter() {
            element.draw(window);
        }
        for element in self.dynamic_elements.iter() {
            element.draw(window);
        }
    }

    pub fn add_dynamic_element(&mut self, element: Box<dyn UIFocusableElement>) {
        self.dynamic_elements.push(element);
    }

    pub fn add_static_element(&mut self, element: Box<dyn UIElement>) {
        self.static_elements.push(element);
    }

    fn select(&mut self, direction: i32) {
        let new_id = ((self.current_element_id as i32 + direction).rem_euclid(self.dynamic_elements.len() as i32)) as usize;
        self.dynamic_elements[self.current_element_id].set_focus(false);
        self.current_element_id = new_id;
        self.dynamic_elements[self.current_element_id].set_focus(true);
    }

    pub fn next(&mut self) {
        self.select(1);
    }

    pub fn previous(&mut self) {
        self.select(-1);
    }
}



pub fn new() -> Ui {
    Ui {
        static_elements: vec![],
        dynamic_elements: vec![],
        current_element_id: 0,
    }
}
