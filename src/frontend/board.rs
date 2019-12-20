use super::log::LogView;
use super::menu::{Color, ColorScheme, Preferences};

use crate::backend;
use backend::{Board, Move, Piece, PieceType, Square, Team};
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
    ChoosingPiece(Team),
    ChoosingMove(Team, Square, Vec<Move>, bool),
}

pub const SQUARE_WIDTH: usize = 3;

pub struct BoardView {
    preferences: Preferences,
    board: Board,
    window: pancurses::Window,

    log: Rc<RefCell<LogView>>, // TODO not sure if this is the best way to do this
    backend_channel: Rc<RefCell<FrontendEndpoint>>,

    cursor: Square,
    state: State,
}
impl BoardView {
    pub fn new(
        preferences: Preferences,
        window: pancurses::Window,
        log: Rc<RefCell<LogView>>,
        backend_channel: Rc<RefCell<FrontendEndpoint>>,
    ) -> BoardView {
        let result = BoardView {
            preferences: preferences,
            board: Board::new(),
            window: window,
            log: log,
            backend_channel: backend_channel,
            cursor: Square{ x: 0, y: 7 },
            state: State::Waiting,
        };
        result.window.draw_box(ACS_VLINE(), ACS_HLINE());

        result
    }

    pub fn start_selecting_piece(&mut self, team: Team) {
        self.state = State::ChoosingPiece(team);
    }

    pub fn continue_jumping(&mut self, team: Team, square: Square, valid_moves: Vec<Move>) {
        self.state = State::ChoosingMove(team, square, valid_moves, true);
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
    }

    pub fn do_action(&mut self) {
        match self.process_state() {
            Some(new_state) => self.state = new_state,
            None => (),
        }
    }

    fn process_state(&self) -> Option<State> {
        match &self.state {
            State::Waiting => None,
            State::ChoosingPiece(team) => {
                // log!(self.log, "choosing piece.. {:?}", team);
                match self.board.get_piece_at(&self.cursor) {
                    Some(piece) if piece.team == *team => {
                        let valid_moves = self.board.get_valid_moves_for_piece_at(&self.cursor);
                        if valid_moves.is_empty() {
                            // log!(self.log, "Piece at {} has no valid moves", self.cursor);
                            return None;
                        }
                        // log!(self.log, "valid moves: {:?}", valid_moves);
                        return Some(State::ChoosingMove(*team, self.cursor, valid_moves, false));
                    },
                    _ => return None,
                };
            },
            State::ChoosingMove(team, piece_pos, valid_moves, only_jumps) => {
                // log!(self.log, "choosing move.. {:?} {:?}", valid_moves, only_jumps);
                if self.cursor == *piece_pos {
                    // Cancel move
                    if *only_jumps {
                        log!(self.log, "Jump canceled");
                        self.send_cancel_move_to_backend();
                        return Some(State::Waiting);
                    } else {
                        log!(self.log, "Move canceled");
                        return Some(State::ChoosingPiece(*team));
                    }
                }
                let mv = Move{ from: *piece_pos, to: self.cursor };
                if valid_moves.contains(&mv) {
                    // log!(self.log, "sending move {:?}", mv);
                    self.send_move_to_backend(mv);
                    return Some(State::Waiting);
                } else {
                    log!(self.log, "Illegal move {}", mv);
                    return None;
                }
            },
        }
    }

    fn send_cancel_move_to_backend(&self) {
        self.send_msg(crate::channel::FrontToBackMessage::CancelMove);
    }

    fn send_move_to_backend(&self, mv: Move) {
        self.send_msg(crate::channel::FrontToBackMessage::Move(mv));
    }

    fn send_msg(&self, msg: crate::channel::FrontToBackMessage) {
        self.backend_channel.borrow_mut().tx.send(msg).expect("Could not send message"); // TODO better error handling
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
        self.board = board;
        // This gets rid of the wide-char artifacts, but not the most efficient
        // Doing this here instead of in draw() prevents flickering
        self.window.clearok(true);
    }

    pub fn draw(&mut self) {
        let pieces = self.board.get_pieces();
        for y in 0..Board::SIZE {
            for x in 0..Board::SIZE {
                // TODO blink cursor when piece selected, highlight valid moves?
                let left   = if self.cursor == (Square{x, y}) { "[" } else { " " };
                let center = Self::get_piece_glyph(pieces.get(&Square{x, y}), self.preferences.ascii);
                let right  = if self.cursor == (Square{x, y}) { "]" } else { " " };
                let ch = format!("{left}{center}{right}", left=left, center=center, right=right);
                let colors = match self.preferences.color_scheme {
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
