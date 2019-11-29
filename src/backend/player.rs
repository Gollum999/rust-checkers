use super::ai::Ai;

pub enum Player<'a> {
    Human,
    Computer{ ai: Ai<'a> },
}
