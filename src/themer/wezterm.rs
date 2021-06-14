use std::fs;
use dirs;
use super::SetError;

// Force WezTerm to reload config to read new color mode
pub fn update() -> Result<(), SetError> {
    let mut wezterm_config_file = dirs::home_dir().unwrap();
    wezterm_config_file.push(".config");
    wezterm_config_file.push("wezterm");
    wezterm_config_file.push("wezterm.lua");
    let config = fs::read(&wezterm_config_file).map_err(|_| SetError::ReadFailure)?;
    fs::write(wezterm_config_file, config).map_err(|_| SetError::WriteFailure)?;
    Ok(())
}
