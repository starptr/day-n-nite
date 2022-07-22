use super::*;

pub struct MacosSystemInfo;

impl SystemInfo for MacosSystemInfo {
    fn get_theme(&self) -> Theme {
        use std::process::Command;
        let output = Command::new("sh")
            .arg("-c")
            .arg("defaults read -g AppleInterfaceStyle")
            .output()
            .expect("Process to get system theme failed");
        let output = String::from_utf8(output.stdout).unwrap();

        const DARK: &str = "Dark\n";
        if output == DARK {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;

    #[test]
    fn no_crash() {
        let macos_sys_info = MacosSystemInfo {};
        macos_sys_info.get_theme();
    }
}
