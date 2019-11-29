use rand;
use rand::seq::SliceRandom;
use super::board::Board;
use super::board::Move;
use super::board::Team;

pub struct Ai {
    pub team: Team,
}
impl Ai {
    pub fn get_next_move(&self, board: &Board) -> Move {
        let moves = board.get_all_valid_moves(self.team);
        // println!("VALID MOVES: {:?}\r", moves);
        let mut rng = rand::thread_rng();
        *moves.choose(&mut rng).unwrap() // TODO why borrow needed?
    }
}
