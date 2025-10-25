use crossbeam_channel::Sender;
use ws::Handler as WsHandler;

use super::Event;

pub struct Handler {
    events_tx: Sender<Event>,
}

impl WsHandler for Handler {}
