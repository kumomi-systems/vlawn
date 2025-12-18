mod core;

use std::fs::File;

use simplelog::{Config, WriteLogger};

fn main() {
    let now = chrono::Local::now();
    WriteLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        File::create(format!("{}.log", now.format("%Y%m%y_%H%M%S"))).unwrap(),
    )
    .unwrap();
}
