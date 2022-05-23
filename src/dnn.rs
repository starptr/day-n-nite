use std::path::Path;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

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

fn get_theme() -> Theme {
    get_system_theme()
}

fn set_theme(theme: Theme) -> Result<(), String> {
    set_system_theme(theme)
}

fn get_dnn_data_path() -> PathBuf {
    let projdir = ProjectDirs::from("", env!("CARGO_PKG_AUTHORS"), env!("CARGO_CRATE_NAME")).unwrap();
    let data_dir = projdir.data_dir();
    let dnn_data = {
        let mut dnn_data_path_builder = data_dir.to_path_buf();
        dnn_data_path_builder.push("dnn");
        dnn_data_path_builder.set_extension("toml");
        dnn_data_path_builder
    };
    dnn_data
}

#[derive(Serialize, Deserialize)]
enum DnnEntryKind {
    Script,
    Config,
}

#[derive(Serialize, Deserialize)]
struct DnnEntry {
    path: PathBuf,
    kind: DnnEntryKind,
    hash: (),
}

#[derive(Serialize, Deserialize)]
struct DnnData {
    configs: Vec<DnnEntry>,
}
fn get_dnn_data() -> Result<DnnData, String> {
    use std::fs::read_to_string;
    let dnn_data_path = get_dnn_data_path();
    let dnn_data = read_to_string(dnn_data_path).map_err(|e| e.to_string())?;
    let dnn_data: DnnData = toml::from_str(&dnn_data).map_err(|e| e.to_string())?;
    Ok(dnn_data)
}

pub fn cmd_toggle() -> Result<(), String> {
    let target_theme = get_theme().invert();
    set_theme(target_theme)
}

pub fn cmd_add(input_filename: String) -> Result<(), String> {
    let path = Path::new(input_filename.as_str());

    let dnn_data = read_data(Path::new(&get_pathstr(input_filename)?))?;
    Ok(())
}
