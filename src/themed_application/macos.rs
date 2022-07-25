use super::*;

use std::sync::Mutex;
use once_cell::sync::Lazy;

static MACOS_SYSTEM_THEME: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

pub struct MacosSystemTheme;

impl Themeable for MacosSystemTheme {
    fn into_job(mut self, theme: Theme) -> Box<(dyn FnOnce() -> Result<(), std::string::String> + 'static)> {
        Box::new(move || {
            use std::process::Command;
            let is_dark = theme == Theme::Dark;
            let res = Command::new("sh")
                .arg("-c")
                .arg(format!("osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to {}'", is_dark))
                .output()
                .map(|_| ())
                .map_err(|_| "Process to set system theme failed".into());
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_job() -> Result<(), String> {
        use crate::system_info::macos::MacosSystemInfo;
        use crate::system_info::SystemInfo;
        let _shared = MACOS_SYSTEM_THEME.lock().map_err(|e| format!("{:}", e))?;
        let mac_sys_info = MacosSystemInfo {};
        let original_theme = mac_sys_info.get_theme();
        let to_opposite = (MacosSystemTheme {}).into_job(!original_theme);
        let to_original = (MacosSystemTheme {}).into_job(original_theme);
        to_opposite()?;
        assert_eq!(!original_theme, mac_sys_info.get_theme());
        to_original()?;
        assert_eq!(original_theme, mac_sys_info.get_theme());
        Ok(())
    }
}
