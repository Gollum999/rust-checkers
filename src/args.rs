use clap::{Arg, App};

#[derive(Copy, Clone, Debug)]
pub struct Args {
    pub debug: bool,
}

pub fn get_args() -> Args {
    let args = App::new("checkers")
        .version("1.0")
        .arg(Arg::with_name("debug").long("debug").help("Enable debug logging"))
        .get_matches();

    Args {
        debug: args.is_present("debug"),
    }
}
