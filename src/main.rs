mod admin;
mod entities;
mod ip;
mod member;

use crossbeam_channel::unbounded;
use ws::listen;

use entities::{Event, Handler, StateManager};

fn main() {
    let (events_tx, events_rx) = unbounded::<Event>();
    let manager = StateManager::new(events_tx.clone());

    listen("localhost:5432", |out| {
        let connection_id = out.connection_id();
        events_tx
            .send(Event::Join(out))
            .expect("failed to send join event");
        Handler::new(events_tx.clone(), connection_id)
    })
    .expect("listener failed");
}
