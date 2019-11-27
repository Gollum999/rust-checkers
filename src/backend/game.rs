use super::board::Board;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
        }
    }

    pub fn start(&self) {
        println!("START");
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}
