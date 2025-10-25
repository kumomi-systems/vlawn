use serde::{Deserialize, Serialize};

use super::{Hierarchy, Peer, Room};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub payload: Payload,
}

impl Message {
    pub fn new(payload: Payload) -> Self {
        Message { payload }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    Text(String),
    JoinReq(Peer),
    JoinNotify(Peer, Hierarchy),
    LeaveNotify(Peer, Hierarchy),
    Sync(Room),
}
