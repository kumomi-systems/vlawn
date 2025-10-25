use crossbeam_channel::Sender as ChSender;
use postcard::to_allocvec;
use ws::{connect, Sender as WsSender};

use super::{Event, Handler, Message, Payload, Peer, Room};

pub struct StateManager {
    state: State,
    peer: Peer,
    events_tx: ChSender<Event>,
}

impl StateManager {
    pub fn new(events_tx: ChSender<Event>) -> Self {
        StateManager {
            state: State::Initial,
            peer: Peer::get_local(),
            events_tx,
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
            (State::Initial, Event::Open(sender)) => {
                self.state = State::Connect(ConnectState { admin: sender });
            }
            (State::Admin(state), Event::Message(msg, con_id)) => match msg.payload {
                Payload::Text(str) => println!("{str}"),
                Payload::JoinReq(peer) => {
                    state.room.hierarchy.push(peer);
                    let msg = Message::new(Payload::Sync(state.room.clone()));
                    let msg_vec = to_allocvec(&msg).unwrap();
                    state
                        .clients
                        .iter()
                        .find(|t| t.connection_id() == con_id)
                        .unwrap()
                        .send(msg_vec)
                        .unwrap()
                }
                _ => todo!(),
            },
            (State::Admin(state), Event::Open(sender)) => {
                state.clients.push(sender);
            }
            (State::Admin(state), Event::Closed(con_id)) => {
                state.clients.retain(|t| t.connection_id() != con_id)
            }
            (State::Member(state), Event::Closed(con_id)) => {
                if state.admin.connection_id() == con_id {
                    let new_admin = state.room.hierarchy.next_leader().unwrap();
                    let endpoint = format!("ws://{}:57185", new_admin.addr());
                    if *new_admin == self.peer {
                        log::info!("Promoting self to admin");
                        self.state = State::Admin(AdminState {
                            room: state.room.clone(),
                            clients: Vec::new(),
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
}

impl AdminState {
    pub fn new() -> Self {
        AdminState {
            room: Room::new(),
            clients: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberState {
    room: Room,
    admin: WsSender,
}
