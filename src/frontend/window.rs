extern crate pancurses;

use super::super::backend;
use backend::{Board, Piece, PieceType, Team};

use pancurses::{
    ACS_HLINE, ACS_VLINE,
    COLOR_BLACK, COLOR_RED, COLOR_WHITE,
    initscr, init_pair, Input, noecho, start_color,
};

#[derive(Debug)]
pub struct WindowError {
    code: i32,
    message: String,
}

impl From<i32> for WindowError {
    fn from(code: i32) -> Self {
        WindowError {
            code: code,
            message: format!("Error code {}", code), // TODO prettify common error codes
        }
    }
}

enum ColorScheme {
    WhiteRed,
    RedBlack,
    WhiteBlack,
}

#[repr(i16)]
enum Color {
    RedOnWhite   = 1,
    WhiteOnRed   = 2,
    RedOnBlack   = 3,
    BlackOnRed   = 4,
    WhiteOnBlack = 5,
    BlackOnWhite = 6,
}

struct Point {
    x: i32,
    y: i32,
}

macro_rules! log {
    ( $window:expr, $( $arg:expr ),* ) => {
        let old_pos = Point{ x: $window.get_cur_x(), y: $window.get_cur_y() };
        const LOG_POS: Point = Point{ x: 1, y: 12 };
        $window.mv(LOG_POS.y, LOG_POS.x);
        $window.insertln();
        $window.addstr(format!($($arg),*));
        $window.mv(old_pos.y, old_pos.x);
    };
}

const SQUARE_WIDTH: usize = 3;
const COLOR_SCHEME: ColorScheme = ColorScheme::RedBlack;

pub struct Window<'a> {
    main_window: pancurses::Window,
    board_window: pancurses::Window,

    board: &'a Board,
}

impl<'a> Window<'a> {
    pub fn new(board: &'a Board) -> Result<Window, WindowError> {
        let main_window = initscr();
        let sub_window = main_window.subwin(2 + Board::SIZE, 2 + Board::SIZE * SQUARE_WIDTH as i32, 1, 1)?;
        let w = Window {
            main_window: main_window,
            board_window: sub_window,
            board: board,
        };
        w.main_window.keypad(true); // Allow control characters
        start_color(); // Enable colors
        noecho(); // Don't echo typed characters

        init_pair(Color::RedOnWhite as i16,   COLOR_RED,   COLOR_WHITE);
        init_pair(Color::WhiteOnRed as i16,   COLOR_WHITE, COLOR_RED);
        init_pair(Color::RedOnBlack as i16,   COLOR_RED,   COLOR_BLACK);
        init_pair(Color::BlackOnRed as i16,   COLOR_BLACK, COLOR_RED);
        init_pair(Color::WhiteOnBlack as i16, COLOR_WHITE, COLOR_BLACK);
        init_pair(Color::BlackOnWhite as i16, COLOR_BLACK, COLOR_WHITE);

        w.main_window.draw_box(ACS_VLINE(), ACS_HLINE());
        w.board_window.draw_box(ACS_VLINE(), ACS_HLINE());

        Ok(w)
    }

    fn get_piece_char(piece: &Piece) -> char {
        match (piece.team, piece.piece_type) {
            (Team::White, PieceType::Man)  => '⛀',
            (Team::Black, PieceType::Man)  => '⛂',
            (Team::White, PieceType::King) => '⛁',
            (Team::Black, PieceType::King) => '⛃',
        }
    }

    fn draw_board(&self) {
        // for (row_idx, row) in self.board.iter().enumerate() {
        for (y, row) in self.board.value().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let c = match cell {
                    Some(piece) => Self::get_piece_char(piece),
                    None => ' ',
                };
                let colors = match COLOR_SCHEME {
                    ColorScheme::WhiteRed   => [Color::WhiteOnRed as i16,   Color::RedOnWhite as i16],
                    ColorScheme::RedBlack   => [Color::RedOnBlack as i16,   Color::BlackOnRed as i16],
                    ColorScheme::WhiteBlack => [Color::WhiteOnBlack as i16, Color::BlackOnWhite as i16],
                };
                self.board_window.color_set(colors[(x + y + 1) % 2]);
                self.board_window.mvaddstr(
                    (y + 1) as i32,
                    (x * SQUARE_WIDTH + 1) as i32,
                    format!("{char:^width$}", char=c, width=SQUARE_WIDTH),
                );
            }
        }
    }

    pub fn run(&self) {
        loop {
            self.draw_board();

            // self.main_window.addstr("○●◯◖◗⬤⭗⭕⭘🔴🔵🞉🞊♛♕♔♚👑⛀⛂⛁⛃");
            self.board_window.refresh();
            self.main_window.refresh();

            match self.main_window.getch() {
                Some(Input::Character('q')) => break,
                Some(Input::KeyDC) => break,
                Some(_) => (),
                None => ()
            }
        }
    }
}
