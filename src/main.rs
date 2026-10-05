#[cfg(target_os = "windows")]
fn fix_windows_console() {
    unsafe {
        windows_sys::Win32::System::Console::SetConsoleCP(65001);
        windows_sys::Win32::System::Console::SetConsoleOutputCP(65001);

    }
}

// pub mod app;
pub mod event;
pub mod tui;
// pub mod input;
// mod tools;

pub mod editor;
pub mod manager;
pub mod viewer;
pub mod ui;
mod common;
mod input;
mod actions_register;

use color_eyre::Result;

use crossterm::ExecutableCommand;

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub enum ShouldDo {
    #[default]
    Continue,
    OpenEditor(Option<PathBuf>),
    OpenViewer(PathBuf),
    OpenManager,
    Quit,
}

#[derive(Parser, Debug)]
#[command(name = "escimo", version, about = "ASCII-Movie Editor & Player")]
struct Args {
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(short, long)]
    edit: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    common::reg()?;

    match args.file {
        Some(path) => {
            if args.edit {
                editor::init_editor()?;
            } else {
                viewer::init_viewer(path)?;
            }
        }
        None => {
            if args.edit {
                editor::init_editor()?;
            } else {
                manager::init_manager()?;
            }
        }
    }

    Ok(())
}