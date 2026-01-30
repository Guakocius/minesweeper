<<<<<<< HEAD
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
=======
use std::thread;

use color_eyre::Result;

mod observer;
mod view;
mod model;

use crate::view::tui::app::App;
use crate::view::tui::terminal::Tui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let handle = thread::spawn(|| -> Result<()> {
        let mut terminal = view::tui::terminal::init()?;
        let app_result = App::default().run(&mut terminal);
        if let Err(err) = view::tui::terminal::restore() {
            eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"
            );
        }
        app_result
    });
    handle.join().expect("tui thread panicked")?;
    Ok(())
>>>>>>> 166f2dcd293967d29e5536fbfe26fe61898c38e9
}