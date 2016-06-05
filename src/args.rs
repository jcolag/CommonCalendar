use getopts::Options;
use std::env::Args;

pub struct Arguments {
    pub program_name: String,
    pub exit: bool,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&*brief));
}

pub fn parse_args(arguments: Args) -> Arguments {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(arguments) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut args = Arguments {
        program_name: "".to_string(),
        exit: matches.opt_present("h"),
    };

    if args.exit {
        print_usage(&*args.program_name, opts);
    }

    args.entries = match matches.opt_str("n") {
        Some(s) => match (&*s).parse() {
            Ok(x) => Some(x),
            Err(_) => None,
        },
        None => None,
    };

    args.filename = matches.opt_str("o");

    args.port = match matches.opt_str("s") {
        Some(s) => match (&*s).parse() {
            Ok(x) => Some(x),
            Err(_) => None,
        },
        None => None,
    };

    args
}

