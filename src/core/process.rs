use crossbeam_channel::{Receiver as ChannelRx, Sender as ChannelTx};

use super::{Control, Event, EventType, Peer, State};

pub struct Process {
    state: State,
    ctrl_tx: ChannelTx<Control>,
    ctrl_rx: ChannelRx<Control>,
}

impl Process {
    pub fn new() -> Self {
        let (ctrl_tx, ctrl_rx) = crossbeam_channel::unbounded();

        Process {
            state: State::init(),
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
        // TODO: maybe transition to a failed state on error?
        if let Ok(ctrl) = self.ctrl_rx.try_recv() {
            self.state = Self::transition(self.state, ctrl)?;
        }
        Ok(self)
    }

    /// Performs the state transition logic for the process.
    fn transition(state: State, ctrl: Control) -> anyhow::Result<State> {
        log::debug!("transitioning state: {:?} + {:?}", state, ctrl);
        let new_state = match (state, ctrl) {
            (State::Init, Control::Host) => Ok(State::active(vec![], vec![], Peer::new("peer"), 0)),
            (State::Init, Control::Join(ctrl)) => {
                let me = Peer::new("peer");
                let evt = Event::join_request(me.id(), 0, me.clone());
                ctrl.ws.send(postcard::to_allocvec(&evt)?)?;

                // Waiting for network state from peer
                Ok(State::joining(vec![ctrl.ws], me))
            }
            (State::Joining(jn), Control::Event(ctrl)) => {
                // Lamport clocks: initialize counter to received + 1
                let new_counter = ctrl.evt.counter + 1;
                match ctrl.evt.event_type {
                    EventType::Welcome { peers: net_peers } => {
                        Ok(State::active(jn.neighbours, net_peers, jn.me, new_counter))
                    }
                    _ => Err(invalid(&jn.into(), &ctrl.into())),
                }
            }
            (State::Active(mut act), Control::Event(ctrl)) => {
                // Lamport clocks: set counter to max(received, local) + 1
                act.counter = std::cmp::max(&(ctrl.evt).counter, &act.counter) + 1;
                match &ctrl.evt.event_type {
                    EventType::Joined { new_peer } => {
                        assert!(!act.peers.contains(new_peer));
                        act.peers.push(new_peer.clone());
                        for ws in act.neighbours.iter() {
                            if ws == &ctrl.ws {
                                // Don't send the event back to the neighbour who sent it
                                continue;
                            }
                            ws.send(postcard::to_allocvec(&ctrl.evt)?)?;
                        }
                        Ok(State::Active(act))
                    }
                    _ => Err(invalid(&act.into(), &ctrl.into())),
                }
            }
            (state, ctrl) => Err(invalid(&state, &ctrl)),
        }?;
        log::debug!("new state: {:?}", new_state);
        Ok(new_state)
    }
}

fn invalid(state: &State, ctrl: &Control) -> anyhow::Error {
    anyhow::anyhow!("invalid state transition: {:?} + {:?}", state, ctrl)
}
