use super::*;

use std::{io, ffi::OsStr, process};

pub trait CommandTrait {
    fn cmd<S: AsRef<OsStr>>(program: S) -> Self;

    fn env<K, V>(&mut self, key: K, val: V) -> &mut Self where K: AsRef<OsStr>, V: AsRef<OsStr>;

    fn output(&mut self) -> io::Result<process::Output>;
}

pub struct Command {
    inner: process::Command,
}

impl CommandTrait for Command {
    fn cmd<S: AsRef<OsStr>>(program: S) -> Self {
        Command { inner: process::Command::new(program) }
    }

    fn env<K, V>(&mut self, key: K, val: V) -> &mut Self where K: AsRef<OsStr>, V: AsRef<OsStr> {
        self.inner.env(key, val);
        self
    }

    fn output(&mut self) -> io::Result<process::Output> {
        self.inner.output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::OsString;
    use once_cell::sync::Lazy;

    #[test]
    fn no_crash() {
        let mut cmd = Command::cmd("echo");
        let _output = cmd.output().unwrap();
    }

    pub struct SimpleCommand {
        path: OsString,
        env: Option<(OsString, OsString)>,
    }

    static PROCESS_OK_OUTPUT: Lazy<process::Output> = Lazy::new(|| {
        let res = process::Command::new("echo").output().unwrap();
        res
    });

    impl CommandTrait for SimpleCommand {
        fn cmd<S: AsRef<OsStr>>(program: S) -> Self {
            SimpleCommand { path: program.as_ref().into(), env: None }
        }

        fn env<K, V>(&mut self, key: K, val: V) -> &mut Self where K: AsRef<OsStr>, V: AsRef<OsStr> {
            self.env = Some((key.as_ref().into(), key.as_ref().into()));
            self
        }

        fn output(&mut self) -> io::Result<process::Output> {
            Ok(PROCESS_OK_OUTPUT.to_owned())
        }
    }

    #[test]
    fn simple_no_crash() {
        let mut simple_cmd = SimpleCommand::cmd("test");
        simple_cmd.env("key", "val");
        let _output = simple_cmd.output().unwrap();
    }
}
