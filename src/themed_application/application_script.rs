use super::*;
use std::path::PathBuf;

pub struct ApplicationScript {
    path: PathBuf,
    env_var: Option<String>,
    flag: (), // TODO:
    should_pipe: (), // TODO:
}

impl Themeable for ApplicationScript {
    fn into_job(self, theme: Theme) -> Box<dyn FnOnce() -> Result<(), String>> {
        Box::new(move || {
            use crate::std_wrapper::command::{Command, CommandTrait};
            let is_dark = theme == Theme::Dark;
            let mut cmd = Command::cmd(self.path);
            let cmd = if self.env_var.is_none() {
                &mut cmd
            } else {
                cmd.env(self.env_var.unwrap(), if is_dark { "dark" } else { "light" })
            };
            let res = cmd.output()
                .map(|_| ())
                .map_err(|e| format!("{:}", e));
            res
        })
    }
}

impl ConfigurableThemedSystem for ApplicationScript {
    fn get_kind(&self) -> ConfigurableThemedSystemKind {
        ConfigurableThemedSystemKind::Script
    }

    fn get_path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
