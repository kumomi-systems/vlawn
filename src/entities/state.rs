use super::{Counter, Event, Hierarchy, Message};

pub enum State {
    Initial,
    Discover(DiscoverState),
    Connect(ConnectState),
    Admin(AdminState),
    Member(MemberState),
    Leaving,
}

impl State {
    pub fn handle(self, event: Event) -> State {
        todo!()
    }
}

pub struct DiscoverState {}

pub struct ConnectState {}

pub struct AdminState {
    /// Keeps messages ordered
    counter: Counter,

    hierarchy: Hierarchy,
    // listeners: Vec<Sender<Message>>,
}

pub struct MemberState {
    /// Kept in sync with admin in case this becomes an admin
    counter: Counter,

    hierarchy: Hierarchy,
}
