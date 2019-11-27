use super::ai::Ai;

enum Player {
    Human,
    Computer{ ai: Ai },
}
