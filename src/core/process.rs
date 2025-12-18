use crossbeam_channel::{Receiver as ChannelRx, Sender as ChannelTx};

use super::{Control, Peer, State};

pub struct Process {
    state: State,
    ctrl_tx: ChannelTx<Control>,
    ctrl_rx: ChannelRx<Control>,
}

impl Process {
    pub fn new() -> Self {
        let (ctrl_tx, ctrl_rx) = crossbeam_channel::unbounded();

        Process {
            state: State::Init,
            ctrl_tx,
            ctrl_rx,
        }
    }

    /// Returns a send handle to the process' queue of control messages.
    pub fn ctrl_tx(&self) -> ChannelTx<Control> {
        self.ctrl_tx.clone()
    }

    pub fn submit_ctrl(&self, ctrl: Control) {
        self.ctrl_tx.send(ctrl).unwrap();
    }

    pub fn step(&mut self) {
        if let Ok(ctrl) = self.ctrl_rx.try_recv() {
            self.state = Self::transition(self.state.clone(), ctrl);
        }
    }

    /// Performs the state transition logic for the process.
    fn transition(state: State, ctrl: Control) -> State {
        log::info!("Transitioning state: {:?} + {:?}", state, ctrl);
        match (&state, &ctrl) {
            (State::Init, Control::Host) => State::Active {
                peer: Peer::new("peer"),
                counter: 0,
            },
            _ => {
                log::warn!("Unhandled state transition: {:?} + {:?}", state, ctrl);
                state
            }
        }
    }
}
