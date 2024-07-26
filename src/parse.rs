use clap::{arg, value_parser, Arg, ArgMatches, Command};

pub(super) fn parse() -> ArgMatches {
    Command::new("collatz")
        .arg(
            Arg::new("n")
                .short('n')
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("start")
                .long("start")
                .short('s')
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("end")
                .long("end")
                .short('e')
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(arg!(--table))
        .arg(arg!(--cycle))
        .get_matches()
}
