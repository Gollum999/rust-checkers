use super::ai::Ai;
use super::board::Board;
use super::player::Player;
use super::super::channel; // TODO any way to clean this up?

use std::sync::mpsc::RecvError;
use std::thread;
use std::time::Duration;

pub struct Game<'a> {
    frontend_channel: channel::Endpoint,
    board: &'a Board,
    players: [Player<'a>; 2],
    score: [i8; 2],
}

macro_rules! log {
    ( $self:expr, $( $arg:expr ),* ) => {
        // TODO don't panic here
        $self.frontend_channel.tx.send(channel::Message::Log{ msg: format!($($arg),*) }).expect("Failed to log");
    };
}

impl<'a> Game<'a> {
    pub fn new(board: &Board, frontend_channel: channel::Endpoint) -> Game {
        Game {
            frontend_channel: frontend_channel,
            board: board,
            players: [Player::Computer{ ai: Ai{ board } }, Player::Computer{ ai: Ai{ board } }],
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
            let result = match current_player {
                Player::Human => self.process_human(current_player),
                Player::Computer{ai} => self.process_ai(ai),
            };
            match result { // TODO clean up
                Err(_) => break, // Frontend closed, channel broken
                _ => (),
            };
            // TODO if ai/ai, want to be able to interrupt but don't want to block
            log!(self, "Player {}'s turn", player_idx);
        }
        println!("Game thread done");
    }

    fn process_human(&self, player: &Player) -> Result<(), RecvError> {
        let msg = self.frontend_channel.rx.recv()?;
        match msg {
            // Ok(channel::Message::Move{ msg: s }) => s,  // TODO
            x => log!(self, "Warning: Unhandled message from frontend: {:?}", x), // TODO I think there is a more idiomatic way to write this
        };
        Ok(())
    }

    fn process_ai(&self, ai: &Ai) -> Result<(), RecvError> {
        // let msg = self.frontend_channel.rx.recv()?;
        let next_move = ai.get_next_move();
        log!(self, "Processing AI, next move: {}", next_move);
        thread::sleep(Duration::from_millis(1000));
        Ok(())
    }
}
