mod themer;
use std::string::ToString;

fn main() {
    println!("Hello, world!");
    let cur_mode = themer::get_mode();
    //println!("{}", cur_mode.map_or_else(|e| match e {
    //        themer::ThemerError::UnknownMode => "unknown mode".to_owned(),
    //        themer::ThemerError::NoMode => "mode missing".to_owned(),
    //    }
    //, |v| v.to_string()));

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
        Err(_) => {
            eprintln!("Setting mode failed!");
        }
    };
}
