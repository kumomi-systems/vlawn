use super::{Counter, Peer};

#[derive(Debug, Clone)]
pub enum State {
    Init,
    Active { peer: Peer, counter: Counter },
}
