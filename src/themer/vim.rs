use super::ModuleError;
use lazy_static::*;
use regex::{Regex, RegexSet};
use sysinfo::{self, ProcessExt, Signal, SystemExt};

#[derive(Clone)]
enum RegexList {
    Singular(Regex),
    Multiple(RegexSet),
}

pub fn update() -> Result<(), ModuleError> {
    lazy_static! {
        static ref NVP_HEADLESS_CMD_RE: Vec<RegexList> = [
            RegexList::Singular(Regex::new(r"^/tmp/.*/nvim$").unwrap()),
            RegexList::Singular(Regex::new(r"^--headless$").unwrap()),
            RegexList::Singular(Regex::new(r"^--listen$").unwrap()),
            RegexList::Singular(Regex::new(r"^localhost:\d+$").unwrap()),
        ]
        .to_vec();
        static ref TUI_NON_HEADLESS_RE: Vec<RegexList> =
            [RegexList::Multiple(RegexSet::new(&[r"^/tmp/.*/nvim$", r"^nvim$", r"^vim$",]).unwrap())].to_vec();
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

fn is_match_n(command: &[String], first_n_re: Vec<RegexList>) -> bool {
    command.len() >= first_n_re.len()
        && first_n_re
            .iter()
            .enumerate()
            .all(|(index, re)| match &re {
                &RegexList::Singular(re) => re.is_match(&command[index]),
                &RegexList::Multiple(re) => re.is_match(&command[index]),
            })
}
