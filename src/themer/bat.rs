use super::ModuleError;
use dirs;
use super::simple_string_replace;
use super::Mode;

// Use simple string replacer to convert day-n-nite template to readable config
pub fn set(mode: Mode) -> Result<(), ModuleError> {
    let bat_config_file = dirs::home_dir().unwrap().join(".config").join("bat").join("config.day-n-nite");
    let mut bat_config_generated_file = bat_config_file.clone();
    bat_config_generated_file.set_extension("");
    simple_string_replace(mode, bat_config_file, bat_config_generated_file).map_err(|e| ModuleError::Bat(e))?;
    Ok(())
}
