use std::net::IpAddr;

use ws::Sender;

use super::Message;

#[derive(Debug)]
pub enum Event {
    /// A connection to a peer has closed
    Closed(u32),

    Timeout,
    Message(Message, u32),

    Open(Sender),
    JoinSend(IpAddr),

    StartRoom,
}
