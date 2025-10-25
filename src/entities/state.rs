use crossbeam_channel::Sender as ChSender;
// use ws::Sender as WsSender;

use super::{Counter, Event, Hierarchy, Message};

pub struct StateManager {
    state: State,
    events_tx: ChSender<Event>,
}

impl StateManager {
    pub fn new(events_tx: ChSender<Event>) -> Self {
        StateManager {
            state: State::Initial,
            events_tx,
        }
    }

    pub fn handle(self, event: Event) -> State {
        todo!()
    }
}

pub enum State {
    Initial,
    Discover(DiscoverState),
    Connect(ConnectState),
    Admin(AdminState),
    Member(MemberState),
    Leaving,
}

pub struct DiscoverState {}

pub struct ConnectState {}

pub struct AdminState {
    /// Keeps messages ordered
    counter: Counter,

    hierarchy: Hierarchy,
    // listeners: Vec<WsSender>,
}

pub struct MemberState {
    /// Kept in sync with admin in case this becomes an admin
    counter: Counter,

    hierarchy: Hierarchy,
}
