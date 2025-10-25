use std::collections::HashMap;

use crossbeam_channel::Sender as ChSender;
use postcard::to_allocvec;
use ws::{connect, Sender as WsSender};

use crate::entities::{peer, ForwardPayload};

use super::{Event, Handler, Message, Payload, Peer, Room};

pub struct StateManager {
    state: State,
    peer: Peer,
    events_tx: ChSender<Event>,
    history: Vec<(Peer, ForwardPayload)>,
}

impl StateManager {
    pub fn new(events_tx: ChSender<Event>) -> Self {
        StateManager {
            state: State::Initial,
            peer: Peer::get_local(),
            events_tx,
            history: Vec::new(),
        }
    }

    pub fn handle(&mut self, event: Event) {
        match (&mut self.state, event) {
            (State::Initial, Event::StartRoom) => self.state = State::Admin(AdminState::new()),
            (State::Initial, Event::JoinSend(addr)) => {
                let msg = Message::new(Payload::JoinReq(self.peer.clone()));
                let msg_vec = to_allocvec(&msg).unwrap();
                let events_tx = self.events_tx.clone();

                std::thread::Builder::new()
                    .name("connect".into())
                    .spawn(move || {
                        connect(format!("ws://{addr}:57185"), |out| {
                            events_tx.send(Event::Open(out.clone())).unwrap();
                            out.send(msg_vec.clone()).unwrap();

                            Handler::new(events_tx.clone(), out.connection_id())
                        })
                        .unwrap();
                    })
                    .unwrap();
            }
            (State::Initial, Event::Open(sender)) => {
                self.state = State::Connect(ConnectState { admin: sender });
            }
            (State::Connect(state), Event::Message(msg, _con_id)) => match msg.payload {
                Payload::Sync(room) => {
                    self.state = State::Member(MemberState {
                        room,
                        admin: state.admin.clone(),
                    });
                    log::info!("Joined room!")
                }
                _ => panic!(),
            },
            (State::Admin(state), Event::Message(msg, con_id)) => match &msg.payload {
                Payload::JoinReq(peer) => {
                    state.room.hierarchy.push(peer.clone());
                    let msg = Message::new(Payload::Sync(state.room.clone()));
                    let msg_vec = to_allocvec(&msg).unwrap();
                    state
                        .clients
                        .iter()
                        .find(|t| t.connection_id() == con_id)
                        .unwrap()
                        .send(msg_vec)
                        .unwrap();
                    state.peers.insert(con_id, peer.clone());
                }
                Payload::Forward(peer, payload) => {
                    self.history.push((peer.clone(), payload.clone()));
                    let msg_vec = to_allocvec(&(msg.clone())).unwrap();
                    state.clients.iter().for_each(|c| {
                        c.send(msg_vec.clone()).unwrap();
                    });
                }
                _ => todo!(),
            },
            (State::Admin(state), Event::Open(sender)) => {
                state.clients.push(sender);
            }
            (State::Admin(state), Event::Closed(con_id)) => {
                let closed_peer = state.peers.remove(&con_id).unwrap();
                state.room.hierarchy.remove(&closed_peer);
                state.clients.retain(|c| c.connection_id() != con_id);

                let msg = Message::new(Payload::Sync(state.room.clone()));
                let msg_vec = to_allocvec(&msg).unwrap();
                state
                    .clients
                    .iter()
                    .for_each(|s| s.send(msg_vec.clone()).unwrap());
            }
            (State::Member(state), Event::Closed(con_id)) => {
                if state.admin.connection_id() == con_id {
                    log::info!("{:?}", state.room.hierarchy);

                    let new_admin = state.room.hierarchy.next_leader().unwrap();
                    let endpoint = format!("ws://{}:57185", new_admin.addr());

                    if *new_admin == self.peer {
                        log::info!("Promoting self to admin");
                        self.state = State::Admin(AdminState {
                            room: state.room.clone(),
                            clients: Vec::new(),
                            peers: HashMap::new(),
                        })
                    } else {
                        log::info!("Connecting to new admin @ {}", new_admin.addr());
                        connect(endpoint, |out| {
                            let msg = Message::new(Payload::JoinReq(self.peer.clone()));
                            let msg_vec = to_allocvec(&msg).unwrap();
                            out.send(msg_vec.as_slice()).unwrap();

                            Handler::new(self.events_tx.clone(), out.connection_id())
                        })
                        .unwrap();
                    }
                }
            }
            (State::Member(state), Event::Message(msg, _con_id)) => match msg.payload {
                Payload::Sync(room) => {
                    log::info!("resyncing state...");
                    state.room = room
                }
                Payload::Forward(peer, payload) => self.history.push((peer, payload)),
                _ => panic!(),
            },
            (_, evt) => log::error!("No transition for ({:?}, {evt:?})", self.state),
        };
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Initial,
    Discover(DiscoverState),
    Connect(ConnectState),
    Admin(AdminState),
    Member(MemberState),
    Leaving,
}

#[derive(Debug, Clone)]
pub struct DiscoverState {}

#[derive(Debug, Clone)]
pub struct ConnectState {
    admin: WsSender,
}

#[derive(Debug, Clone)]
pub struct AdminState {
    room: Room,
    clients: Vec<WsSender>,
    peers: HashMap<u32, Peer>,
}

impl AdminState {
    pub fn new() -> Self {
        AdminState {
            room: Room::new(),
            clients: Vec::new(),
            peers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberState {
    room: Room,
    admin: WsSender,
}
