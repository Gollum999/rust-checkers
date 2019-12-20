extern crate pancurses;

use crate::backend;
use backend::{Board, Player};

use pancurses::{
    ACS_HLINE, ACS_VLINE,
    curs_set, endwin, initscr, init_pair, Input, noecho, start_color,
};

#[repr(i16)]
pub enum Color {
    RedOnWhite   = 1,
    WhiteOnRed   = 2,
    RedOnBlack   = 3,
    BlackOnRed   = 4,
    WhiteOnBlack = 5,
    BlackOnWhite = 6,
}

arg_enum! {
    #[derive(Clone, Debug)]
    pub enum ColorScheme {
        WhiteRed,
        RedBlack,
        WhiteBlack,
    }
}

#[derive(Clone, Debug)]
pub struct Preferences {
    pub ascii: bool,
    pub color_scheme: ColorScheme,
}

// struct MenuItem{description: &'static str, values: &'static [&'static str], default: &'static str}
struct MenuItem{
    description: &'static str,
    values: &'static [&'static str],
    default: usize,
}
// const MENU: &'static [&'static MenuItem] = &[
const MENU: &[&MenuItem] = &[
    // TODO CPU difficulty
    &MenuItem{
        description: "Player 1",
        values:      &["Human", "CPU"],
        default:     0,
    },
    &MenuItem{
        description: "Player 2",
        values:      &["Human", "CPU"],
        default:     1,
    },
    &MenuItem{
        description: "Color Scheme",
        values:      &["Red/Black", "White/Red", "White/Black"],
        default:     0,
    },
    &MenuItem{
        description: "Fancy Icons",
        values:      &["ON (⛂⛃⛀⛁)", "OFF (O@=#)"],
        default:     0,
    },
];

pub struct Menu {
    cursor: usize,
    selections: [usize; MENU.len()],
}
impl Menu {
    pub fn new() -> Menu {
        Menu {
            // TODO probably need an intermediate representation before constructing actual values
            cursor: 0, // TODO any way to use iterators here?
            selections: [0; MENU.len()],
        }
    }

    pub fn process_input(&self) {
        // let key = self.main_window.getch();
        // match key {
        //     KEY_LEFT | KEY_RIGHT => cycle_option(key),
        //     KEY_UP => selection -= 1,
        //     KEY_DOWN => selection += 1,
        //     KEY_ENTER | KEY_SPACE if selection == 5 => {
        //         let preferences = Preferences {
        //             players: [
        //             ],
        //         };
        //         start_game(preferences);
        //     },
        //     _ => (),
        // }
    }

    pub fn draw(&self) {

    }
}
