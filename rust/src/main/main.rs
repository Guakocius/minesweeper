use color_eyre::Result;

mod observer;
mod view;
use crate::view::tui::app::App;
use crate::view::tui::terminal::Tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = view::tui::terminal::init()?;
    let app_result = App::default().run(&mut terminal);
    if let Err(err) = view::tui::terminal::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"
        );
    }
    app_result
}