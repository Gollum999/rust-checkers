use super::ai::Ai;
use super::board::Team;

pub enum Player {
    Human{ team: Team },
    Computer{ ai: Ai },
}
