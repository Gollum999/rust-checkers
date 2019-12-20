use super::ai::Ai;
use super::board::{Board, Move, Square, Team};
use super::player::Player;

use crate::args::Args;
use crate::channel::{BackendEndpoint, BackToFrontMessage, FrontToBackMessage};

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
        let msg = self.frontend_channel.rx.recv().unwrap();
        let prefs = match msg {
            FrontToBackMessage::StartGame(prefs) => prefs,
            msg => panic!("Unexpected message from frontend: {:?}", msg),
        };
        let make_player = |team, pref| {
            match pref {
                "Human" => Player::Human{ team: team },
                "CPU"   => Player::Computer{ ai: Ai{ team: team } },
                _ => panic!("Bad player pref: {:?}", pref)
            }
        };
        let players = [
            make_player(Team::Light, prefs.players[0]),
            make_player(Team::Dark,  prefs.players[1]),
        ];
        self.update_frontend();
        let mut player_iter = players.iter().enumerate().cycle();
        while !self.board.game_over() {
            let (player_idx, current_player) = player_iter.next().unwrap();
            log!(self, "Player {}'s turn", player_idx);
            let result = match current_player {
                Player::Human{team} => self.process_human(current_player, *team), // TODO relationship between player/team is awkward
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

    fn process_human(&mut self, player: &Player, team: Team) -> Result<bool, RecvError> {
        self.request_move_from_frontend(team);

        self.handle_move_msg_from_frontend(player, team)
    }

    fn handle_move_msg_from_frontend(&mut self, player: &Player, team: Team) -> Result<bool, RecvError> {
        let msg = self.frontend_channel.rx.recv()?;
        let mv = match msg {
            FrontToBackMessage::Move(mv) => mv,
            FrontToBackMessage::CancelMove => return Ok(true),
            msg => panic!("Unexpected message from frontend: {:?}", msg),
        };

        match self.board.get_piece_at(&mv.from) {
            Some(piece) if piece.team == team => { // TODO let board handle this logic, plus check other validity
                log!(self, "Human ({:?}) taking move: {}", team, mv);
                self.apply_move(&mv);

                // TODO do I need to handle case where player has no valid moves?  (seems like I shouldn't request move in the first place)
                let jumps = self.board.get_valid_jumps_for_piece_at(&mv.to);
                if mv.is_jump() && !jumps.is_empty() {
                    self.request_jump_from_frontend(team, mv.to, jumps);
                    return self.handle_move_msg_from_frontend(player, team);
                }
            },
            _ => panic!("Frontend sent bad move: {}", mv),
        }

        Ok(true)
    }

    fn apply_move(&mut self, mv: &Move) {
        self.board.apply_move(&mv);
        self.update_frontend();
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
            log!(self, "AI ({:?}) taking move: {}", ai.team, mv);
            self.board.apply_move(&mv);
            self.update_frontend();
        }
        Ok(true)
    }

    fn request_move_from_frontend(&self, team: Team) {
        // log!(self, "Requesting move for team {:?}...", team);
        self.frontend_channel.tx.send(BackToFrontMessage::RequestMove(team)).expect("Could not send RequestMove"); // TODO better handling
    }

    fn request_jump_from_frontend(&self, team: Team, square: Square, valid_moves: Vec<Move>) {
        // log!(self, "Requesting jump for team {:?}, square {:?}, one of {:?}...", team, square, valid_moves);
        self.frontend_channel.tx.send(BackToFrontMessage::RequestJump(team, square, valid_moves)).expect("Could not send RequestJump"); // TODO better handling
    }

    fn update_frontend(&self) {
        self.frontend_channel.tx.send(BackToFrontMessage::BoardState(self.board.clone())).expect("Could not send board state"); // TODO better handling
    }
}
