use std::io::stdout;
use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use crossterm::ExecutableCommand;
use crossterm::terminal::SetSize;

use crate::manager::input::process_keycode;
use crate::manager::app::App;
use crate::event::{Event, EventHandler};
use crate::tui::Tui;

use color_eyre::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::common::{launch_window, CommonFields};
use crate::ShouldDo;

pub mod app;
pub mod input;
mod layout;

pub fn init_manager() -> Result<()> {
    stdout().execute(EnableMouseCapture)?;
    stdout().execute(SetSize(80, 24))?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut app = App::new();
    app.configure_keybinds();

    while matches!(app.should_do, ShouldDo::Continue) {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => process_keycode(&mut app, key_event),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {}
        };
    }

    launch_window(app.should_do)?;

    stdout().execute(DisableMouseCapture)?;
    tui.exit()?;
    Ok(())
}