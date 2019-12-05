use super::board::{Board, Move, PieceType, Team};

struct Decision {
    pub team: Team,
    pub moves: Vec<Move>,
    pub board_state: Board,
}
impl Decision {
    pub fn score(&self) -> i32 {
        use std::i32;
        static GAME_WIN: i32 = i32::MAX;
        static GAME_LOSS: i32 = i32::MIN;
        static VALUE_MAN: i32 = 10;
        static VALUE_KING: i32 = 20;

        if self.board_state.pieces_alive(self.team) == 0 {
            return GAME_LOSS;
        }
        if self.board_state.pieces_alive(self.team.other()) == 0 {
            return GAME_WIN;
        }

        let score = self.board_state.get_pieces()
                        .values()
                        .fold(0, |score, piece| {
                            let multiplier = if piece.team == self.team { 1 } else { -1 };
                            match piece.piece_type {
                                PieceType::Man  => score + multiplier * VALUE_MAN,
                                PieceType::King => score + multiplier * VALUE_KING,
                            }
                        });
        // println!("Score for {:?}: {} ({:?})", self.team, score, self.moves);
        score
    }
}

pub struct Ai {
    pub team: Team,
}
impl Ai {
    pub fn get_next_moves(&self, board: Board) -> Vec<Move> {
        let mut decisions = Self::_get_possible_decisions(self.team, board);

        decisions.sort_by_cached_key(Decision::score);
        // for d in &decisions {
        //     println!("  decision: {:?} {} ({:?})", d.team, d.score(), d.moves);
        // }
        match decisions.last() {
            Some(decision) => decision.moves.clone(), // TODO I think this clone is necessary because I can't move out of a reference to an element of a Vector
            None => Vec::new(),
        }
    }

    fn _get_possible_decisions(team: Team, board: Board) -> Vec<Decision> {
        let mut result = Vec::new();
        let moves = board.get_all_valid_moves(team);
        // println!("VALID MOVES: {:?}\r", moves);
        for mv in moves {
            let mut new_board = board.clone();
            new_board.apply_move(&mv);

            if mv.is_jump() {
                // Chain jumps
                // println!("processing jump {}\r", mv);
                result.append(&mut Self::_process_jump(team, vec![mv], new_board, &mv));
            } else {
                // println!("processing normal move {}\r", mv);
                result.push(Decision {
                    team: team,
                    moves: vec![mv],
                    board_state: new_board,
                });
            }
        }

        result
    }

    fn _process_jump(team: Team, current_path: Vec<Move>, board: Board, jump: &Move) -> Vec<Decision> {
        let mut result = vec![Decision {
            team: team,
            moves: current_path.clone(),
            board_state: board.clone(),
        }];
        let jumps = board.get_valid_jumps_for_piece_at(&jump.to);
        // println!("filtering... {:?}", jumps);
        for jump in jumps {
            // println!("processing recursive jump {}\r", jump);
            let mut new_board = board.clone();
            new_board.apply_move(&jump);
            // Keep chaining
            let mut next_path = current_path.clone(); // TODO is there a one-liner for this?
            next_path.push(jump);
            result.append(&mut Self::_process_jump(team, next_path, new_board, &jump));
        }

        result
    }
}
