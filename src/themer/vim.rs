use super::ModuleError;
use lazy_static::*;
use regex::Regex;
use sysinfo::{self, ProcessExt, Signal, SystemExt};

pub fn update() -> Result<(), ModuleError> {
    lazy_static! {
        static ref VIM_HEADLESS_CMD_RE: [Regex; 4] = [
            Regex::new(r"^/tmp/.*/nvim$").unwrap(),
            Regex::new(r"^--headless$").unwrap(),
            Regex::new(r"^--listen$").unwrap(),
            Regex::new(r"^localhost:\d+$").unwrap(),
        ];
    }
    let mut sys = sysinfo::System::new();
    sys.refresh_processes();
    sys.get_processes().iter().for_each(|(_pid, process)| {
        let command = process.cmd();
        if VIM_HEADLESS_CMD_RE
            .iter()
            .enumerate()
            .all(|(index, re)| re.is_match(&command[index]))
        {
            process.kill(Signal::User1);
        };
    });
    Ok(())
}
