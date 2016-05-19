extern crate time;

fn main() {
    let dow = ["Duinday", "Sitaday", "Wikiday", "Tuxday", "Gnuday", "Commonday"];
    let moy = ["Jabim", "Zodrak", "Trogool", "Yanar", "Shkumbe", "Habniah", "Skarl", "Mikon", "Pertunda", "Kib", "Broket"];
    let dpm = 33;
    let dpw = dow.len();
    let mpy = moy.len();
    let (year, month, day, yoff) = current_day(time::now());
    let wd = (yoff + month * 3 + day) % dpw;
    let mut moff = yoff;

    for mm in 0..mpy {
        let spaces = (17 - moy[mm].len()) / 2;
        for _ in 0..spaces {
            print!(" ");
        }
        println!("{}", moy[mm]);
        
        for dd in 0..dpw {
            let mut abbrd = String::from(dow[dd]);
            abbrd.truncate(2);
            print!("{} ", abbrd);
        }

        let dd = if mm == month { day } else { 1000 };
        moff = print_days(moff, dpm, dpw, dd);
        if mm == 2 {
            println!("-- Peer Day --");
        }
        else if mm == 6 {
            println!("-- Torrent Feast --");
        }
        else if is_leap_year(year) && mm == 10 {
            println!("-- Immersion Feast --");
        }
    }

    println!("Today is {}, {:02} of {} {}.  [{}.{:02}.{:02}]", dow[wd], day, moy[month], year, year, month + 1, day);
}

fn print_days(moff: usize, dpm: usize, dpw: usize, highlight: usize) -> usize {
    if moff != 0 {
        println!("");
    }
    
    for _ in 0 .. moff {
        print!("   ");
    }
    
    for day in 1 .. dpm + 1 {
        if (day + moff) % dpw == 1 {
            println!("");
        }

        let ch = if day == highlight { ">" } else if day + 1 == highlight { "<" } else { " " };
        print!("{:2}{}", day, ch);
    }

    println!("");
    return if moff == 0 { 3 } else { 0 };
}

fn current_day(current_time: time::Tm) -> (usize, usize, usize, usize) {
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
    
    let month = days_remaining / days_per_month;
    let day = days_remaining - month * days_per_month;
    return (year, month as usize, day as usize + 1, (year % 2 * 3) as usize);
}

fn is_leap_year(year: usize) -> bool {
    let rem = year % 33;
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
    else if month == 5 && day == 4 {
        result = format!(", Harvest Gift");
    }
    else if month == 7 && day == 29 {
        result = format!(", Familytide");
    }
    else if month == 10 && day == 21 {
        result = format!(", Freedom Day");
    }
    else if month == 0 && day == 27 {
        result = format!(", International Workers' Day");
    }
    return result;
}

