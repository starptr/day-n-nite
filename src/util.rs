use std::path::{Path, PathBuf};

use directories::UserDirs;

pub fn write_data(path: &Path, data: &str) -> Result<(), String> {
    use std::fs;
    fs::write(path, data).map(|_| ()).map_err(|e| e.to_string())
}

pub fn read_data(path: &Path) -> Result<String, String> {
    use std::fs;
    fs::read_to_string(path).map_err(|e| e.to_string())
}

/*
Converts an input filename into absolute path
*/
pub fn get_abspath(input_filename: String) -> Result<PathBuf, String> {
    use std::env;
    let path = Path::new(input_filename.as_str());
    if path.is_absolute() {
        return Ok(path.into());
    }

    let cwd = env::current_dir().map_err(|e| e.to_string())?;
    let path = cwd.join(path);
    Ok(path)
}

pub fn with_prefix<'a, T: ToString>(prefix: &'a str) -> impl Fn(T) -> String + 'a {
    |e| prefix.to_string() + ": " + &e.to_string()
}
