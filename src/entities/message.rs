use serde::{Deserialize, Serialize};

use super::{Counter, Hierarchy, Peer};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub counter: Counter,
    pub payload: Payload,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    Text(String),
    // File(String, String),
    JoinNotify(Peer, Hierarchy),
    LeaveNotify(Peer, Hierarchy),
    Sync(Hierarchy),
}
