use rand;
use rand::seq::SliceRandom;
use super::board::Board;
use super::board::Move;

pub struct Ai<'a> {
    pub board: &'a Board,
}
impl<'a> Ai<'a> {
    pub fn get_next_move(&self) -> Move {
        let moves = self.board.get_all_valid_moves();
        let mut rng = rand::thread_rng();
        *moves.choose(&mut rng).unwrap() // TODO why borrow needed?
    }
}
