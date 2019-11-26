extern crate pancurses;

use super::super::backend;

use pancurses::{initscr};

pub struct Window<'a> {
    window: pancurses::Window,
    board: &'a backend::Board,
}

impl<'a> Window<'a> {
    pub fn new(board: &'a backend::Board) -> Window {
        let w = Window {
            window: initscr(),
            board: board,
        };
        w.window.keypad(true);

        w
    }
}
