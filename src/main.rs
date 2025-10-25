mod admin;
mod entities;
mod ip;
mod member;
mod ui;

use simplelog::{Config, TermLogger};

use color_eyre::Result;

fn main() -> Result<()> {
    TermLogger::init(
        simplelog::LevelFilter::Info,
        Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = ui::App::new().run(terminal);
    ratatui::restore();
    app_result
}
