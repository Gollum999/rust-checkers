extern crate pancurses;

use super::args::{Args, Color, ColorScheme};
use super::board::{BoardView, SQUARE_WIDTH};
use super::log::LogView;

use crate::backend;
use backend::{Board, Player};
use crate::channel::FrontendEndpoint;

use std::cell::RefCell;
use std::rc::Rc;

use pancurses::{
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
    color_scheme: ColorScheme,
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
        use crate::channel::BackToFrontMessage as Msg;
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

