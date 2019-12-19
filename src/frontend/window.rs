extern crate pancurses;

use super::super::args; // TODO any way to clean this up?
use args::FrontendArgs as Args;
use super::super::backend;
use super::super::channel::FrontendEndpoint; // TODO any way to clean this up?
use backend::{Board, Piece, PieceType, Player, Square, Team}; // TODO this is a bit messy too

use std::cell::RefCell;
use std::rc::Rc;

use pancurses::{
    A_BLINK, A_NORMAL,
    ACS_HLINE, ACS_VLINE,
    COLOR_BLACK, COLOR_RED, COLOR_WHITE,
    endwin, initscr, init_pair, Input, noecho, start_color,
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

struct MenuItem{description: String, values: Vec<String>, default: String}
// const MENU: Vec<MenuItem> = vec![
//     MenuItem{
//         description: String::from("Player 1"),
//         values:      vec![String::from("Human"), String::from("CPU")],
//         default:     String::from("Human"),
//     },
//     MenuItem{
//         description: String::from("Player 2"),
//         values:      vec![String::from("Human"), String::from("CPU")],
//         default:     String::from("CPU"),
//     },
//     MenuItem{
//         description: String::from("Color Scheme"),
//         values:      vec![String::from("Red/Black"), String::from("White/Red"), String::from("White/Black")],
//         default:     String::from("Red/Black"),
//     },
//     MenuItem{
//         description: String::from("Fancy Icons"),
//         values:      vec![String::from("ON"), String::from("OFF")],
//         default:     String::from("ON"),
//     },
// ];

struct Settings {
    players: [Player; 2],
    color_scheme: args::ColorScheme,
    ascii: bool,
}
struct Menu {
    settings: Settings,
    selection: usize,
}
impl Menu {
    fn process_input() {
        // let key = self.main_window.getch();
        // match key {
        //     KEY_LEFT | KEY_RIGHT => cycle_option(key),
        //     KEY_UP => selection -= 1,
        //     KEY_DOWN => selection += 1,
        //     KEY_ENTER | KEY_SPACE if selection == 5 => {
        //         let settings = Settings {
        //             players: [
        //             ],
        //         };
        //         start_game(settings);
        //     },
        //     _ => (),
        // }
    }
}

struct LogView {
    window: pancurses::Window,
}
macro_rules! log {
    ( $log_view:expr, $( $arg:expr ),* ) => {{
        let log = $log_view.borrow_mut();
        log.window.mv(1, 1);
        log.window.insertln();
        log.window.addstr(format!($($arg),*));
    }};
}

enum State {
    Waiting,
    ChoosingPiece,
    ChoosingMove(Piece),
}

const SQUARE_WIDTH: usize = 3;

struct BoardView {
    args: Args,
    board: Option<Board>,
    window: pancurses::Window,
    log: Rc<RefCell<LogView>>, // TODO not sure if this is the best way to do this
    cursor: Square,
    state: State,
}
impl BoardView {
    fn new(args: Args, window: pancurses::Window, log: Rc<RefCell<LogView>>) -> BoardView {
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
    fn move_cursor(&mut self, dir: Input) {
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

    fn do_action(&mut self) {
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

    fn draw(&mut self) {
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
                    args::ColorScheme::WhiteRed   => [Color::WhiteOnRed as i16,   Color::RedOnWhite as i16],
                    args::ColorScheme::RedBlack   => [Color::RedOnBlack as i16,   Color::BlackOnRed as i16],
                    args::ColorScheme::WhiteBlack => [Color::WhiteOnBlack as i16, Color::BlackOnWhite as i16],
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

pub struct Window {
    args: Args,

    backend_channel: FrontendEndpoint,
    main_window: pancurses::Window,

    log: Rc<RefCell<LogView>>,
    board: BoardView,
}
impl Window {
    pub fn new(args: Args, backend_channel: FrontendEndpoint) -> Result<Window, WindowError> {
        let main_window = initscr();
        let board_window = main_window.subwin(
            2 + Board::SIZE as i32,
            2 + Board::SIZE as i32 * SQUARE_WIDTH as i32,
            0,
            0,
        )?;
        let log_window = main_window.subwin(
            main_window.get_max_y() - board_window.get_max_y(),
            main_window.get_max_x(),
            board_window.get_max_y(),
            0,
        )?;
        let log = LogView {
            window: log_window,
        };
        let log_rc = Rc::new(RefCell::new(log));
        let w = Window {
            args: args.clone(),
            backend_channel: backend_channel,
            main_window: main_window,
            board: BoardView::new(args, board_window, log_rc.clone()),
            log: log_rc,
        };
        w.main_window.keypad(true); // Allow control characters
        w.main_window.nodelay(true); // Input is non-blocking
        start_color(); // Enable colors
        noecho(); // Don't echo typed characters

        init_pair(Color::RedOnWhite as i16,   COLOR_RED,   COLOR_WHITE);
        init_pair(Color::WhiteOnRed as i16,   COLOR_WHITE, COLOR_RED);
        init_pair(Color::RedOnBlack as i16,   COLOR_RED,   COLOR_BLACK);
        init_pair(Color::BlackOnRed as i16,   COLOR_BLACK, COLOR_RED);
        init_pair(Color::WhiteOnBlack as i16, COLOR_WHITE, COLOR_BLACK);
        init_pair(Color::BlackOnWhite as i16, COLOR_BLACK, COLOR_WHITE);

        Ok(w)
    }

    fn process_input(&mut self) -> bool {
        let key = self.main_window.getch();
        const ESC: char = 27 as char;
        match key {
            None => (),
            Some(key) => match key {
                Input::KeyLeft | Input::KeyRight | Input::KeyUp | Input::KeyDown => self.board.move_cursor(key),
                Input::KeyEnter | Input::Character('\n') | Input::Character(' ') => self.board.do_action(),
                Input::Character('q') | Input::KeyDC | Input::Character(ESC) => return false,
                // i => log!(self.main_window, "unknown... {:?}", i),
                _ => (),
            },
        };

        true
    }

    pub fn run(&mut self) {
        use super::super::channel::BackToFrontMessage as Msg;
        loop {
            let msg = self.backend_channel.rx.try_recv();
            match msg {
                Ok(msg) => {
                    match msg {
                        Msg::Log{ msg: s } => log!(self.log, "{}", s),
                        Msg::BoardState(board) => self.board.set_board_state(board),
                    };
                },
                Err(err) => match err {
                    std::sync::mpsc::TryRecvError::Disconnected => { println!("Disconnected"); break; },
                    _ => (),
                },
            };

            // self.main_window.addstr("○●◯◖◗⬤⭗⭕⭘🔴🔵🞉🞊♛♕♔♚👑⛀⛂⛁⛃");
            self.board.draw();
            self.log.borrow_mut().window.draw_box(ACS_VLINE(), ACS_HLINE()); // TODO temp?
            self.log.borrow_mut().window.refresh();
            self.main_window.refresh();

            if !self.process_input() {
                break;
            }
        }
        // println!("DONE");
        endwin();
    }
}

