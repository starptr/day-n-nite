use super::SetError;
use dirs;
use super::simple_string_replace;
use super::Mode;

// Use simple string replacer to convert day-n-nite template to readable config
pub fn set(mode: Mode) -> Result<(), SetError> {
    let mut bat_config_file = dirs::home_dir().unwrap();
    bat_config_file.push(".config");
    bat_config_file.push("bat");
    bat_config_file.push("config.day-n-nite");
    let mut bat_config_generated_file = bat_config_file.clone();
    bat_config_generated_file.set_extension("");
    simple_string_replace(mode, bat_config_file, bat_config_generated_file)?;
    Ok(())
}
