use crossbeam_channel::Sender;
use postcard::from_bytes;
use ws::{Handler as WsHandler, Message as WsMessage, Result};

use super::{Event, Message};

pub struct Handler {
    events_tx: Sender<Event>,
    connection_id: u32,
}

impl Handler {
    pub fn new(events_tx: Sender<Event>, connection_id: u32) -> Self {
        Handler {
            events_tx,
            connection_id,
        }
    }
}

impl WsHandler for Handler {
    fn on_message(&mut self, ws_msg: WsMessage) -> Result<()> {
        let WsMessage::Binary(bin) = ws_msg else {
            unimplemented!("Expected binary message");
        };
        let msg: Message = from_bytes(&bin).unwrap();

        self.events_tx
            .send(Event::Message(msg, self.connection_id))
            .unwrap();
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, reason: &str) {
        self.events_tx
            .send(Event::Closed(self.connection_id))
            .unwrap();
        println!("closed peer {:?}: {reason}", self.connection_id);
    }
}
