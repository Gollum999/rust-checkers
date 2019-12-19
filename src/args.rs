use super::backend::args::{Args as BackendArgs};
use super::frontend::args::{ColorScheme, Args as FrontendArgs};
use clap::{Arg, App};

pub fn get_args() -> (BackendArgs, FrontendArgs) {
    let args = App::new("checkers")
        .version("1.0")
        .arg(Arg::with_name("ascii").long("ascii").help("Render pieces using only ASCII characters"))
        .arg(Arg::with_name("color-scheme").long("color-scheme").takes_value(true).possible_values(&ColorScheme::variants())
             .help("Color scheme (default: RedBlack)"))
        .get_matches();

    (BackendArgs {

    }, FrontendArgs {
        ascii: args.is_present("ascii"),
        color_scheme: value_t!(args.value_of("color-scheme"), ColorScheme).unwrap_or(ColorScheme::RedBlack),
    })
}
