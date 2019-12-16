use super::ai::Ai;
use super::board::{Board, Team};
use super::player::Player;
use super::super::args::BackendArgs as Args; // TODO any way to clean this up?
use super::super::channel::{BackendEndpoint, BackToFrontMessage}; // TODO any way to clean this up?

use std::sync::mpsc::RecvError;
use std::thread;
use std::time::{Duration, Instant};

pub struct Game {
    args: Args,
    frontend_channel: BackendEndpoint,
    board: Board,
    // score: [i8; 2],
}

macro_rules! log {
    ( $self:expr, $( $arg:expr ),* ) => {
        // TODO don't panic here
        $self.frontend_channel.tx.send(BackToFrontMessage::Log{ msg: format!($($arg),*) }).expect("Failed to log");
    };
}

impl Game {
    pub fn new(args: Args, frontend_channel: BackendEndpoint) -> Game {
        Game {
            args: args,
            frontend_channel: frontend_channel,
            board: Board::new(),
            // score: [0, 0],
        }
    }

    pub fn start(&mut self) {
        let players = [
            Player::Computer{ ai: Ai{ team: Team::Light } },
            Player::Computer{ ai: Ai{ team: Team::Dark } },
        ];
        self.update_frontend();
        let mut player_iter = players.iter().enumerate().cycle();
        while !self.board.game_over() {
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
        // println!("Game over!");
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
        const AUTO_PLAY: bool = true;
        const MIN_AUTO_PLAY_DELAY: Duration = Duration::from_millis(800);

        let now = Instant::now();
        let next_moves = ai.get_next_moves(self.board.clone());
        let time_spent_in_ai = now.elapsed();
        log!(self, "Processing AI, elapsed: {:?}", time_spent_in_ai);

        if next_moves.is_empty() {
            return Ok(false); // TODO clean up
        }

        // log!(self, "next moves: {:?}", next_moves);
        for (idx, mv) in next_moves.iter().enumerate() {
            if AUTO_PLAY {
                match MIN_AUTO_PLAY_DELAY.checked_sub(time_spent_in_ai) {
                    Some(remaining_delay) => thread::sleep(remaining_delay),
                    None if idx > 0 => thread::sleep(MIN_AUTO_PLAY_DELAY),
                    _ => (),
                };
            } else {
                use std::io;
                use std::io::Read;
                let mut stdin = io::stdin();
                let _ = stdin.read(&mut [0u8]).unwrap();
            }
            log!(self, "AI taking move: {}", mv);
            self.board.apply_move(&mv);
            self.update_frontend();
        }
        Ok(true)
    }

    fn update_frontend(&self) {
        self.frontend_channel.tx.send(BackToFrontMessage::BoardState(self.board.clone())).expect("Could not send board state"); // TODO better handling
    }
}
