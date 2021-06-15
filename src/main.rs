mod themer;
use std::string::ToString;

fn main() {
    let cur_mode = themer::get_mode();

    let set_result = match cur_mode {
        Ok(mode) => match mode {
            themer::Mode::Day => themer::set_night(),
            themer::Mode::Night => themer::set_day(),
        },
        Err(themer::GetError::NoMode) => themer::set_day(),
        Err(themer::GetError::UnknownMode) => themer::set_day(),
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
