use super::{Mode, ModuleError, SetError};
use std::fs;

pub fn get_config_filepath() -> std::path::PathBuf {
    dirs::home_dir().unwrap().join(".config").join(env!("CARGO_PKG_NAME")).join("mode_config")
}

pub fn write(mode: Mode) -> Result<Mode, ModuleError> {
    let mut config_file_pathbuf = dirs::home_dir().unwrap().join(".config").join(env!("CARGO_PKG_NAME"));
    fs::create_dir_all(&config_file_pathbuf).expect("Could not create directory for mode data.");
    config_file_pathbuf.push("mode_config");
    fs::write(config_file_pathbuf, mode.to_string()).map_or_else(|_| Err(ModuleError::DayNNite(SetError::WriteFailure)), |_| Ok(mode))
}
