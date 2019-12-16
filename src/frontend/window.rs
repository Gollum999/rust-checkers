extern crate pancurses;

use super::super::args; // TODO any way to clean this up?
use args::FrontendArgs as Args;
use super::super::backend;
use super::super::channel::FrontendEndpoint; // TODO any way to clean this up?
use backend::{Board, Piece, PieceType, Player, Square, Team}; // TODO this is a bit messy too

use pancurses::{
    ACS_HLINE, ACS_VLINE,
    COLOR_BLACK, COLOR_RED, COLOR_WHITE,
    endwin, initscr, init_pair, Input, noecho, start_color,
};

macro_rules! log {
    ( $window:expr, $( $arg:expr ),* ) => {{
        let old_pos = Point{ x: $window.get_cur_x(), y: $window.get_cur_y() };
        const LOG_POS: Point = Point{ x: 1, y: 11 };
        $window.mv(LOG_POS.y, LOG_POS.x);
        $window.insertln();
        $window.addstr(format!($($arg),*));
        $window.mv(old_pos.y, old_pos.x);
    }};
}

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

enum State {
    Waiting,
    ChoosingPiece,
    ChoosingMove(Piece),
}

const SQUARE_WIDTH: usize = 3;

struct BoardView {
    args: Args,
    window: pancurses::Window,
    cursor: Square,
    state: State,
}
impl BoardView {
    fn new(args: Args, window: pancurses::Window) -> BoardView {
        let board = BoardView {
            args: args,
            window: window,
            cursor: Square{ x: 0, y: 0 },
            state: State::Waiting,
        };
        board.window.draw_box(ACS_VLINE(), ACS_HLINE());

        board
    }

    // TODO if selecting move, limit to valid moves?
    fn move_cursor(&mut self, dir: Input) {
        // println!("move_cursor {:?}", dir);
        match dir {
            Input::KeyLeft => self.cursor.x -= 2,
            Input::KeyRight => self.cursor.x += 2,
            Input::KeyUp => {
                if self.cursor.x % 2 == 0 {
                    self.cursor.x -=1;
                } else {
                    self.cursor.x += 1;
                }
                self.cursor.y -= 1;
            },
            Input::KeyDown => {
                if self.cursor.x % 2 == 0 {
                    self.cursor.x +=1;
                } else {
                    self.cursor.x -= 1;
                }
                self.cursor.y += 1;
            },
            _ => panic!("Bad dir passed to move_cursor: {:?}", dir),
        }
        self.cursor.x = self.cursor.x % Board::SIZE;
        self.cursor.y = self.cursor.y % Board::SIZE;
    }

    fn do_action(&mut self) {
        // println!("do_action");
        match self.state {
            State::Waiting => (),
            State::ChoosingPiece => {
                log!(self.window, "choosing piece.."); // TODO
                // let piece = self.board.get_piece_at(self.cursor);
                // self.state = State::ChoosingMove(piece);
            },
            State::ChoosingMove(piece) => {
                log!(self.window, "choosing move.."); // TODO
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

    fn draw(&mut self, board: Board) {
        let pieces = board.get_pieces();
        for y in 0..Board::SIZE {
            for x in 0..Board::SIZE {
                let c = Self::get_piece_glyph(pieces.get(&Square{x, y}), self.args.ascii);
                let colors = match self.args.color_scheme {
                    args::ColorScheme::WhiteRed   => [Color::WhiteOnRed as i16,   Color::RedOnWhite as i16],
                    args::ColorScheme::RedBlack   => [Color::RedOnBlack as i16,   Color::BlackOnRed as i16],
                    args::ColorScheme::WhiteBlack => [Color::WhiteOnBlack as i16, Color::BlackOnWhite as i16],
                };
                self.window.color_set(colors[((x + y + 1) % 2) as usize]);
                self.window.mvaddstr(
                    (y + 1) as i32,
                    (x * SQUARE_WIDTH as i8 + 1) as i32,
                    format!("{char:^width$}", char=c, width=SQUARE_WIDTH),
                );
            }
        }

        self.window.clearok(true); // This gets rid of the wide-char artifacts, but not the most efficient
        self.window.refresh();
    }
}

pub struct Window {
    args: Args,

    backend_channel: FrontendEndpoint,
    main_window: pancurses::Window,
    board: BoardView,
}
impl Window {
    pub fn new(args: Args, backend_channel: FrontendEndpoint) -> Result<Window, WindowError> {
        let main_window = initscr();
        let sub_window = main_window.subwin(2 + Board::SIZE as i32, 2 + Board::SIZE as i32 * SQUARE_WIDTH as i32, 0, 0)?;
        let w = Window {
            args: args.clone(),
            backend_channel: backend_channel,
            main_window: main_window,
            board: BoardView::new(args, sub_window),
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
        loop {
            let msg = self.backend_channel.rx.recv().unwrap();
            use super::super::channel::BackToFrontMessage as Msg;
            match msg {
                Msg::Log{ msg: s } => log!(self.main_window, "{}", s),
                Msg::BoardState(board) => self.board.draw(board),
            };

            // self.main_window.addstr("○●◯◖◗⬤⭗⭕⭘🔴🔵🞉🞊♛♕♔♚👑⛀⛂⛁⛃");
            self.main_window.refresh();

            if !self.process_input() {
                break;
            }
        }
        // println!("DONE");
        endwin();
    }
}

