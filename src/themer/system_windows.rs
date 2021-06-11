use std::process::Command;
use super::Mode;
use super::SetError;

// Should be called on every system
// OS conditional is done in the function
pub fn set(mode: Mode) -> Result<(), SetError> {
    if (env!("IS_WSL") == "true") || cfg!(target_os = "windows") {
        Command::new("reg.exe")
            .args(&["add", r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize", "/v", "AppsUseLightTheme", "/t", "REG_DWORD", "/d", if mode == Mode::Day {"1"} else {"0"}, "/f"]).output().map_err(|_| SetError::RegEditFailure)?;
        Command::new("reg.exe")
            .args(&["add", r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize", "/v", "SystemUsesLightTheme", "/t", "REG_DWORD", "/d", if mode == Mode::Day {"1"} else {"0"}, "/f"]).output().map_err(|_| SetError::RegEditFailure)?;
    }
    Ok(())
}