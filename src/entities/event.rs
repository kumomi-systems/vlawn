use ws::Sender;

use super::Message;
use super::Peer;

pub enum Event {
    /// A connection to a peer has closed
    Closed(u32),

    Timeout,
    Message(Message, u32),
    Join(Sender),
}
