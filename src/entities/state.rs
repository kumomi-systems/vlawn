use super::{Counter, Event, Hierarchy, Message};

pub enum State {
    Admin(AdminState),
    Listen(ListenState),
    Discover(DiscoverState),
}

impl State {
    pub fn handle(self, event: Event) -> State {
        todo!()
    }
}

pub struct AdminState {
    /// Keeps messages ordered
    counter: Counter,

    hierarchy: Hierarchy,
    // listeners: Vec<Sender<Message>>,
}

pub struct ListenState {
    /// Kept in sync with admin in case this becomes an admin
    counter: Counter,

    hierarchy: Hierarchy,
}

pub struct DiscoverState {}
