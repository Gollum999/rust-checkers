use std::sync::mpsc;
use super::backend::Board;

pub struct Endpoint {
    pub tx: mpsc::Sender<Message>,
    pub rx: mpsc::Receiver<Message>,
}

pub fn make_two_way_channel() -> (Endpoint, Endpoint) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    (Endpoint{ tx: tx1, rx: rx2 }, Endpoint{ tx: tx2, rx: rx1 })
}

pub enum Message {
    Log{ msg: String },
    BoardState(Board),
}
impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Message::BoardState(_) => write!(f, "Message::BoardState(...)"),
            _                      => write!(f, "{:?}", self),
        }
    }
}
