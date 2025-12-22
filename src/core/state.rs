use ws::Sender;

use super::{Counter, Peer};

#[derive(Debug, Clone)]
pub enum State {
    Init,

    /// Join request sent, waiting for network state.
    Joining(JoiningState),

    /// Actively participating in the session.
    Active(ActiveState),
}

impl State {
    pub fn init() -> Self {
        State::Init
    }

    pub fn active(neighbours: Vec<Sender>, peers: Vec<Peer>, me: Peer, counter: Counter) -> Self {
        State::Active(ActiveState {
            neighbours,
            peers,
            me,
            counter,
        })
    }

    pub fn joining(neighbours: Vec<Sender>, me: Peer) -> Self {
        State::Joining(JoiningState { neighbours, me })
    }
}

impl From<JoiningState> for State {
    fn from(st: JoiningState) -> Self {
        State::Joining(st)
    }
}

impl From<ActiveState> for State {
    fn from(st: ActiveState) -> Self {
        State::Active(st)
    }
}

#[derive(Debug, Clone)]
pub struct JoiningState {
    pub neighbours: Vec<Sender>,
    pub me: Peer,
}

#[derive(Debug, Clone)]
pub struct ActiveState {
    pub neighbours: Vec<Sender>,
    pub peers: Vec<Peer>,
    pub me: Peer,
    pub counter: Counter,
}
