mod macos;

use super::*;

/// Abstract interface representing a set of objects that can be themed
/// Examples include:
/// - MacOS
/// - Windows
/// - Applications
pub trait ThemedSystem {
    fn set_theme(&mut self, theme: Theme) -> Result<(), String> where Self: Sized;

    fn into_job(self, theme: Theme) -> Box<dyn FnOnce() -> Result<(), String>>;
}

pub struct AppThemer {
    kind: DnnEntryKind,
}
impl AppThemer {
    fn set_theme(&self, theme: Theme) -> Result<(), String> where Self: Sized {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::system_info::tests::SimpleSystemInfo;
    impl ThemedSystem for SimpleSystemInfo {
        fn set_theme(&mut self, theme: Theme) -> Result<(), String> where Self: Sized {
            self.theme = theme;
            Ok(())
        }

        fn into_job(mut self, theme: Theme) -> Box<(dyn FnOnce() -> Result<(), std::string::String> + 'static)> {
            Box::new(move || {
                self.set_theme(theme)?;
                Ok(())
            })
        }
    }

    #[test]
    fn theme_set() {
        use crate::system_info::SystemInfo;
        let mut simple_sys_info = SimpleSystemInfo::new(Theme::Light);
        assert!(simple_sys_info.set_theme(Theme::Dark).is_ok());
        assert_eq!(simple_sys_info.get_theme(), Theme::Dark);
    }

    #[test]
    fn into_job() {
        let simple_sys_info = SimpleSystemInfo::new(Theme::Light);
        let job = simple_sys_info.into_job(Theme::Dark);
        assert!(job().is_ok());
    }
}
