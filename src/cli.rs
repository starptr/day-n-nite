use clap::{App, Arg};

pub fn get_app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("no_emit")
                .help("Whether to emit calls to other systems")
                .short("m")
                .long("no-emit"),
        )
        .arg(
            Arg::with_name("daymode")
                .help("Force day mode")
                .short("d")
                .long("day"),
        )
        .arg(
            Arg::with_name("nightmode")
                .help("Force night moode")
                .short("n")
                .long("night")
                .conflicts_with("daymode"),
        )
}
