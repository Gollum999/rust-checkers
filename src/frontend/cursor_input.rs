use pancurses::Input;

pub trait CursorInput {
    fn move_cursor(&mut self, dir: Input);
    fn do_action(&mut self);
}
