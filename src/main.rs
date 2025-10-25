mod admin;
mod entities;
mod ip;
mod member;

use crossbeam_channel::unbounded;
use ws::listen;

use entities::{Event, Handler};

fn main() {
    let (events_tx, events_rx) = unbounded::<Event>();
    listen("localhost:5432", |out| Handler);
}
