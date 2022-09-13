use clap::{Arg, Command};

fn app() -> Command<'static> {
    Command::new("fli")
        .version("1.0.0")
        .author("flucium")
        .args(vec![
            Arg::new("all")
                .long("all")
                .short('a')
                .required(false)
                .takes_value(false),
            Arg::new("format")
                .long("format")
                .short('f')
                .required(false)
                .takes_value(true),
            Arg::new("count")
                .long("count")
                .short('c')
                .required(false)
                .takes_value(false),
        ])
}

fn encode(mut string: String) {}

fn main() {}
