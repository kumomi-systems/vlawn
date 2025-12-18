use std::borrow::Cow;

use crossbeam_channel::Sender as ChannelTx;
use ws::{Handler, Sender};

use super::Control;

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
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        log::info!("Connection opened");
        self.ctrl_tx
            .send(Control::Open)
            .map_err(|e| ws::Error::new(ws::ErrorKind::Internal, Cow::Owned(e.to_string())))
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        log::info!("Connection closed: {:?} - {}", code, reason);
        if let Err(e) = self.ctrl_tx.send(Control::Close) {
            log::error!("Failed to send Close control message: {}", e);
        }
    }
}
