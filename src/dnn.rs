use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::LineWriter;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use clap::ArgEnum;
use clap::Subcommand;
use clap::lazy_static::lazy_static;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use regex::*;

use crate::DNN_FILE_EXTENSION;
use crate::os::*;
use crate::util::*;

#[derive(Clone, Copy, PartialEq, Debug)]
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

fn substitution(data: String, target: Theme) -> Result<String, String> {
    Err("unimp".to_string())
}

fn new_job(entry: DnnEntry, target: Theme) -> Result<Box<dyn FnOnce() -> Result<(), String>>, String> {
    match entry.kind {
        DnnEntryKind::Config => {
            let extension = entry.path.extension().unwrap_or(OsStr::new("")).to_str().ok_or("OsStr is not valid utf8")?;
            if extension != DNN_FILE_EXTENSION {
                Err("Invalid file extension".to_string())
            } else {
                Ok(Box::new(move || {
                    lazy_static! {
                        static ref RE: Regex = Regex::new(
                            r"(?x)
                            〔
                            (?P<day>[^〜]*)
                            〜
                            (?P<night>[^〕]*)
                            〕"
                        ).unwrap();
                    }


                    let mut target_file = {
                        let mut target_path = entry.path.clone();
                        target_path.set_extension("");
                        File::create(target_path).map_err(with_prefix("Creating target file for writing failed"))?
                    };

                    let config = File::open(entry.path).map_err(with_prefix("Opening dnn data file failed"))?;
                    for line in BufReader::new(config).lines() {
                        let line = line.map_err(with_prefix("BufReader line iterator failed"))?;
                        let line = RE.replace_all(&line, match target {
                            Theme::Light => "$day",
                            Theme::Dark => "$night",
                        });
                        use std::io::Write;
                        write!(target_file, "{}", line).map_err(with_prefix("Writing to target file failed"))?;
                    }
                    Ok(())
                }))
            }
        }
        DnnEntryKind::Script => {
            Ok(Box::new(move || {
                let res = Command::new(entry.path).output()
                    .map(|_output| ())
                    .map_err(with_prefix("Running target script failed"));
                res
            }))
        }
        DnnEntryKind::Pair => {
            let _extension = entry.path.extension().unwrap_or(OsStr::new("")).to_str().ok_or("OsStr is not valid utf8")?;
            panic!("Not implemented")
        }
    } //.map_err(with_prefix(&format!("Setting up job for {} failed", entry.path.display())))
}

fn get_jobs(target: Theme) -> Result<Vec<impl FnOnce() -> Result<(), String>>, String> {
    let dnn_data = get_dnn_data()?;
    let mut jobs: Vec<_> = dnn_data.configs.into_iter().map(|entry| new_job(entry, target)).collect();
    if jobs.iter().any(|job| job.is_err()) {
        Err("Failed to retrieve some jobs".to_string())
    } else {
        Ok(jobs.into_iter().map(|job| job.unwrap()).collect())
    }
}

fn set_theme(target: Theme) -> Result<(), String> {
    set_system_theme(target)?;
    let jobs = get_jobs(target)?;
    let res = jobs.into_iter()
        .filter_map(|f| f().err())
        .fold(String::new(), |acc, elem| acc + &elem + "\n");
    if res.is_empty() { Ok(()) } else { Err(res) }
}

fn get_dnn_data_path() -> PathBuf {
    let projdir = ProjectDirs::from("", env!("CARGO_PKG_AUTHORS"), env!("CARGO_CRATE_NAME")).unwrap();
    let data_dir = projdir.data_local_dir();
    let dnn_data = {
        let mut dnn_data_path_builder = data_dir.to_path_buf();
        dnn_data_path_builder.push("dnn");
        dnn_data_path_builder.set_extension("toml");
        dnn_data_path_builder
    };
    dnn_data
}

#[derive(Copy, Clone, ArgEnum, Serialize, Deserialize, Debug)]
pub enum DnnEntryKind {
    Script,
    Config,
    Pair,
}

#[derive(Serialize, Deserialize, Debug)]
struct DnnEntry {
    path: PathBuf,
    kind: DnnEntryKind,
}

#[derive(Serialize, Deserialize, Debug)]
struct DnnData {
    configs: Vec<DnnEntry>,
}
fn get_dnn_data() -> Result<DnnData, String> {
    use std::fs::read_to_string;
    let dnn_data_path = get_dnn_data_path();
    let dnn_data = read_to_string(dnn_data_path)
        .map_or_else(|e| match e.kind() {
            ErrorKind::NotFound => Ok("configs = []".to_string()),
            _ => Err(e),
        }, |s| Ok(s))
        .map_err(with_prefix("Reading dnn data file failed"))?;
    let dnn_data: DnnData = toml::from_str(&dnn_data).map_err(with_prefix("toml deserialization failed"))?;
    Ok(dnn_data)
}
fn set_dnn_data(data: DnnData) -> Result<(), String> {
    let mut dnn_data_file = {
        let path = get_dnn_data_path();
        File::create(&path)
            .map_or_else(|e| -> Result<File, String> { match e.kind() {
                ErrorKind::NotFound => {
                    let parent_path = path.parent().ok_or("Parent path is None")?;
                    fs::create_dir_all(parent_path).map_err(with_prefix("Failed to recursively create parent directories for dnn data file"))?;
                    File::create(path).map_err(with_prefix("Creating dnn data file still failed"))
                }
                _ => Err(e.to_string())
            }}, |s| Ok(s))
            .map_err(with_prefix("Creating dnn data file for writing failed"))?
    };
    let dnn_data = toml::to_string(&data).map_err(with_prefix("toml serialization failed"))?;
    use std::io::Write;
    write!(dnn_data_file, "{}", dnn_data).map_err(with_prefix("Writing to dnn data file failed"))
}

pub fn cmd_toggle() -> Result<(), String> {
    let target_theme = get_theme().invert();
    set_theme(target_theme)
}

pub fn cmd_add(input_filename: String, kind: DnnEntryKind) -> Result<(), String> {
    let path = get_abspath(input_filename)?;
    let dnn_data = get_dnn_data()?;
    let idx = dnn_data.configs.iter().position(|e| e.path == path);
    let mut dnn_data = dnn_data;
    if let Some(idx) = idx {
        let mut entry = &mut dnn_data.configs[idx];
        entry.kind = kind;
    } else {
        let entry = DnnEntry {
            path,
            kind,
        };
        dnn_data.configs.push(entry);
    }
    set_dnn_data(dnn_data)
}

pub fn cmd_rm(input_filename: String) -> Result<(), String> {
    let path = get_abspath(input_filename)?;
    let dnn_data = get_dnn_data()?;
    let idx = dnn_data.configs.iter().position(|e| e.path == path);
    if idx == None {
        return Err("File is not already added".to_string());
    }
    let idx = idx.unwrap();
    let mut dnn_data = dnn_data;
    dnn_data.configs.swap_remove(idx);
    set_dnn_data(dnn_data)
}
