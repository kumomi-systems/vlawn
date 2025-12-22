use ws::Sender;

use super::Event;

#[derive(Debug, Clone)]
pub enum Control {
    /// Start hosting a session.
    Host,

    /// Join a session through the given open connection.
    Join { ws: Sender },

    /// A neighbouring process has opened a connection.
    Open { ws: Sender },

    /// A neighbouring process has closed its connection.
    Close { ws: Sender },

    /// A process has sent an event.
    Event(Event),
}
