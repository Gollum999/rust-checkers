use super::ai::Ai;
use super::board::{Board, Team};
use super::player::Player;
use super::super::channel; // TODO any way to clean this up?

use std::sync::mpsc::RecvError;
use std::thread;
use std::time::Duration;

pub struct Game {
    frontend_channel: channel::Endpoint,
    board: Board,
    score: [i8; 2],
}

macro_rules! log {
    ( $self:expr, $( $arg:expr ),* ) => {
        // TODO don't panic here
        $self.frontend_channel.tx.send(channel::Message::Log{ msg: format!($($arg),*) }).expect("Failed to log");
    };
}

impl Game {
    pub fn new(frontend_channel: channel::Endpoint) -> Game {
        Game {
            frontend_channel: frontend_channel,
            board: Board::new(),
            score: [0, 0],
        }
    }

    fn game_over(&self) -> bool {
        const PIECES_PER_PLAYER: i8 = 12;
        self.score.iter().any(|score| score >= &PIECES_PER_PLAYER) // TODO why do I have to borrow here?
    }

    pub fn start(&mut self) {
        let players = [
            Player::Computer{ ai: Ai{ team: Team::White } },
            Player::Computer{ ai: Ai{ team: Team::Black } },
        ];
        self.update_frontend();
        let mut player_iter = players.iter().enumerate().cycle();
        while !self.game_over() {
            let (player_idx, current_player) = player_iter.next().unwrap();
            log!(self, "Player {}'s turn", player_idx);
            let result = match current_player {
                Player::Human => self.process_human(current_player),
                Player::Computer{ai} => self.process_ai(ai),
            };
            match result { // TODO clean up
                Err(_) => break, // Frontend closed, channel broken
                Ok(false) => break,
                _ => (),
            };
        }
        log!(self, "Game over!");
    }

    fn process_human(&mut self, player: &Player) -> Result<bool, RecvError> {
        let msg = self.frontend_channel.rx.recv()?;
        match msg {
            // Ok(channel::Message::Move{ msg: s }) => s,  // TODO
            x => log!(self, "Warning: Unhandled message from frontend: {:?}", x), // TODO I think there is a more idiomatic way to write this
        };
        Ok(true)
    }

    fn process_ai(&mut self, ai: &Ai) -> Result<bool, RecvError> {
        // let msg = self.frontend_channel.rx.recv()?;
        let next_moves = ai.get_next_moves(self.board.clone());
        if next_moves.is_empty() {
            return Ok(false); // TODO clean up
        }
        // log!(self, "next moves: {:?}", next_moves);
        for mv in next_moves {
            const AUTO_PLAY: bool = false;
            if AUTO_PLAY {
                thread::sleep(Duration::from_millis(1000));
            } else {
                use std::io;
                use std::io::Read;
                let mut stdin = io::stdin();
                let _ = stdin.read(&mut [0u8]).unwrap();
            }
            log!(self, "Processing AI, next move: {}", mv);
            self.board.apply_move(&mv);
            self.update_frontend();
        }
        Ok(true)
    }

    fn update_frontend(&self) {
        self.frontend_channel.tx.send(channel::Message::BoardState(self.board.get_pieces().clone())).expect("Could not send board state"); // TODO better handling
    }
}
