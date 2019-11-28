use super::ai::Ai;
use super::board::Board;
use super::player::Player;

pub struct Game {
    board: Board,
    players: [Player; 2],
    score: [i8; 2],
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            players: [Player::Computer{ ai: Ai{} }, Player::Computer{ ai: Ai{} }],
            score: [0, 0],
        }
    }

    fn game_over(&self) -> bool {
        const PIECES_PER_PLAYER: i8 = 12;
        self.score.iter().any(|score| score >= &PIECES_PER_PLAYER) // TODO why do I have to borrow here?
    }

    pub fn start(&self) {
        println!("START");
        // let mut next_player = &self.players[0];
        // while !self.game_over() {
        //     println!("running..."); // TODO
        //     next_player = &self.players[1]; // TODO repeat, custom iter?
        // }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}
