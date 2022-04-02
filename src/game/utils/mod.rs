use yacurses::*;

#[path="types.rs"]
pub mod types;

fn hline(window: &mut Curses, x: u32, y: u32, length: u32) {
    window.move_cursor(Position {x: x, y: y}).ok();
    for _ in 0 .. length {
        window.print_ch(window.acs_hline()).ok();
    }
}

fn vline(window: &mut Curses, x: u32, y: u32, length: u32) {
    for offset in 0 .. length {
        window.move_cursor(Position {x: x, y: y+offset}).ok();
        window.print_ch(window.acs_vline()).ok();
    }
}



pub const ASCII_HEART: &str = r"
 _  _
( \/ )
 \  /
  \/
";

pub const ASCII_SPADE: &str = r"
   .
  / \
 (_,_)
   I  ";

pub const ASCII_DIAMOND: &str = r"
  /\
 /  \
 \  /
  \/";

pub const ASCII_CLUB: &str = r"
   _
  ( )
 (_x_)
   I";



pub fn draw_ascii(window: &mut Curses, ascii: &str, position: &types::Vector2d) {
    window.move_cursor(Position {x: position.x, y: position.y}).ok();
    let mut i = 0;
    for item in ascii.split('\n') {
        window.print_str(item).ok();
        window.move_cursor(Position {x: position.x, y: position.y+i}).ok();
        i = i +1;
    }
}

pub fn rectangle(window: &mut Curses, position: &types::Vector2d, size: &types::Size)
{
    hline(window, position.x, position.y , size.w);
    hline(window, position.x, position.y+size.h , size.w);

    vline(window, position.x, position.y, size.h);
    vline(window, position.x + size.w, position.y, size.h);

    window.move_cursor(Position {x: position.x, y: position.y}).ok();
    window.print_ch(window.acs_ulcorner()).ok();

    window.move_cursor(Position {x: position.x+size.w, y: position.y}).ok();
    window.print_ch(window.acs_urcorner()).ok();

    window.move_cursor(Position {x: position.x, y: position.y+size.h}).ok();
    window.print_ch(window.acs_llcorner()).ok();

    window.move_cursor(Position {x: position.x+size.w, y: position.y+size.h}).ok();
    window.print_ch(window.acs_lrcorner()).ok();
}
