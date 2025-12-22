use ws::Sender;

use super::{Counter, Peer};

#[derive(Debug, Clone)]
pub enum State {
    Init,
    Joining {
        neighbours: Vec<Sender>,
        peers: Vec<Peer>,
    },
    Active {
        neighbours: Vec<Sender>,
        me: Peer,
        counter: Counter,
    },
}
