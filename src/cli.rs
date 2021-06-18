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
}
