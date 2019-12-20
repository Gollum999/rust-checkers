extern crate pancurses;

use super::cursor_input::CursorInput;

use crate::backend;
use backend::{Board, Player};

use pancurses::{
    A_REVERSE,
    ACS_HLINE, ACS_VLINE,
    curs_set, endwin, initscr, init_pair, Input, noecho, start_color,
};

#[repr(i16)]
pub enum Color {
    Default      = 0,
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

    pub fn draw(&self, window: &mut pancurses::Window) {
        let mid_single = |small: i32, big: i32| -> i32 {
            small + (big - small) / 2
        };
        let mid_rect   = |small: (i32, i32), big: (i32, i32)| -> (i32, i32) {
            (mid_single(small.0, big.0), mid_single(small.1, big.1))
        };

        const SPACING_X: usize = 6;
        const SPACING_Y: usize = 1;
        let menu_height = MENU.len() + SPACING_Y + 1;
        let description_column_width = MENU
            .iter()
            .map(|item| item.description.len())
            .max()
            .unwrap();
        let value_column_width = MENU
            .iter()
            .map(|item| item.values.iter().map(|x| x.chars().count()).max().unwrap() + 4) // account for Unicode chars
            .max()
            .unwrap();
        let menu_width = description_column_width + value_column_width + SPACING_X;
        let menu_size = (menu_height, menu_width);
        let menu_half_size = (menu_height / 2, menu_width / 2);

        let top_left = window.get_beg_yx();
        let bottom_right = window.get_max_yx();
        let center_yx = mid_rect(top_left, bottom_right);
        let offset_yx = (center_yx.0 - menu_half_size.0 as i32, center_yx.1 - menu_half_size.1 as i32);

        for (idx, item) in MENU.iter().enumerate() {
            let selected_value = item.values[self.selections[idx]];
            let value = format!("< {} >", selected_value);
            // let value = format!("< {:^width$} >", item.values[self.selections[idx]], width=);
            window.mvaddstr(offset_yx.0 + idx as i32, offset_yx.1,
                            format!("{desc:<desc_width$}{spacing}{value:>value_width$}",
                                    desc=item.description, desc_width=description_column_width,
                                    spacing=str::repeat(" ", SPACING_X),
                                    value=value, value_width=value_column_width));
            if self.cursor == idx {
                let value_width = selected_value.chars().count() as i32; // account for Unicode chars
                window.mvchgat(
                    offset_yx.0 + idx as i32,
                    offset_yx.1 + menu_width as i32 - value_width - 2,
                    value_width,
                    A_REVERSE,
                    Color::Default as i16,
                );
            }
        }
        window.mvaddstr(0, 0, format!("cursor: {} selections: {:?}", self.cursor, self.selections));
        window.refresh();
    }
}
impl CursorInput for Menu {
    fn move_cursor(&mut self, dir: Input) {
        let mut cursor = self.cursor as i32;
        let mut selection = self.selections[self.cursor] as i32;
        let item = MENU[self.cursor];

        match dir {
            Input::KeyLeft => selection -= 1,
            Input::KeyRight => selection += 1,
            Input::KeyUp => cursor -= 1,
            Input::KeyDown => cursor += 1,
            _ => panic!("Bad dir passed to move_cursor: {:?}", dir),
        }

        let num_values = item.values.len() as i32;
        self.selections[self.cursor] = ((selection + num_values) % num_values) as usize;
        self.cursor = std::cmp::min(std::cmp::max(cursor, 0), (MENU.len() - 1) as i32) as usize;
    }

    fn do_action(&mut self) {
        // match self.process_state() {
        //     Some(new_state) => self.state = new_state,
        //     None => (),
        // }
    }
}
