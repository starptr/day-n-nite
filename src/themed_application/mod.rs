mod macos;
mod application_script;

use std::{str::FromStr, path::Path};
use strum_macros::EnumString;

use toml::Value;

use super::*;

/// Abstract interface representing a set of objects that can be themed
/// Examples include:
/// - MacOS
/// - Windows
/// - WSL
/// - ApplicationScript
/// - ApplicationConfig
/// - ApplicationPair
/// - ApplicationCommand
pub trait Themeable {
    fn into_job(self, theme: Theme) -> Box<dyn FnOnce() -> Result<(), String>>;
}

#[derive(EnumString)]
pub enum ConfigurableThemedSystemKind {
    Script,
    Config,
    Pair,
    Command,
}

/// Abstract interface for a ThemedSystem that can be configured
/// Inlcudes the `Application*` themed systems
pub trait ConfigurableThemedSystem: Themeable {
    fn get_path(&self) -> &Path;

    // TODO: uncomment this method
    //fn get_hash(&self);

    fn get_kind(&self) -> ConfigurableThemedSystemKind;
}

/// Abstract interface (thinly) wrapping toml for ConfigurableThemedSystem
pub trait RawConfigurableThemedSystemTrait {
    fn new(str: &str) -> Result<Self, toml::de::Error> where Self: Sized;

    fn get_kind(&self) -> Result<ConfigurableThemedSystemKind, strum::ParseError>;

    fn into_rich(self) -> Box<dyn ConfigurableThemedSystem>;
}

pub struct RawConfigurableThemedSystem {
    value: Value,
}

impl RawConfigurableThemedSystemTrait for RawConfigurableThemedSystem {
    fn new(str: &str) -> Result<Self, toml::de::Error> {
        str.parse::<Value>().map(|value| RawConfigurableThemedSystem{ value })
    }

    fn get_kind(&self) -> Result<ConfigurableThemedSystemKind, strum::ParseError> {
        ConfigurableThemedSystemKind::from_str(self.value["kind"].as_str().unwrap_or_default())
    }

    fn into_rich(self) -> Box<dyn ConfigurableThemedSystem> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::system_info::tests::SimpleSystemInfo;

    #[test]
    fn raw_to_rich() {
        let raw = RawConfigurableThemedSystem::new(str)
    }

    impl Themeable for SimpleSystemInfo {
        fn into_job(mut self, theme: Theme) -> Box<(dyn FnOnce() -> Result<(), std::string::String> + 'static)> {
            Box::new(move || {
                self.theme = theme;
                Ok(())
            })
        }
    }

    #[test]
    fn into_job() {
        let simple_sys_info = SimpleSystemInfo::new(Theme::Light);
        let job = simple_sys_info.into_job(Theme::Dark);
        assert!(job().is_ok());
    }

    #[test]
    fn parse() {
    }
}
