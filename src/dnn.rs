use std::path::PathBuf;

use directories::ProjectDirs;

use crate::os::*;
use crate::util::*;

#[derive(PartialEq)]
pub enum Theme {
    Light,
    Dark,
}
impl Theme {
    pub fn invert(self) -> Theme {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }
}

pub fn get_theme() -> Theme {
    get_system_theme()
}

pub fn set_theme(theme: Theme) -> Result<(), String> {
    set_system_theme(theme)
}

fn get_dnn_data_path() -> PathBuf {
    let projdir = ProjectDirs::from("", env!("CARGO_PKG_AUTHORS"), env!("CARGO_CRATE_NAME")).unwrap();
    let data_dir = projdir.data_dir();
    let dnn_data = {
        let dnn_data_path_builder = data_dir.to_path_buf();
        dnn_data_path_builder.push("dnn");
        dnn_data_path_builder.set_extension("toml");
        dnn_data_path_builder
    };
    dnn_data
}


pub fn cmd_add(input_filename: String) -> Result<(), String> {
    use std::path::Path;
    let path = Path::new(input_filename.as_str());

    let dnn_data = read_data(filename)?;
    Ok(())
}
