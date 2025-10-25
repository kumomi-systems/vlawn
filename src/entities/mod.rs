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

#[derive(Serialize, Deserialize)]
pub struct Hierarchy(Vec<Peer>);

pub type Counter = u64;
