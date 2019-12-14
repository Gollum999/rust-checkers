use super::board::{Board, Move, PieceType, Team};

struct Decision {
    pub team: Team,
    pub moves: Vec<Move>,
    pub board_state: Board,
    pub score: Option<i32>,
}
impl Decision {
    pub fn cached_score_board_state(&mut self) -> i32 {
        match self.score {
            Some(score) => score,
            None => {
                self.score = Some(self.score_board_state());
                // println!("Score for {:?}: {} ({:?})", self.team, score, self.moves);
                self.score.unwrap()
            },
        }
    }

    pub fn score_board_state(&self) -> i32 {
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

        self.board_state.get_pieces()
            .values()
            .fold(0, |score, piece| {
                let multiplier = if piece.team == self.team { 1 } else { -1 };
                match piece.piece_type {
                    PieceType::Man  => score + multiplier * VALUE_MAN,
                    PieceType::King => score + multiplier * VALUE_KING,
                }
            })
    }

    pub fn score_recursive(&mut self, depth: usize, is_max_player: bool) -> i32 {
        if self.score.is_some() {
            return self.score.unwrap();
        }

        // println!("{:width$}score_recursive: depth: {}, team: {:?}, max player: {}", "", depth, self.team, is_max_player, width=5-depth);
        if depth == 0 {
            let score = self.cached_score_board_state();
            // println!("{:width$}score_recursive: hit max depth, returning {}", "", score, width=5-depth);
            return score;
        }
        let mut enemy_decisions = Ai::_get_possible_decisions(self.team.other(), self.board_state.clone());
        if enemy_decisions.is_empty() {
            let score = self.cached_score_board_state();
            // println!("{:width$}score_recursive: enemy has no moves, returning {}", "", score, width=5-depth);
            return score;
        }
        for d in &mut enemy_decisions {
            // println!("{:width$}score_recursive scoring: {:?} {:?}", "", d.team, d.moves, width=5-depth);
            d.score_recursive(depth - 1, !is_max_player);
        }

        let compare = match is_max_player {
            true  => std::iter::Iterator::max_by_key,
            false => std::iter::Iterator::min_by_key,
        };
        self.score = compare(enemy_decisions.iter(), |d: &&Decision| d.score).unwrap().score; // TODO why is && needed?

        // println!("{:width$}score_recursive: depth: {}, team: {:?}, max player: {}, returning {}", "", depth, self.team, is_max_player, self.score.unwrap(), width=5-depth);
        self.score.unwrap()
    }
}

pub struct Ai {
    pub team: Team,
}
impl Ai {
    pub fn get_next_moves(&self, board: Board) -> Vec<Move> {
        // println!("----------------------------------------");
        const MAX_DEPTH: usize = 4;
        let depth = 0;
        let team = self.team;
        let mut my_decisions = Self::_get_possible_decisions(self.team, board);
        // my_decisions.sort_by_cached_key(|d| {
        //     d.score_recursive(MAX_DEPTH, true)
        // }); // TODO If I handle caching myself, sort_by_key might be faster
        for d in &mut my_decisions {
            // println!("ROOT scoring: {:?} {:?}", d.team, d.moves);
            d.score_recursive(MAX_DEPTH, true);
        }
        my_decisions.sort_by_key(|d| d.score);

        let dec = my_decisions.last().unwrap();
        // println!("FINAL SCORE: {} ({:?})", dec.score.unwrap(), dec.moves);

        match my_decisions.last() {
            Some(decision) => decision.moves.clone(),
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
                    score: None,
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
            score: None,
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
