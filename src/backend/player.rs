use super::ai::Ai;

pub enum Player {
    Human,
    Computer{ ai: Ai },
}
