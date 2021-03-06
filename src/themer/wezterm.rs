use std::fs;
use dirs;
use super::SetError;
use super::ModuleError;

// Force WezTerm to reload config to read new color mode
pub fn update() -> Result<(), ModuleError> {
    let wezterm_config_file = dirs::home_dir().unwrap().join(".config").join("wezterm").join("wezterm.lua");
    let config = fs::read(&wezterm_config_file).map_err(|_| ModuleError::Wezterm(SetError::ReadFailure))?;
    fs::write(wezterm_config_file, config).map_err(|_| ModuleError::Wezterm(SetError::WriteFailure))?;
    Ok(())
}
