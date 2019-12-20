#[macro_use]
mod log;

mod board;
mod curses_frontend;
mod cursor_input;
mod menu;

pub use curses_frontend::CursesFrontend;
pub use menu::Preferences;
