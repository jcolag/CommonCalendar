use getopts::Options;
use std::env::Args;

pub struct Arguments {
    pub program_name: String,
    pub has_month: bool,
    pub month: usize,
    pub has_year: bool,
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
    opts.optflag("d", "date", "only print today's date");
    opts.optflag("h", "help", "print this help menu");
    opts.optflagopt("m", "month", "only print the month's calendar", "MONTH");
    opts.optflagopt("y", "year", "only print the year's calendar", "YEAR");

    let matches = match opts.parse(arguments) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut args = Arguments {
        program_name: "".to_string(),
        date_only: matches.opt_present("d"),
        has_month: matches.opt_present("m"),
        month: match matches.opt_str("m") {
            Some(m) => match m.parse::<usize>() {
                Ok(mm) => if mm == 0 { usize::max_value() } else { mm - 1 },
                Err(_) => usize::max_value(),
            },
            None => usize::max_value(),
        },
        has_year: matches.opt_present("y"),
        year: match matches.opt_str("y") {
            Some(y) => match y.parse::<usize>() {
                Ok(mm) => mm,
                Err(_) => usize::max_value(),
            },
            None => usize::max_value(),
        },
        exit: matches.opt_present("h"),
    };
    
    if args.month > 10 {
        args.has_month = false;
        args.month = usize::max_value();
    }

    if args.exit {
        print_usage(&*args.program_name, opts);
    }

    args
}

