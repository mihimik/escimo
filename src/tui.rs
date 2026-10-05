use std::{io, panic};

use color_eyre::Result;
use crossterm::event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use ratatui::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::ui;
use crate::event::EventHandler;

pub trait RenderableApp {
    fn render(&mut self, frame: &mut ratatui::Frame);
}

#[cfg(windows)]
fn force_enable_vt_mode() -> io::Result<()> {
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode,
        ENABLE_VIRTUAL_TERMINAL_INPUT, STD_INPUT_HANDLE
    };

    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        if handle.is_null() || handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::new(io::ErrorKind::Other, "Не удалось получить дескриптор ввода"));
        }

        let mut mode = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return Err(io::Error::last_os_error());
        }

        mode |= ENABLE_VIRTUAL_TERMINAL_INPUT;

        if SetConsoleMode(handle, mode) == 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn force_enable_vt_mode() -> io::Result<()> {
    Ok(()) // На Linux/macOS это не требуется
}

pub struct Tui {
    terminal: CrosstermTerminal,
    pub events: EventHandler,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }
    
    pub fn enter(&mut self) -> Result<()> {
        // force_enable_vt_mode()?;
        terminal::enable_raw_mode()?;
        if let Err(_) = execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture,
            // PushKeyboardEnhancementFlags(
            //     KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
            //         | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
            // )
        ) {};
        
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw<A: RenderableApp>(&mut self, app: &mut A) -> Result<()> {
        self.terminal.draw(|frame| app.render(frame))?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture,
            // PopKeyboardEnhancementFlags
        )?;
        Ok(())
    }
    
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}