mod system_windows;
mod wezterm;
use std::str::FromStr;
use std::string::ToString;
use std::path::PathBuf;
use strum_macros::EnumString;
use strum_macros::Display;
use dirs;
use std::fs;

pub enum GetError {
    UnknownMode,
    NoMode,
}

pub enum SetError {
    WriteFailure,
    ReadFailure,
    RegEditFailure,
}

#[derive(Display, PartialEq, EnumString, Clone, Copy)]
pub enum Mode {
    Day,
    Night,
}

pub fn get_config_filepath() -> PathBuf {
    let mut config_file_pathbuf = dirs::config_dir().unwrap();
    config_file_pathbuf.push(env!("CARGO_PKG_NAME"));
    config_file_pathbuf.push("mode_config");
    config_file_pathbuf
}

pub fn get_mode() -> Result<Mode, GetError> {
    let config_content = fs::read_to_string(get_config_filepath());
    match config_content {
        Ok(mode) => {
            match Mode::from_str(&mode) {
                Ok(mode) => Ok(mode),
                Err(_) => Err(GetError::UnknownMode),
            }
        },
        Err(_) => {
            Err(GetError::NoMode)
        }
    }
}

pub fn set_night() -> Result<Mode, SetError> {
    set_mode(Mode::Night).map(|_| Mode::Night)
}

pub fn set_day() -> Result<Mode, SetError> {
    set_mode(Mode::Day).map(|_| Mode::Day)
}

fn set_mode(mode: Mode) -> Result<(), SetError> {
    fs::write(get_config_filepath(), mode.to_string())
        .map_or_else(|_| Err(SetError::WriteFailure), |_| Ok(mode))?;
    system_windows::set(mode)?;
    wezterm::set()?;
    Ok(())
}
