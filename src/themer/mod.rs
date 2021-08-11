mod disk_record;
mod bat;
mod emit;
mod system_os;
mod terminal;
mod vim;
mod wezterm;
mod alacritty;
use clap::ArgMatches;
use lazy_static::*;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::string::String;
use strum_macros::Display;
use strum_macros::EnumString;

pub enum GetError {
    UnknownMode,
    NoMode,
}

#[derive(Debug)]
pub enum SetError {
    WriteFailure,
    ReadFailure,
    CommandFailure,
    ParseFailure,
}

#[derive(Debug)]
pub enum ModuleError {
    DayNNite(SetError),
    SystemWindows(SetError),
    Wezterm(SetError),
    Terminal(SetError),
    _Vim(SetError),
    Bat(SetError),
    Emit(SetError),
    Alacritty(SetError),
}

#[derive(Display, PartialEq, EnumString, Clone, Copy)]
pub enum Mode {
    Day,
    Night,
}

pub fn toggle(mode: Mode) -> Mode {
    match mode {
        Mode::Night => Mode::Day,
        Mode::Day => Mode::Night,
    }
}

pub fn get_mode() -> Result<Mode, GetError> {
    let config_content = fs::read_to_string(disk_record::get_config_filepath());
    match config_content {
        Ok(mode) => match Mode::from_str(&mode) {
            Ok(mode) => Ok(mode),
            Err(_) => Err(GetError::UnknownMode),
        },
        Err(_) => Err(GetError::NoMode),
    }
}

pub fn set_mode(mode: Mode, arg_matches: ArgMatches) -> Result<Mode, ModuleError> {
    // Must write to disk first
    disk_record::write(mode)?;
    system_os::set(mode)?;
    wezterm::update()?;
    terminal::set(mode)?;
    vim::update()?;
    bat::set(mode)?;
    alacritty::set(mode)?;
    if !arg_matches.is_present("no_emit") {
        emit::set(mode)?;
    }
    Ok(mode)
}

// Replace 〔light〜dark〕 with light if day, dark if night
fn simple_string_replace(
    mode: Mode,
    day_n_nite_template: PathBuf,
    destination_pathbuf: PathBuf,
) -> Result<(), SetError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            〔
            (?P<day>[^〜]*)
            〜
            (?P<night>[^〕]*)
            〕"
        )
        .unwrap();
    }

    let mut replaced_text = String::new();

    for line in
        BufReader::new(File::open(day_n_nite_template).map_err(|_| SetError::ReadFailure)?).lines()
    {
        let line_text = line.unwrap();
        let mut line = RE.replace_all(
            &line_text,
            if mode == Mode::Day { "$day" } else { "$night" },
        );
        line.to_mut().push_str("\n");
        replaced_text += &line;
    }
    fs::write(destination_pathbuf, replaced_text).map_err(|_| SetError::WriteFailure)?;
    Ok(())
}
