use std::env;
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE, KEY_CREATE_SUB_KEY};
use winreg::RegKey;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use crossterm::event::KeyEvent;
use crate::actions_register::Actions;
use crate::manager::init_manager;
use crate::editor::init_editor;
use crate::ShouldDo;

pub fn to_ascii_layout(ch: char) -> char {
    match ch.to_lowercase().next().unwrap_or(ch) {
        'й' => 'q',
        'ц' => 'w',
        'у' => 'e',
        'к' => 'r',
        'е' => 't',
        'н' => 'y',
        'г' => 'u',
        'ш' => 'i',
        'щ' => 'o',
        'з' => 'p',
        'ф' => 'a',
        'ы' => 's',
        'в' => 'd',
        'а' => 'f',
        'п' => 'g',
        'р' => 'h',
        'о' => 'j',
        'л' => 'k',
        'д' => 'l',
        'я' => 'z',
        'ч' => 'x',
        'с' => 'c',
        'м' => 'v',
        'и' => 'b',
        'т' => 'n',
        'ь' => 'm',
        _ => ch.to_ascii_lowercase(),
    }
}

pub fn is_move_or_drag(kind: crossterm::event::MouseEventKind) -> bool {
    matches!(
        kind,
        crossterm::event::MouseEventKind::Drag(_) | crossterm::event::MouseEventKind::Moved
    )
}

pub fn trace_log(msg: &str) {
    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).create(true).open("timeline.log") {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let _ = writeln!(file, "[{}] {}", now, msg);
    }
}

pub trait CommonFields {
    fn should_do(&self) -> ShouldDo;
    fn keybinds(&self) -> HashMap<KeyEvent, Actions>;
    fn process_action(&mut self, action: &Actions);
    fn configure_keybinds(&mut self);
    fn quit(&mut self);
}

pub fn launch_window(should_do: ShouldDo) -> color_eyre::Result<()> {
    match should_do {
        ShouldDo::OpenEditor(path) => init_editor(),
        ShouldDo::OpenManager => init_manager(),
        ShouldDo::OpenViewer(path) => Ok(()),
        _ => Ok(())
    }
}

const EXT: &str = ".ascm";
const PROG_ID: &str = "Escimo.FileAssoc";
const FILE_DESC: &str = "ASCII-Motions";

pub fn reg() -> std::io::Result<()> {
    let exe_path = env::current_exe()?;
    let expected_cmd = format!("\"{}\" \"%1\"", exe_path.to_string_lossy());

    if is_association_valid(&expected_cmd) {
        println!("Association is already in registry.");
    } else {
        println!("Writing to registry...");
        register_association(&expected_cmd)?;
    }

    Ok(())
}

fn is_association_valid(expected_cmd: &str) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let ext_path = format!("Software\\Classes\\{}", EXT);
    let ext_key = match hkcu.open_subkey_with_flags(&ext_path, KEY_READ) {
        Ok(key) => key,
        Err(_) => return false,
    };

    let registered_prog_id: String = match ext_key.get_value("") {
        Ok(val) => val,
        Err(_) => return false,
    };

    if registered_prog_id != PROG_ID {
        return false;
    }

    let cmd_path = format!("Software\\Classes\\{}\\shell\\open\\command", PROG_ID);
    let cmd_key = match hkcu.open_subkey_with_flags(&cmd_path, KEY_READ) {
        Ok(key) => key,
        Err(_) => return false,
    };

    let registered_cmd: String = match cmd_key.get_value("") {
        Ok(val) => val,
        Err(_) => return false,
    };

    registered_cmd == expected_cmd
}

fn register_association(cmd_str: &str) -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes_path = format!("Software\\Classes\\{}", PROG_ID);

    let (prog_key, _) = hkcu.create_subkey_with_flags(&classes_path, KEY_SET_VALUE | KEY_CREATE_SUB_KEY)?;
    prog_key.set_value("", &FILE_DESC)?;

    let (cmd_key, _) = hkcu.create_subkey_with_flags(
        format!("{}\\shell\\open\\command", classes_path),
        KEY_SET_VALUE
    )?;
    cmd_key.set_value("", &cmd_str)?;

    let exe_path = env::current_exe()?;
    let icon_path = format!("\"{}\",0", exe_path.to_string_lossy());

    let (icon_key, _) = hkcu.create_subkey_with_flags(
        format!("{}\\DefaultIcon", classes_path),
        KEY_SET_VALUE
    )?;
    icon_key.set_value("", &icon_path)?;

    let ext_path = format!("Software\\Classes\\{}", EXT);
    let (ext_key, _) = hkcu.create_subkey_with_flags(&ext_path, KEY_SET_VALUE)?;
    ext_key.set_value("", &PROG_ID)?;

    let (progids_key, _) = hkcu.create_subkey_with_flags(
        format!("{}\\OpenWithProgids", ext_path),
        KEY_SET_VALUE
    )?;
    
    progids_key.set_value(&PROG_ID, &"")?;

    Ok(())
}