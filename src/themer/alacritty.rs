use super::ModuleError;
use dirs;
use super::simple_string_replace;
use super::Mode;

pub fn set(mode: Mode) -> Result<(), ModuleError> {
    let alacritty_config_file = dirs::home_dir().unwrap().join(".config").join("alacritty").join("alacritty.yml.day-n-nite");
    let mut alacritty_config_generated_file = alacritty_config_file.clone();
    alacritty_config_generated_file.set_extension("");
    simple_string_replace(mode, alacritty_config_file, alacritty_config_generated_file).map_err(|e| ModuleError::Alacritty(e))?;
    Ok(())
}
