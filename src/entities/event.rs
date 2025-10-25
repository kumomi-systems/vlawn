use std::net::IpAddr;

use ws::Sender;

use super::Message;

#[derive(Debug, Clone)]
pub enum Event {
    /// A connection to a peer has closed
    Closed(u32),

    Message(Message, u32),

    Open(Sender),
    JoinSend(IpAddr),

    StartRoom,
}
