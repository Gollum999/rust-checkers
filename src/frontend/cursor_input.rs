use pancurses::Input;

pub trait CursorInput {
    type Action;
    fn move_cursor(&mut self, dir: Input);
    fn do_action(&mut self) -> Option<Self::Action>;
}
