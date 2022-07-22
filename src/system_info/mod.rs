pub mod macos;

use std::path::PathBuf;

use crate::themed_application::ThemedSystem;

use super::*;

/// Abstract interface representing an operating system's read-only properties
/// Used to allow injection for dependents' tests and OS-specific code
pub trait SystemInfo {
    fn get_theme(&self) -> Theme;

    /// Returns the location of dnn's "config" file
    /// This is actually a data file since 1) it should not be manually edited and 2) there are human-unfriendly attributes
    fn get_dnn_data_path(&self) -> PathBuf {
        use directories::ProjectDirs;
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
}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub struct SimpleSystemInfo {
        pub theme: Theme,
    }
    
    impl SimpleSystemInfo {
        pub fn new(theme: Theme) -> SimpleSystemInfo {
            SimpleSystemInfo { theme }
        }
    }
    
    impl SystemInfo for SimpleSystemInfo {
        fn get_theme(&self) -> Theme {
            self.theme
        }
    
        fn get_dnn_data_path(&self) -> PathBuf {
            // Mock system for unit testing does not have a file system
            unimplemented!();
        }
    }
    
    #[test]
    fn theme() {
        let light = Theme::Light;
        let simple_sys_info = SimpleSystemInfo::new(light);
        assert_eq!(light, simple_sys_info.get_theme());
    }
}
