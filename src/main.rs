mod admin;
mod entities;
mod ip;
mod member;

use std::{net::IpAddr, str::FromStr};

use crossbeam_channel::unbounded;
use ws::listen;

use entities::{Event, Handler, StateManager};

fn main() {
    let (events_tx, events_rx) = unbounded::<Event>();
    let mut manager = StateManager::new(events_tx.clone());

    let events_tx_clone = events_tx.clone();

    std::thread::Builder::new()
        .name("websocket server".into())
        .spawn(move || {
            listen("localhost:5432", |out| {
                let connection_id = out.connection_id();
                events_tx.send(Event::JoinRecv(out)).unwrap();
                Handler::new(events_tx.clone(), connection_id)
            })
            .expect("listener failed");
        })
        .unwrap();

    match std::env::args().nth(1) {
        Some(addr) => {
            let ip_addr = IpAddr::from_str(&addr).unwrap();
            events_tx_clone.send(Event::JoinSend(ip_addr)).unwrap();
        }
        _ => {
            events_tx_clone.send(Event::StartRoom).unwrap();
        }
    }

    loop {
        while let Ok(event) = events_rx.recv() {
            println!("received event: {event:?}");
            manager.handle(event);
        }
    }
}
