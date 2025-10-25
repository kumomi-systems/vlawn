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
        match (&self.state, event) {
            (State::Initial, Event::StartRoom) => self.state = State::Admin(AdminState::new()),
            (State::Initial, Event::JoinSend(addr)) => {
                let endpoint = format!("ws://{addr}:57185");
                println!("join: {endpoint}");
                connect(endpoint, |out| {
                    let msg = Message::new(Payload::JoinReq(self.peer.clone()), 0);
                    let msg_vec = to_allocvec(&msg).unwrap();
                    out.send(msg_vec.as_slice()).unwrap();

                    Handler::new(self.events_tx.clone(), out.connection_id())
                })
                .unwrap();
            }
            (State::Admin(state), Event::Message(msg, con_id)) => match msg.payload {
                Payload::Text(str) => println!("{str}"),
                Payload::JoinReq(peer) => {
                    println!("Join request: {peer:?}")
                }
                _ => todo!(),
            },
            (_, evt) => println!("No transition for ({:?}, {evt:?})", self.state),
        };
    }
}

#[derive(Debug)]
pub enum State {
    Initial,
    Discover(DiscoverState),
    Connect(ConnectState),
    Admin(AdminState),
    Member(MemberState),
    Leaving,
}

#[derive(Debug)]
pub struct DiscoverState {}

#[derive(Debug)]
pub struct ConnectState {}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct MemberState {
    room: Room,
    admin: WsSender,
}
