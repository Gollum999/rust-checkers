use super::ai::Ai;
use super::board::Board;
use super::player::Player;

use super::super::channel; // TODO any way to clean this up?

pub struct Game {
    frontend_channel: channel::Endpoint,
    board: Board,
    players: [Player; 2],
    score: [i8; 2],
}

impl Game {
    pub fn new(frontend_channel: channel::Endpoint) -> Game {
        Game {
            frontend_channel: frontend_channel,
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
        let mut player_iter = self.players.iter().enumerate().cycle();
        while !self.game_over() {
            let (player_idx, current_player) = player_iter.next().unwrap();
            let msg = self.frontend_channel.rx.recv();
            let s = match msg {
                Ok(channel::Message::Log{ msg: s }) => s,
                _ => break, // Frontend closed, channel severed
            };
            println!("BACKEND RECVD {}", s);
            self.frontend_channel.tx.send(channel::Message::Log{msg: format!("Player {}'s turn", player_idx)});
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}
