use ws::Sender;

use super::Event;

#[derive(Debug, Clone)]
pub enum Control {
    /// Start hosting a session.
    Host,

    /// Join a session through the given open connection.
    Join(JoinControl),

    /// A neighbouring process has closed its connection.
    Close(CloseControl),

    /// A process has sent an event.
    Event(EventControl),
}

impl Control {
    /// Creates a Join control message.
    pub fn join(ws: Sender) -> Self {
        Control::Join(JoinControl { ws })
    }

    /// Creates a Close control message.
    pub fn close(ws: Sender) -> Self {
        Control::Close(CloseControl { ws })
    }

    /// Creates an Event control message.
    pub fn event(ws: Sender, evt: Event) -> Self {
        Control::Event(EventControl { ws, evt })
    }
}

impl From<JoinControl> for Control {
    fn from(ctrl: JoinControl) -> Self {
        Control::Join(ctrl)
    }
}

impl From<CloseControl> for Control {
    fn from(ctrl: CloseControl) -> Self {
        Control::Close(ctrl)
    }
}

impl From<EventControl> for Control {
    fn from(ctrl: EventControl) -> Self {
        Control::Event(ctrl)
    }
}

#[derive(Debug, Clone)]
pub struct JoinControl {
    pub ws: Sender,
}

#[derive(Debug, Clone)]
pub struct OpenControl {
    pub ws: Sender,
}

#[derive(Debug, Clone)]
pub struct CloseControl {
    pub ws: Sender,
}

#[derive(Debug, Clone)]
pub struct EventControl {
    pub ws: Sender,
    pub evt: Event,
}
