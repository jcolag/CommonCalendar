extern crate getopts;
extern crate time;

use std::collections::LinkedList;
use std::str::Split;

mod args;

fn main() {
    let args: std::env::Args = std::env::args();
    let opts = args::parse_args(args);
    let dow = ["Duinday", "Sitaday", "Wikiday", "Tuxday", "Gnuday", "Commonday"];
    let moy = ["Jabim", "Zodrak", "Trogool", "Yanar", "Shkumbe", "Habniah", "Skarl", "Mikon", "Pertunda", "Kib", "Broket"];
    let dpm = 33;
    let dpw = dow.len();
    let mpy = moy.len();
    let (year, month, day, yoff) = current_day(time::now());
    let mut select_year = year;
    let mut select_yoff = yoff;
    if opts.has_year && opts.year < i16::max_value() {
        select_year = opts.year;
        let rem = select_year % 2;
        select_yoff = (rem + if rem < 0 { 2 } else { 0 }) as usize * 3;
    }

    let wd = (select_yoff + month * 3 + day - 1) % dpw;
    let mut moff = select_yoff;
    let mut months_across = LinkedList::<String>::new();

    if opts.exit {
        return;
    }
    
    if !opts.date_only {
        if opts.has_month {
            let mut select_month = month;
            if opts.month < usize::max_value() {
                select_month = opts.month;
            }

			let spaces = (17 - moy[select_month].len()) / 2;
			let mut month_text = format!("");
			for _ in 0..spaces {
			    month_text = format!("{} ", month_text);
			}
			
			month_text = format!("{}{}", month_text, moy[select_month]);
			for _ in 0..(18 - spaces - moy[select_month].len()) {
			    month_text = format!("{} ", month_text);
			}

			month_text = format!("{}\n", month_text);
			for dd in 0..dpw {
			    let mut abbrd = String::from(dow[dd]);
			    abbrd.truncate(2);
			    month_text = format!("{}{} ", month_text, abbrd);
			}

			let dd = if select_month == month { day } else { 1000 };
			let days = format_days(moff, dpm, dpw, dd);
			month_text = format!("{}{}", month_text, days.0);
			if select_month == 2 {
			    month_text = format!("{}--  Peer  Day  --", month_text);
			}
			else if select_month == 6 {
			    month_text = format!("{}--Torrent Feast--", month_text);
			}
			else if is_leap_year(select_year) && select_month == 10 {
			    month_text = format!("{}--Immersion Day--", month_text);
			}

			println!("{}", month_text);
        } else {
			for mm in 0..mpy {
				let spaces = (17 - moy[mm].len()) / 2;
				let mut month_text = format!("");
				for _ in 0..spaces {
				    month_text = format!("{} ", month_text);
				}

				month_text = format!("{}{}", month_text, moy[mm]);
				for _ in 0..(18 - spaces - moy[mm].len()) {
				    month_text = format!("{} ", month_text);
				}

				month_text = format!("{}\n", month_text);
				for dd in 0..dpw {
				    let mut abbrd = String::from(dow[dd]);
				    abbrd.truncate(2);
				    month_text = format!("{}{} ", month_text, abbrd);
				}

				let dd = if mm == month { day } else { 1000 };
				let days = format_days(moff, dpm, dpw, dd);
				month_text = format!("{}{}", month_text, days.0);
				moff = days.1;
				if mm == 2 {
				    month_text = format!("{}--  Peer  Day  --", month_text);
				}
				else if mm == 6 {
				    month_text = format!("{}--Torrent Feast--", month_text);
				}
				else if is_leap_year(select_year) && mm == 10 {
				    month_text = format!("{}--Immersion Day--", month_text);
				}

				month_text = format!("{}\n", month_text);
				months_across.push_back(month_text);
				if mm % 4 == 3 {
				    month_zipper(months_across.clone(), "   ", 1);
				    months_across.clear();
				}
			}

			month_zipper(months_across.clone(), "   ", 11);
        }
    }

    if !opts.has_year && !opts.has_month {
		let mut datestr = format!("{}, {:02} of {}", dow[wd], day, moy[month]);
		let mut daystr = format!("{:02}.{:02}", month + 1, day);
		if day == 34 {
			if month == 2 {
			    datestr = format!("Peer Day");
			    daystr = format!("P");
			}
			else if month == 6 {
			    datestr = format!("Torrent Feast");
			    daystr = format!("T");
			}
			else if month == 10 {
			    datestr = format!("Immersion Feast");
			    daystr = format!("T");
			}
		}

		let hol = format!("{}", what_holiday(month, day));
		println!("Today is {} {}{}.  [{}.{}]", datestr, year, hol, year, daystr);
    }
}

