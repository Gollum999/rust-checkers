extern crate pancurses;

pub struct LogView {
    pub window: pancurses::Window,
}

macro_rules! log {
    ( $log_view:expr, $( $arg:expr ),* ) => {{
        let log = $log_view.borrow_mut();
        log.window.mv(1, 1);
        log.window.insertln();
        log.window.addstr(format!($($arg),*));
    }};
}
