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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Hierarchy(Vec<Peer>);

// impl Hierarchy {
//     pub fn new() -> Self {
//         Hierarchy(Vec::new())
//     }
// }

// pub type Counter = u64;

pub type Counter = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hierarchy(Vec<Peer>);

impl Hierarchy {
    pub fn new() -> Self {
        Self(vec![Peer::get_local()])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    name: String,
    counter: Counter,
    hierarchy: Hierarchy,
}

impl Room {
    pub fn new() -> Self {
        Self {
            name: crate::admin::room::random_room_name(),
            counter: 0,
            hierarchy: Hierarchy::new(),
        }
    }
}
