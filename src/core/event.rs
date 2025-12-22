use derivative::Derivative;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Peer;

/// When ordering events, this is the primary key with the sender ID as the tiebreaker.
/// Since counters are unique per sender, total ordering is quaranteed.
pub type Counter = u64;

/// An event sent between peers.
#[derive(Derivative, Debug, Clone, Serialize, Deserialize)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    // The order of these fields is important for ordering events.
    // Order first by counter, then by sender ID as a tiebreaker.
    pub counter: Counter,
    pub sender_id: Uuid,

    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub event_type: EventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// Share network state with a newly joined peer.
    Welcome { peers: Vec<Peer> },

    /// Request to join the network as a new peer.
    JoinRequest { peer: Peer },

    /// A new peer has joined the network.
    Joined { new_peer: Peer },

    /// A peer has left the network.
    Left { peer_id: Uuid },
}

impl Event {
    pub fn new(sender_id: Uuid, counter: Counter, event_type: EventType) -> Self {
        Event {
            sender_id,
            counter,
            event_type,
        }
    }

    pub fn welcome(sender_id: Uuid, counter: Counter, peers: Vec<Peer>) -> Self {
        Event::new(sender_id, counter, EventType::Welcome { peers })
    }

    pub fn join_request(sender_id: Uuid, counter: Counter, peer: Peer) -> Self {
        Event::new(sender_id, counter, EventType::JoinRequest { peer })
    }

    pub fn joined(sender_id: Uuid, counter: Counter, new_peer: Peer) -> Self {
        Event::new(sender_id, counter, EventType::Joined { new_peer })
    }

    pub fn left(sender_id: Uuid, counter: Counter, peer_id: Uuid) -> Self {
        Event::new(sender_id, counter, EventType::Left { peer_id })
    }
}
