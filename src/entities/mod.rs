mod event;
mod handler;
mod message;
mod peer;
mod state;

pub use event::*;
pub use handler::*;
pub use message::*;
pub use peer::*;
pub use state::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hierarchy(Vec<Peer>);

impl Hierarchy {
    pub fn new() -> Self {
        Self(vec![Peer::get_local()])
    }

    pub fn push(&mut self, peer: Peer) {
        self.0.push(peer);
    }

    pub fn next_leader(&mut self) -> Option<&Peer> {
        self.0.remove(0);
        self.0.first()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    name: String,
    hierarchy: Hierarchy,
}

impl Room {
    pub fn new() -> Self {
        Self {
            name: crate::admin::room::random_room_name(),
            hierarchy: Hierarchy::new(),
        }
    }
}
