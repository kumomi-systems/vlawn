mod admin;
mod entities;
mod ip;
mod member;
mod ui;

use std::fs::File;

use simplelog::{Config, WriteLogger};

use color_eyre::Result;

fn main() -> Result<()> {
    let now = chrono::Local::now();
    WriteLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        File::create(format!("{}.log", now.format("%Y%m%y_%H%M%S"))).unwrap(),
    )
    .unwrap();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = ui::App::new().run(terminal);
    ratatui::restore();
    app_result
}
