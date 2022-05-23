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
Converts an input filename into absolute path wrt home or root
1) Root if input is an absolute path or a path that does not have `~` as an ancestor
2) Home otherwise
*/
pub fn get_pathstr(input_filename: String) -> Result<String, String> {
    use std::env;
    let path = Path::new(input_filename.as_str());
    if path.is_absolute() {
        return Ok(input_filename);
    }

    let userdirs = UserDirs::new().ok_or("User dirs could not be initialized in memory")?;
    let homedir = userdirs.home_dir();

    let cwd = env::current_dir().map_err(|e| e.to_string())?;
    let path_with_cwd = cwd.join(path);
    if !path_with_cwd.starts_with(homedir) {
        path_with_cwd.into_os_string().into_string().map_err(|_| "OsString is invalid utf8".into())
    } else {
        // Home
        let home_rel = path_with_cwd.strip_prefix(homedir).map_err(|e| e.to_string())?;
        let home_rel = Path::new("~").join(home_rel);
        home_rel.into_os_string().into_string().map_err(|_| "OsString is invalid utf8".into())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    fn get_abs_home_path() -> PathBuf {
        let userdirs = UserDirs::new().unwrap();
        userdirs.home_dir().to_owned()
    }

    #[test]
    fn test_simple_pathstr() {
        let res = get_pathstr("yeah".to_string()).unwrap();
        // Assumption: source exists under home
        let expected =  {
            let expected_builder = env::current_dir().unwrap().join("yeah");
            let expected_builder = expected_builder.strip_prefix(get_abs_home_path()).unwrap();
            let expected_builder = expected_builder.to_str().unwrap();
            Path::new("~").join(expected_builder).to_str().unwrap().to_owned()
        };
        assert_eq!(res, expected);
    }
}