fn format_days(moff: usize, dpm: usize, dpw: usize, highlight: usize) -> (String, usize) {
    let mut month = format!("");

    if moff != 0 {
        month = format!("{}\n", month);
    }

    for _ in 0 .. moff {
        month = format!("{}   ", month);
    }

    for day in 1 .. dpm + 1 {
        if (day + moff) % dpw == 1 {
            month = format!("{}\n", month);
        }

        let ch = if day == highlight { ">" } else if day + 1 == highlight { "<" } else { " " };
        month = format!("{}{:2}{}", month, day, ch);
    }

    for _ in 0 .. (3 - moff) {
        month = format!("{}   ", month);
    }

    month = format!("{}\n", month);
    return (month, if moff == 0 { 3 } else { 0 });
}

fn current_day(current_time: time::Tm) -> (i16, usize, usize, usize) {
    let epoch_days_offset = 1483889;
    let seconds_per_day = 86400;
    let now = current_time.to_timespec();
    let days_since_epoch = (epoch_days_offset + now.sec / seconds_per_day) as i32;
    let days_per_year = 365;
    let leap_year = 4;
    let not_leap_year = 33;
    let days_per_month = 33;
    
    let mut count = 1;
    let mut days_remaining = days_since_epoch;
    let mut year = 0;
    while days_remaining > days_per_year + 1 {
        days_remaining -= days_per_year;
        year += 1;
        if count > not_leap_year {
            count = 1;
        }
        if count % leap_year == 0 {
            days_remaining -= 1;
        }
        count += 1;
    }
    if days_remaining == days_per_year + 1 && count % leap_year == 0 {
        days_remaining = 0;
        year += 1;
    }

    let mut month = 0;
    while days_remaining > days_per_month {
        month += 1;
        days_remaining -= days_per_month;
        if month == 3 || month == 7 || month == 11 {
            days_remaining -= 1;
        }
    }

    let day = days_remaining;
    return (year, month as usize, day as usize + 1, (year % 2 * 3) as usize);
}

fn is_leap_year(year: i16) -> bool {
    let rem = year % 33 + if year < 0 { 33 } else { 0 };
    return rem != 0 && rem % 4 == 0;
}

fn what_holiday(month: usize, day: usize) -> String {
    let mut result = format!("");
    if month == 0 && day == 1 {
        result = format!(", New Year's Day");
    }
    else if month == 2 && day == 12 {
        result = format!(", Purification Day");
    }
    else if month == 2 && day == 24 {
        result = format!(", Document Freedom Day");
    }
    else if month == 3 && day == 33 {
        result = format!(", Hardware Freedom Day");
    }
    else if month == 5 && day == 4 {
        result = format!(", Harvest Gift");
    }
    else if month == 5 && day == 9 {
        result = format!(", Culture Freedom Day");
    }
    else if month == 7 && day == 29 {
        result = format!(", Familytide");
    }
    else if month == 8 && day == 33 {
        result = format!(", Software Freedom Day");
    }
    else if month == 9 && day == 30 {
        result = format!(", Open Access Day");
    }
    else if month == 10 && day == 21 {
        result = format!(", Freedom Day");
    }
    else if month == 0 && day == 27 {
        result = format!(", International Workers' Day");
    }
    return result;
}

fn month_zipper(months: LinkedList<String>, buffer: &str, front: usize) {
    let month_iterator = months.iter();
    let mut length = 0;
    let mut split_months = LinkedList::<Split<&str>>::new();
    for m in month_iterator {
        let split = m.split("\n");
        if split.clone().count() > length {
            length = split.clone().count();
        }
        split_months.push_back(split);
    }

    for i in 0..(length - 1) {
        for _ in 0..front {
            print!(" ");
        }

        let split_iterator = split_months.iter();
        let mut full_line = format!("");
        for m in split_iterator {
            let line = m.clone().nth(i);
            match line {
                Some(l) => {
                    let part = if l == "" { "                  " } else { l };
                    full_line = format!("{}{}{}", full_line, part, buffer);
                },
                None => {},
            }
        }

        println!("{}", full_line.trim_right());
    }
}

