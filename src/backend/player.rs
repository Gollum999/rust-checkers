use super::ai::Ai;

enum Player {
    HUMAN,
    COMPUTER{ ai: Ai },
}
