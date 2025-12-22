use crossbeam_channel::{Receiver as ChannelRx, Sender as ChannelTx};

use super::{Control, Event, Peer, State};

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

    pub fn submit_ctrl(&self, ctrl: Control) -> anyhow::Result<()> {
        Ok(self.ctrl_tx.send(ctrl)?)
    }

    pub fn step(mut self) -> anyhow::Result<Process> {
        if let Ok(ctrl) = self.ctrl_rx.try_recv() {
            self.state = Self::transition(self.state, ctrl)?;
        }
        Ok(self)
    }

    /// Performs the state transition logic for the process.
    fn transition(state: State, ctrl: Control) -> anyhow::Result<State> {
        log::debug!("Transitioning state: {:?} + {:?}", state, ctrl);
        let new_state = match (state, ctrl) {
            (State::Init, Control::Host) => State::Active {
                neighbours: vec![],
                me: Peer::new("peer"),
                counter: 0,
            },
            (State::Init, Control::Join { ws }) => {
                // TODO: send request
                State::Joining {
                    neighbours: vec![ws],
                    peers: vec![],
                }
            }
            (state, ctrl) => {
                log::warn!("Unhandled state transition: {:?} + {:?}", state, ctrl);
                state
            }
        };
        log::debug!("New state: {:?}", new_state);
        Ok(new_state)
    }
}
