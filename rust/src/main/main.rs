use color_eyre::Result;

mod app;
mod tui;

use crate::app::App;
use crate::tui::Tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"
        );
    }
    app_result
}