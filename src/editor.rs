use crossterm::event::{DisableMouseCapture, EnableMouseCapture, PushKeyboardEnhancementFlags, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, KeyModifiers, MouseButton, MouseEventKind};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use crossterm::execute;
use std::io::stdout;
use std::panic;
use crate::editor::app::App;
use crate::editor::input::{process_keycode, process_mouse};
use crate::event::{Event, EventHandler};
use crate::tui::Tui;

use color_eyre::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::common::{launch_window, is_move_or_drag, trace_log, CommonFields};
use crate::ShouldDo;

pub mod app;
pub mod input;
pub mod tools;
pub mod layout;

pub fn init_editor() -> Result<()> {
    //
    // execute!(
    //     stdout(),
    //     EnableMouseCapture,
    //     EnterAlternateScreen,
    //     // PushKeyboardEnhancementFlags(
    //     //     KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
    //     //         | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
    //     // )
    // )?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut app = App::new(64, 32);
    app.configure_keybinds();
    app.tools_state.select(Some(0));

    let mut counter: u8 = 0;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        //let _ = terminal::disable_raw_mode();
        // let _ = execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture);

        if let Ok(mut file) = std::fs::OpenOptions::new().append(true).create(true).open("crash_report.txt") {
            use std::io::Write;
            let _ = writeln!(file, "=== ПРОГРАММА УПАЛА С ПАНИКОЙ ===");
            let _ = writeln!(file, "Информация: {}", panic_info);
        }

        panic_hook(panic_info);
    }));

    while matches!(app.should_do, ShouldDo::Continue) {
        let first_event = tui.events.next()?;

        let mut is_dirty = false;

        match first_event {
            Event::Tick => is_dirty = true,
            Event::Key(key_event) => {
                process_keycode(&mut app, key_event);
                is_dirty = true;
            }
            Event::Mouse(mouse_event) => {
                process_mouse(&mut app, mouse_event);
                if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    counter += 1;
                    trace_log(&format!("{}{}{}", counter, counter, counter));
                };
                is_dirty = true;
            }
            Event::Resize(_, _) => is_dirty = true,
        }

        while let Ok(Some(next_event)) = tui.events.try_next() {
            match next_event {
                Event::Tick => is_dirty = true,
                Event::Key(key_event) => {
                    process_keycode(&mut app, key_event);
                    is_dirty = true;
                }
                Event::Mouse(mouse_event) => {
                    process_mouse(&mut app, mouse_event);
                    is_dirty = true;
                }
                Event::Resize(_, _) => is_dirty = true,
            }
        }

        if is_dirty {
            tui.draw(&mut app)?;
        }
    }
    
    launch_window(app.should_do)?;

    // execute!(
    //     stdout(),
    //     DisableMouseCapture,
    //     LeaveAlternateScreen,
    //     // PopKeyboardEnhancementFlags,
    // )?;
    tui.exit()?;
    Ok(())
}