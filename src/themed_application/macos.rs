use super::*;

pub struct MacosSystemTheme;

impl ThemedSystem for MacosSystemTheme {
    fn set_theme(&mut self, theme: Theme) -> Result<(), String> where Self: Sized {
        use std::process::Command;
        let is_dark = theme == Theme::Dark;
        let res = Command::new("sh")
            .arg("-c")
            .arg(format!("osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to {}'", is_dark))
            .output()
            .map(|_| ())
            .map_err(|_| "Process to set system theme failed".into());
        res
    }

    fn into_job(mut self, theme: Theme) -> Box<(dyn FnOnce() -> Result<(), std::string::String> + 'static)> {
        Box::new(move || {
            self.set_theme(theme)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_set() -> Result<(), String> {
        use crate::system_info::macos::MacosSystemInfo;
        use crate::system_info::SystemInfo;
        let mac_sys_info = MacosSystemInfo {};
        let original_theme = mac_sys_info.get_theme();
        let mut mac_sys_themer = MacosSystemTheme {};
        mac_sys_themer.set_theme(!original_theme)?;
        assert_eq!(!original_theme, mac_sys_info.get_theme());
        mac_sys_themer.set_theme(original_theme)?;
        assert_eq!(original_theme, mac_sys_info.get_theme());
        Ok(())
    }

    #[test]
    fn into_job() -> Result<(), String> {
        use crate::system_info::macos::MacosSystemInfo;
        use crate::system_info::SystemInfo;
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
