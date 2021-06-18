mod cli;
mod themer;
use std::string::ToString;

fn main() {
    let arg_matches = cli::get_app().get_matches();

    let target_mode = if arg_matches.is_present("daymode") {
        Ok(themer::Mode::Day)
    } else if arg_matches.is_present("nightmode") {
        Ok(themer::Mode::Night)
    } else {
        themer::get_mode().map(|mode| themer::toggle(mode))
    };

    let set_result = match target_mode {
        Ok(mode) => themer::set_mode(mode, arg_matches),
        Err(themer::GetError::NoMode) => themer::set_mode(themer::Mode::Day, arg_matches),
        Err(themer::GetError::UnknownMode) => themer::set_mode(themer::Mode::Day, arg_matches),
    };

    match set_result {
        Ok(mode) => {
            println!("Set to {} Mode.", mode.to_string());
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    };
}
