use std::sync::mpsc;
use super::backend::{Board, Move, Square, Team};

pub struct Endpoint<TxMsg, RxMsg> {
    pub tx: mpsc::Sender<TxMsg>,
    pub rx: mpsc::Receiver<RxMsg>,
}
pub type BackendEndpoint  = Endpoint<BackToFrontMessage, FrontToBackMessage>;
pub type FrontendEndpoint = Endpoint<FrontToBackMessage, BackToFrontMessage>;

pub fn make_two_way_channel() -> (BackendEndpoint, FrontendEndpoint) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    (BackendEndpoint{ tx: tx1, rx: rx2 }, FrontendEndpoint{ tx: tx2, rx: rx1 })
}

pub enum BackToFrontMessage {
    Log{ msg: String },
    BoardState(Board),
    RequestMove(Team),
    RequestJump(Team, Square, Vec<Move>),
}
impl std::fmt::Debug for BackToFrontMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BackToFrontMessage::BoardState(_) => write!(f, "BackToFrontMessage::BoardState(...)"),
            _                                 => write!(f, "{:?}", self),
        }
    }
}
#[derive(Debug)]
pub enum FrontToBackMessage {
    Move(Move),
    CancelMove,
}
