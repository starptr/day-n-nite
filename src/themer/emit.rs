use super::{Mode, ModuleError, SetError};
use std::process::Command;

pub fn set(mode: Mode) -> Result<(), ModuleError> {
    if option_env!("IS_WSL").unwrap_or("false") == "true" {
        Command::new("powershell.exe")
            .args(&["-C", "day-n-nite", "--no-emit"])
            .output()
            .map_err(|_| ModuleError::Emit(SetError::CommandFailure))?;
    }
    Ok(())
}
