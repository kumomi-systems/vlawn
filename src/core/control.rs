use ws::Sender;

use super::Event;

#[derive(Debug, Clone)]
pub enum Control {
    /// Start hosting a session.
    Host,

    /// Join a session through the given open connection.
    Join(Sender),

    /// A neighbouring process has opened a connection.
    Open,

    /// A neighbouring process has closed its connection.
    Close,

    /// A process has sent an event.
    Event(Event),
}
