use std::sync::mpsc;
use std::collections::HashMap;
use super::backend::{Piece, Square};

pub struct Endpoint {
    pub tx: mpsc::Sender<Message>,
    pub rx: mpsc::Receiver<Message>,
}

pub fn make_two_way_channel() -> (Endpoint, Endpoint) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    (Endpoint{ tx: tx1, rx: rx2 }, Endpoint{ tx: tx2, rx: rx1 })
}

#[derive(Debug)]
pub enum Message {
    Log{ msg: String },
    BoardState(HashMap<Square, Piece>),
}
