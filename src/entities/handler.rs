use crossbeam_channel::Sender;
use ws::{Error, Handler as WsHandler, Message as WsMessage, Result};

use super::{Event, Peer};

pub struct Handler {
    peer: Peer,
    events_tx: Sender<Event>,
}

impl WsHandler for Handler {
    // fn on_message(&mut self, ws_msg: WsMessage) -> Result<()> {
    //     let WsMessage::Binary(bin) = ws_msg else {

    //     }

    //     self.events_tx.send(Event::Message(msg, self.peer.clone()));
    //     Some(())
    // }
}
