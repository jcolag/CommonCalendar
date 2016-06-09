use getopts::Options;
use std::env::Args;

pub struct Arguments {
    pub program_name: String,
    pub month: usize,
    pub year: usize,
    pub date_only: bool,
    pub exit: bool,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&*brief));
}

pub fn parse_args(arguments: Args) -> Arguments {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("d", "date", "only print today's date");
    opts.optopt("y", "year", "only print the year's calendar", "YEAR");
    opts.optopt("m", "month", "only print the month's calendar", "MONTH");

    let matches = match opts.parse(arguments) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut args = Arguments {
        program_name: "".to_string(),
        date_only: matches.opt_present("d"),
        month: 0,
        year: 0,
        exit: matches.opt_present("h"),
    };

    if args.exit {
        print_usage(&*args.program_name, opts);
    }

    args
}

