mod system_windows;
mod wezterm;
mod vim;
mod bat;
mod terminal;
use std::str::FromStr;
use std::string::ToString;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum_macros::EnumString;
use strum_macros::Display;
use std::string::String;
use std::fs;
use lazy_static::*;
use regex::Regex;
use dirs;

pub enum GetError {
    UnknownMode,
    NoMode,
}

#[derive(Debug)]
pub enum SetError {
    WriteFailure,
    ReadFailure,
    RegEditFailure,
    ParseFailure,
}

#[derive(Debug)]
pub enum ModuleError {
    DayNNite(SetError),
    SystemWindows(SetError),
    Wezterm(SetError),
    Terminal(SetError),
    Vim(SetError),
    Bat(SetError),
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

pub fn set_night() -> Result<Mode, ModuleError> {
    set_mode(Mode::Night).map(|_| Mode::Night)
}

pub fn set_day() -> Result<Mode, ModuleError> {
    set_mode(Mode::Day).map(|_| Mode::Day)
}

fn set_mode(mode: Mode) -> Result<(), ModuleError> {
    // Must write to disk first
    fs::write(get_config_filepath(), mode.to_string())
        .map_or_else(|_| Err(ModuleError::DayNNite(SetError::WriteFailure)), |_| Ok(mode))?;
    system_windows::set(mode)?;
    wezterm::update()?;
    terminal::set(mode)?;
    vim::update()?;
    bat::set(mode)?;
    Ok(())
}

// Replace 〔light〜dark〕 with light if day, dark if night
fn simple_string_replace(mode: Mode, day_n_nite_template: PathBuf, destination_pathbuf: PathBuf) -> Result<(), SetError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            〔
            (?P<day>[^〜]*)
            〜
            (?P<night>[^〕]*)
            〕").unwrap();
    }

    let mut replaced_text = String::new();

    for line in BufReader::new(File::open(day_n_nite_template)
            .map_err(|_| SetError::ReadFailure)?)
            .lines() {
        let line_text = line.unwrap();
        let mut line = RE.replace_all(&line_text, if mode == Mode::Day { "$day" } else { "$night" });
        line.to_mut().push_str("\n");
        replaced_text += &line;
    }
    fs::write(destination_pathbuf, replaced_text).map_err(|_| SetError::WriteFailure)?;
    Ok(())
}
