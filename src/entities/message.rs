use serde::{Deserialize, Serialize};

use super::{Peer, Room};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub payload: Payload,
}

impl Message {
    pub fn new(payload: Payload) -> Self {
        Message { payload }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Payload {
    Forward(Peer, ForwardPayload),
    JoinReq(Peer),
    Sync(Room),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ForwardPayload {
    Text(String),
    Notification(String),
}
