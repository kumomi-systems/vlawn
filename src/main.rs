mod core;

use std::fs::File;

use simplelog::{Config, WriteLogger};
use ws::listen;

use core::{Control, Endpoint, Process};

fn main() -> anyhow::Result<()> {
    let now = chrono::Local::now();
    WriteLogger::init(
        log::LevelFilter::Debug,
        Config::default(),
        File::create(format!("{}.log", now.format("%Y%m%y_%H%M%S"))).unwrap(),
    )
    .unwrap();

    let mut proc = Process::new();
    let ctrl_tx = proc.ctrl_tx();

    std::thread::Builder::new()
        .name("websocket server".into())
        .spawn(move || {
            log::info!("websocket server started");
            listen("0.0.0.0:57185", |out| {
                log::info!("new connection");
                Endpoint::new(out, ctrl_tx.clone())
            })
            .expect("listener failed");
        })?;

    let ctrl = match std::env::args().nth(1) {
        Some(_) => todo!(),
        None => Control::Host,
    };
    proc.submit_ctrl(ctrl)?;

    loop {
        proc = proc.step()?;
    }
}
