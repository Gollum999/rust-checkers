use rand;
use rand::seq::SliceRandom;
use super::board::Board;
use super::board::Move;
use super::board::Team;

pub struct Ai {
    pub team: Team,
}
impl Ai {
    pub fn get_next_moves(&self, board: Board) -> Vec<Move> {
        let move_sets = Self::_get_valid_move_sets(self.team, board);

        let mut rng = rand::thread_rng();
        match move_sets.choose(&mut rng) {
            Some(move_set) => move_set.clone(), // TODO I think this clone is necessary because I can't move out of a reference to an element of a Vector
            None => Vec::new(), // No valid moves, game over
        }
    }

    fn _get_valid_move_sets(team: Team, board: Board) -> Vec<Vec<Move>> {
        let mut result = Vec::new();
        let moves = board.get_all_valid_moves(team);
        // println!("VALID MOVES: {:?}\r", moves);
        for mv in moves {
            let mut new_board = board.clone();
            new_board.apply_move(&mv);

            if mv.is_jump() {
                // Chain jumps
                // println!("processing jump {}\r", mv);
                result.append(&mut Self::_process_jump(vec![mv], new_board, &mv));
            } else {
                // println!("processing normal move {}\r", mv);
                result.push(vec![mv]);
            }
        }

        result
    }

    fn _process_jump(current_path: Vec<Move>, board: Board, jump: &Move) -> Vec<Vec<Move>> {
        let mut result = vec![current_path.clone()];
        let jumps = board.get_valid_jumps_for_piece_at(&jump.to);
        // println!("filtering... {:?}", jumps);
        for jump in jumps {
            // println!("processing recursive jump {}\r", jump);
            let mut new_board = board.clone();
            new_board.apply_move(&jump);
            // Keep chaining
            let mut next_path = current_path.clone(); // TODO is there a one-liner for this?
            next_path.push(jump);
            result.append(&mut Self::_process_jump(next_path, new_board, &jump));
        }

        result
    }
}
