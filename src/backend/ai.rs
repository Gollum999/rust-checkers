use rand;
use rand::seq::{IteratorRandom, SliceRandom};
use super::board::Board;
use super::board::Move;
use super::board::Team;

pub struct Ai {
    pub team: Team,
}
impl Ai {
    pub fn get_next_moves(&self, mut board: Board) -> Vec<Move> {
        let mut result = Vec::new();
        let moves = board.get_all_valid_moves(self.team);
        // println!("VALID MOVES: {:?}\r", moves);
        let mut rng = rand::thread_rng();
        let mut mv = match moves.choose(&mut rng) {
            Some(mv) => *mv,
            None => return result, // No valid moves, game over
        };
        board.apply_move(&mv);
        result.push(mv);
        while mv.is_jump() {
            // println!("Move was jump, chaining...");
            // Chain jumps
            let moves = match board.get_valid_moves_for_piece_at(&mv.to) {
                Ok(moves) => moves,
                Err(_) => break,
            };
            // println!("filtering... {:?}", moves);
            let filtered = moves.iter().filter(|mv| mv.is_jump());
            mv = match filtered.choose(&mut rng) {
                Some(mv) => *mv,
                None => break,
            };
            // println!("pushing...");
            board.apply_move(&mv);
            result.push(mv);
        }

        result
    }
}
