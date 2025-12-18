use derivative::Derivative;
use uuid::Uuid;

use super::Peer;

/// When ordering events, this is the primary key with the sender ID as the tiebreaker.
/// Since counters are unique per sender, total ordering is quaranteed.
pub type Counter = u64;

/// An event sent between peers.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    // The order of these fields is important for ordering events.
    // Order first by counter, then by sender ID as a tiebreaker.
    counter: Counter,
    sender_id: Uuid,

    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    event_type: EventType,
}

#[derive(Debug, Clone)]
pub enum EventType {
    /// Assigns a new peer ID to a joining peer.
    Welcome(Uuid),
    Join(Peer),
    Leave(Uuid),
}

impl Event {
    pub fn new(sender_id: Uuid, counter: Counter, event_type: EventType) -> Self {
        Event {
            sender_id,
            counter,
            event_type,
        }
    }

    pub fn sender_id(&self) -> Uuid {
        self.sender_id
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn counter(&self) -> Counter {
        self.counter
    }
}
