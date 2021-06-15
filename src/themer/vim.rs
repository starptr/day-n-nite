use super::ModuleError;
use lazy_static::*;
use regex::RegexSet;
use sysinfo::{self, ProcessExt, Signal, SystemExt};

pub fn update() -> Result<(), ModuleError> {
    lazy_static! {
        static ref NVP_HEADLESS_CMD_RE: Vec<RegexSet> = [
            RegexSet::new(&[r"^/tmp/.*/nvim$"]).unwrap(),
            RegexSet::new(&[r"^--headless$"]).unwrap(),
            RegexSet::new(&[r"^--listen$"]).unwrap(),
            RegexSet::new(&[r"^localhost:\d+$"]).unwrap(),
        ]
        .to_vec();
        static ref TUI_NON_HEADLESS_RE: Vec<RegexSet> =
            [RegexSet::new(&[r"^/tmp/.*/nvim$", r"^nvim$", r"^vim$",]).unwrap()].to_vec();
    }
    let mut sys = sysinfo::System::new();
    sys.refresh_processes();
    sys.get_processes().iter().for_each(|(_pid, process)| {
        let command = process.cmd();
        if is_match_n(command, NVP_HEADLESS_CMD_RE.to_vec())
            || is_match_n(command, TUI_NON_HEADLESS_RE.to_vec())
        {
            process.kill(Signal::User1);
        };
    });
    Ok(())
}

fn is_match_n(command: &[String], first_n_re: Vec<RegexSet>) -> bool {
    command.len() >= first_n_re.len()
        && first_n_re
            .iter()
            .enumerate()
            .all(|(index, re)| re.is_match(&command[index]))
}
