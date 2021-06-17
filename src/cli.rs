use clap::{App, Arg, ArgMatches};

pub fn get_app() -> App<'static, 'static> {
    App::new("day-n-nite")
        .version("0.1.0")
        .author("Yuto Nishida")
        .about("Global light/dark mode toggler")
        .arg(
            Arg::with_name("no_emit")
                .help("Whether to emit calls to other systems")
                .short("m")
                .long("no-emit"),
        )
}
