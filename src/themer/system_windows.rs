use std::process::Command;
use super::Mode;

// Should be called on every system
// OS conditional is done in the function
pub fn set(mode: &Mode) {
    if (env!("IS_WSL") == "true") || cfg!(target_os = "windows") {
        Command::new("reg.exe")
            .args(&["add", r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize", "/v", "AppsUseLightTheme", "/t", "REG_DWORD", "/d", if mode == &Mode::Day {"1"} else {"0"}, "/f"]).output().expect("Failed to modify windows apps theme.");
        Command::new("reg.exe")
            .args(&["add", r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize", "/v", "SystemUsesLightTheme", "/t", "REG_DWORD", "/d", if mode == &Mode::Day {"1"} else {"0"}, "/f"]).output().expect("Failed to modify windows system theme.");
    }
}
