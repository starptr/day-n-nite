mod themer;
mod cli;
use std::string::ToString;

fn main() {
    let arg_matches = cli::get_app().get_matches();

    let cur_mode = themer::get_mode();

    let set_result = match cur_mode {
        Ok(mode) => themer::set_mode(themer::toggle(mode), arg_matches),
        Err(themer::GetError::NoMode) => themer::set_mode(themer::Mode::Day, arg_matches),
        Err(themer::GetError::UnknownMode) => themer::set_mode(themer::Mode::Day, arg_matches),
    };

    match set_result {
        Ok(mode) => {
            println!("Set to {} Mode.", mode.to_string());
        },
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    };
}
