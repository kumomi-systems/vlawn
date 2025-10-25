use super::Message;
use super::Peer;

pub enum Event {
    /// A connection to a peer has closed
    Closed(Peer),
    Timeout,
    Message(Message, Peer),
}
