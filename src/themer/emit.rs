use super::{Mode, ModuleError, SetError};
use std::process::Command;

use std::env::consts::OS;

pub fn set(mode: Mode) -> Result<(), ModuleError> {
    if option_env!("IS_WSL").unwrap_or("false") == "true" {
        Command::new("powershell.exe")
            .args(&["-C", "day-n-nite", if mode == Mode::Day { "-d" } else { "-n" }, "--no-emit"])
            .output()
            .map_err(|_| ModuleError::Emit(SetError::CommandFailure))?;
    } else if OS == "windows" {
        Command::new("wsl")
            .args(&["--", "day-n-nite", if mode == Mode::Day { "-d" } else { "-n" }, "--no-emit"])
            .output()
            .map_err(|_| ModuleError::Emit(SetError::CommandFailure))?;
    }
    Ok(())
}
