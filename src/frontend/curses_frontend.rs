extern crate pancurses;

use super::board::{BoardView, SQUARE_WIDTH};
use super::cursor_input::CursorInput;
use super::log::LogView;
use super::menu::{Color, Menu, Preferences};

use crate::args::Args;
use crate::backend;
use backend::{Board, Player};
use crate::channel::FrontendEndpoint;

use std::cell::RefCell;
use std::rc::Rc;

use pancurses::{
    ACS_HLINE, ACS_VLINE,
    COLOR_BLACK, COLOR_RED, COLOR_WHITE,
    curs_set, endwin, initscr, init_pair, Input, noecho, start_color,
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
            message: format!("Error code {}", code), // TODO prettify common error codes... actually I think pancurses may only have a single -1 code for everything
        }
    }
}

pub struct CursesFrontend {
    // args: Args,
    backend_channel: Rc<RefCell<FrontendEndpoint>>,
    window: pancurses::Window,
}
impl CursesFrontend {
    pub fn new(args: Args, backend_channel: FrontendEndpoint) -> CursesFrontend {
        let mut window = initscr();
        window.keypad(true); // Allow control characters
        window.nodelay(true); // Input is non-blocking
        curs_set(0); // Hide cursor
        start_color(); // Enable colors
        noecho(); // Don't echo typed characters

        init_pair(Color::RedOnWhite as i16,   COLOR_RED,   COLOR_WHITE);
        init_pair(Color::WhiteOnRed as i16,   COLOR_WHITE, COLOR_RED);
        init_pair(Color::RedOnBlack as i16,   COLOR_RED,   COLOR_BLACK);
        init_pair(Color::BlackOnRed as i16,   COLOR_BLACK, COLOR_RED);
        init_pair(Color::WhiteOnBlack as i16, COLOR_WHITE, COLOR_BLACK);
        init_pair(Color::BlackOnWhite as i16, COLOR_BLACK, COLOR_WHITE);

        CursesFrontend {
            // args: args.clone(),
            window: window,
            backend_channel: Rc::new(RefCell::new(backend_channel)),
        }
    }

    fn process_input<Actor: CursorInput>(&self, actor: &mut Actor) -> bool {
        let key = self.window.getch();
        const ESC: char = 27 as char;
        match key {
            None => (),
            Some(key) => match key {
                Input::KeyLeft | Input::KeyRight | Input::KeyUp | Input::KeyDown => actor.move_cursor(key),
                Input::KeyEnter | Input::Character('\n') | Input::Character(' ') => actor.do_action(),
                Input::Character('q') | Input::KeyDC | Input::Character(ESC) => return false,
                // i => log!(self.window, "unknown... {:?}", i),
                _ => (),
            },
        };

        true
    }

    fn send_msg(&self, msg: crate::channel::FrontToBackMessage) {
        self.backend_channel.borrow_mut().tx.send(msg).expect("Could not send message"); // TODO better error handling
    }

    pub fn run(&mut self) -> Result<(), WindowError> {
        let preferences = self.handle_menu();
        self.send_msg(crate::channel::FrontToBackMessage::StartGame(preferences.clone()));

        self.main_loop(preferences)
    }

    fn handle_menu(&mut self) -> Preferences {
        let mut menu = Menu::new();
        loop {
            if !self.process_input(&mut menu) {
                break; // TODO immediate exit, not just continue to game
            }
            menu.draw(&mut self.window); // TODO window doesn't need to be mut everywhere

            std::thread::sleep(std::time::Duration::from_millis(10)); // Throttle to keep my laptop from melting
        }

        // TODO
        self.window.clear();
        Preferences {
            ascii: false,
            color_scheme: super::menu::ColorScheme::RedBlack,
        }
    }

    fn main_loop(&mut self, preferences: Preferences) -> Result<(), WindowError> {
        let board_window = self.window.subwin(
            2 + Board::SIZE as i32,
            2 + Board::SIZE as i32 * SQUARE_WIDTH as i32,
            0,
            0,
        )?;
        let log_window = self.window.subwin(
            self.window.get_max_y() - board_window.get_max_y(),
            self.window.get_max_x(),
            board_window.get_max_y(),
            0,
        )?;
        let log = Rc::new(RefCell::new(LogView {
            window: log_window,
        }));
        let mut board = BoardView::new(preferences, board_window, log.clone(), self.backend_channel.clone());

        loop {
            let msg = self.backend_channel.borrow_mut().rx.try_recv();
            match msg {
                Ok(msg) => {
                    use crate::channel::BackToFrontMessage as Msg;
                    match msg {
                        Msg::Log{ msg: s } => log!(log, "{}", s),
                        Msg::BoardState(state) => board.set_board_state(state),
                        Msg::RequestMove(team) => board.start_selecting_piece(team),
                        Msg::RequestJump(team, square, valid_moves) => board.continue_jumping(team, square, valid_moves),
                    };
                },
                Err(err) => match err {
                    std::sync::mpsc::TryRecvError::Disconnected => { println!("Disconnected"); break; },
                    _ => (),
                },
            };

            // self.window.addstr("○●◯◖◗⬤⭗⭕⭘🔴🔵🞉🞊♛♕♔♚👑⛀⛂⛁⛃");
            board.draw();
            log.borrow_mut().window.draw_box(ACS_VLINE(), ACS_HLINE()); // TODO temp?
            log.borrow_mut().window.refresh();
            self.window.refresh();

            if !self.process_input(&mut board) {
                break;
            }

            // TODO I think I can turn this up if I rearrange some things in here
            std::thread::sleep(std::time::Duration::from_millis(10)); // Throttle to keep my laptop from melting
        }
        // println!("DONE");
        endwin();

        Ok(())
    }
}

