use crate::dnn::Theme;

/*
Retrieves the OS theme
*/
pub fn get_system_theme() -> Theme {
    use std::process::Command;
    if cfg!(target_os = "macos") {
        const DARK: &str = "Dark\n";

        let output = Command::new("sh")
            .arg("-c")
            .arg("defaults read -g AppleInterfaceStyle")
            .output()
            .expect("Process to get system theme failed");
        let output = String::from_utf8(output.stdout).unwrap();

        if output == DARK {
            Theme::Dark
        } else {
            Theme::Light
        }
    } else {
        panic!("Not implemented for current OS")
    }
}

pub fn set_system_theme(theme: Theme) -> Result<(), String> {
    use std::process::Command;
    if cfg!(target_os = "macos") {
        let is_dark = theme == Theme::Dark;
        let res = Command::new("sh")
            .arg("-c")
            .arg(format!("osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to {}'", is_dark))
            .output()
            .map(|_| ())
            .map_err(|_| "Process to set system theme failed".into());
        res
    } else {
        panic!("Not implemented for current OS")
    }
}
