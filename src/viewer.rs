use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyModifiers, MouseButton, MouseEventKind};
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::panic;
use std::path::PathBuf;
use crate::viewer::app::App;
use crate::viewer::input::{process_keycode, process_mouse};
use crate::event::{Event, EventHandler};
use crate::tui::Tui;

use color_eyre::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::common::{launch_window, is_move_or_drag, trace_log, CommonFields};
use crate::ShouldDo;

pub mod app;
pub mod input;
pub mod layout;

pub fn init_viewer(file: PathBuf) -> Result<()> {
    stdout().execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(16);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut app = App::new(file);
    app.configure_keybinds();

    while matches!(app.should_do, ShouldDo::Continue) {
        let first_event = tui.events.next()?;

        let mut is_dirty = false;

        match first_event {
            Event::Tick => is_dirty = true,
            Event::Key(key_event) => {
                process_keycode(&mut app, key_event);
                // is_dirty = true;
            }
            Event::Mouse(mouse_event) => {
                process_mouse(&mut app, mouse_event);
                // is_dirty = true;
            }
            Event::Resize(_, _) => {} // is_dirty = true,
        }

        while let Ok(Some(next_event)) = tui.events.try_next() {
            match next_event {
                Event::Tick => is_dirty = true,
                Event::Key(key_event) => {
                    process_keycode(&mut app, key_event);
                    // is_dirty = true;
                }
                Event::Mouse(mouse_event) => {
                    process_mouse(&mut app, mouse_event);
                    // is_dirty = true;
                }
                Event::Resize(_, _) => {} // is_dirty = true,
            }
        }

        if is_dirty {
            tui.draw(&mut app)?;
        }
    }
    
    launch_window(app.should_do)?;

    stdout().execute(DisableMouseCapture)?;
    tui.exit()?;
    Ok(())
}