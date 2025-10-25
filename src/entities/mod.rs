mod event;
mod message;
mod peer;
mod state;

use serde::{Deserialize, Serialize};

use event::*;
use message::*;
use peer::*;
use state::*;

#[derive(Serialize, Deserialize)]
pub struct Hierarchy(Vec<Peer>);

pub type Counter = u64;
