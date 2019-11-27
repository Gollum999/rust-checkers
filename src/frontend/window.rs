// extern crate libc;
// extern crate ncurses;
extern crate pancurses;

use super::super::backend;

use pancurses::{ACS_HLINE, ACS_VLINE, initscr, Input, noecho};
// use libc::{LcCategory, setlocale};

#[derive(Debug)]
pub struct WindowError {
    code: i32,
    message: String,
}

impl From<i32> for WindowError {
    fn from(code: i32) -> Self {
        WindowError {
            code: code,
            message: format!("Error code {}", code),
        }
    }
}

pub struct Window<'a> {
    main_window: pancurses::Window,
    board_window: pancurses::Window,

    board: &'a backend::Board,
}

impl<'a> Window<'a> {
    pub fn new(board: &'a backend::Board) -> Result<Window, WindowError> {
        let main_window = initscr();
        let sub_window = main_window.subwin(10, 30, 1, 1)?;
        let w = Window {
            main_window: main_window,
            board_window: sub_window,
            board: board,
        };
        w.main_window.keypad(true); // Allow control characters
        noecho(); // Don't echo typed characters
        // ncurses::setlocale(ncurses::LcCategory::all, "en_US.UTF-8");
        // libc::setlocale(libc::LcCategory::all, "en_US.UTF-8");
        // setlocale(LcCategory::all, "");

        w.main_window.draw_box(ACS_VLINE(), ACS_HLINE());
        w.board_window.draw_box(ACS_VLINE(), ACS_HLINE());

        Ok(w)
    }

    fn draw_board(&self) {
        // for (row_idx, row) in self.board.iter().enumerate() {
        for (row_idx, row) in self.board.value().iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                use std::convert::TryInto;
                let c = match cell {
                    Some(backend::Piece::MAN) => 'M',
                    Some(backend::Piece::KING) => 'K',
                    None => '.',
                };
                // TODO any way to clean this up?
                self.board_window.mvaddch((col_idx + 1).try_into().unwrap(), (row_idx * 3 + 1).try_into().unwrap(), c);
                // self.board_window.mvaddch(col_idx, row_idx, c)
            }
        }
        self.board_window.addstr("○●◯◖◗⬤⭗⭕⭘🔴🔵♛♕♔♚👑");
    }

    pub fn run(&self) {
        loop {
            self.draw_board();

            self.board_window.refresh();
            self.main_window.refresh();

            match self.main_window.getch() {
                // Some(Input::Character(c)) => { self.main_window.addch(c); },
                Some(Input::Character('q')) => break,
                Some(Input::KeyDC) => break,
                Some(_) => (),
                // Some(input) => { self.main_window.addstr(&format!("{:?}", input)); },
                None => ()
            }
        }
    }
}
