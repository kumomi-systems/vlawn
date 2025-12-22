use crossbeam_channel::Sender as ChannelTx;
use postcard::from_bytes;
use ws::{Error as WsError, Handler, Sender};

use super::{Control, Event};

/// Handles the connection for a neighbouring process.
/// Sends control messages to the process based on connection events.
pub struct Endpoint {
    ws_sender: Sender,
    ctrl_tx: ChannelTx<Control>,
}

impl Endpoint {
    pub fn new(ws_sender: Sender, ctrl_tx: ChannelTx<Control>) -> Self {
        Endpoint { ws_sender, ctrl_tx }
    }
}

impl Handler for Endpoint {
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        log::info!("connection closed: {:?} - {}", code, reason);
        let ctrl = Control::close(self.ws_sender.clone());
        if let Err(e) = self.ctrl_tx.send(ctrl) {
            log::error!("failed to send Close control message: {}", e);
        }
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let data = msg.into_data();
        let evt: Event = from_bytes(&data).map_err(|e| WsError::from(Box::new(e)))?;
        let ctrl = Control::event(self.ws_sender.clone(), evt);
        self.ctrl_tx
            .send(ctrl)
            .map_err(|e| WsError::from(Box::new(e)))
    }
}
