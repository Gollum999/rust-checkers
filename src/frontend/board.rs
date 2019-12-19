use super::args::{Args, Color, ColorScheme};
use super::log::LogView;

use crate::backend;
use backend::{Board, Piece, PieceType, Square, Team};
use crate::channel::FrontendEndpoint;

use std::cell::RefCell;
use std::rc::Rc;

use pancurses::{
    A_BLINK, A_NORMAL,
    ACS_HLINE, ACS_VLINE,
    Input,
};

enum State {
    Waiting,
    ChoosingPiece,
    ChoosingMove(Piece),
}

pub const SQUARE_WIDTH: usize = 3;

pub struct BoardView {
    args: Args,
    board: Option<Board>,
    window: pancurses::Window,
    log: Rc<RefCell<LogView>>, // TODO not sure if this is the best way to do this
    cursor: Square,
    state: State,
}
impl BoardView {
    pub fn new(args: Args, window: pancurses::Window, log: Rc<RefCell<LogView>>) -> BoardView {
        let board = BoardView {
            args: args,
            board: None,
            window: window,
            log: log,
            cursor: Square{ x: 0, y: 7 },
            state: State::Waiting,
        };
        board.window.draw_box(ACS_VLINE(), ACS_HLINE());

        board
    }

    // TODO if selecting move, limit to valid moves?
    pub fn move_cursor(&mut self, dir: Input) {
        match dir {
            Input::KeyLeft => self.cursor.x -= 2,
            Input::KeyRight => self.cursor.x += 2,
            Input::KeyUp => {
                if self.cursor.x % 2 == 0 {
                    self.cursor.x += 1;
                } else {
                    self.cursor.x -= 1;
                }
                self.cursor.y -= 1;
            },
            Input::KeyDown => {
                if self.cursor.x % 2 == 0 {
                    self.cursor.x += 1;
                } else {
                    self.cursor.x -= 1;
                }
                self.cursor.y += 1;
            },
            _ => panic!("Bad dir passed to move_cursor: {:?}", dir),
        }
        self.cursor.x = (self.cursor.x + Board::SIZE) % Board::SIZE;
        self.cursor.y = (self.cursor.y + Board::SIZE) % Board::SIZE;
        log!(self.log, "move_cursor {:?}, new pos = {}", dir, self.cursor);
    }

    pub fn do_action(&mut self) {
        // println!("do_action");
        match self.state {
            State::Waiting => (),
            State::ChoosingPiece => {
                log!(self.log, "choosing piece.."); // TODO
                // let piece = self.board.get_piece_at(self.cursor);
                // self.state = State::ChoosingMove(piece);
            },
            State::ChoosingMove(piece) => {
                log!(self.log, "choosing move.."); // TODO
                // self.send_msg(Message::Move{ from: piece.pos(), to: self.cursor });
                // if !move.is_jump() || self.board.get_jumps_for(piece).is_empty() {
                //     self.state = State::Waiting;
                // }
            },
        }
    }

    fn get_piece_glyph(piece: Option<&Piece>, ascii: bool) -> char {
        match piece {
            Some(piece) => match (piece.team, piece.piece_type, ascii) {
                (Team::Light, PieceType::Man,  true)  => 'O', // TODO better chars
                (Team::Dark,  PieceType::Man,  true)  => '=',
                (Team::Light, PieceType::King, true)  => '@',
                (Team::Dark,  PieceType::King, true)  => '#',
                (Team::Light, PieceType::Man,  false) => '⛂',
                (Team::Dark,  PieceType::Man,  false) => '⛀',
                (Team::Light, PieceType::King, false) => '⛃',
                (Team::Dark,  PieceType::King, false) => '⛁',
            },
            None => ' ',
        }
    }

    pub fn set_board_state(&mut self, board: Board) {
        self.board = Some(board);
        // This gets rid of the wide-char artifacts, but not the most efficient
        // Doing this here instead of in draw() prevents flickering
        self.window.clearok(true);
    }

    pub fn draw(&mut self) {
        if self.board.is_none() {
            return;
        }
        let pieces = self.board.as_ref().unwrap().get_pieces();
        for y in 0..Board::SIZE {
            for x in 0..Board::SIZE {
                let left   = if self.cursor == (Square{x, y}) { "[" } else { " " };
                let center = Self::get_piece_glyph(pieces.get(&Square{x, y}), self.args.ascii);
                let right  = if self.cursor == (Square{x, y}) { "]" } else { " " };
                let ch = format!("{left}{center}{right}", left=left, center=center, right=right);
                let colors = match self.args.color_scheme {
                    ColorScheme::WhiteRed   => [Color::WhiteOnRed as i16,   Color::RedOnWhite as i16],
                    ColorScheme::RedBlack   => [Color::RedOnBlack as i16,   Color::BlackOnRed as i16],
                    ColorScheme::WhiteBlack => [Color::WhiteOnBlack as i16, Color::BlackOnWhite as i16],
                };
                let real_x = (x * SQUARE_WIDTH as i8 + 1) as i32;
                let real_y = (y + 1) as i32;
                let color_pair = colors[((x + y + 1) % 2) as usize];
                // let attrs = if self.cursor == (Square{x, y}) { A_BLINK } else { A_NORMAL };
                self.window.color_set(color_pair);
                self.window.mvaddstr(real_y, real_x, format!("{char:^width$}", char=ch, width=SQUARE_WIDTH));
            }
        }
        self.window.refresh();
    }
}
