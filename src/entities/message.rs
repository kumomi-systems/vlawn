use serde::{Deserialize, Serialize};

use super::{Counter, Hierarchy, Peer};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub counter: Counter,
    pub payload: Payload,
}

impl Message {
    pub fn new(payload: Payload, counter: Counter) -> Self {
        Message { payload, counter }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    Text(String),
    // File(String, String),
    JoinReq(Peer),
    JoinNotify(Peer, Hierarchy),
    LeaveNotify(Peer, Hierarchy),
    Sync(Hierarchy),
}
