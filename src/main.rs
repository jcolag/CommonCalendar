fn main() {
    let dow = ["Duinday", "Sitaday", "Wikiday", "Tuxday", "Gnuday", "Commonday"];
    let moy = ["Jabim", "Zodrak", "Trogool", "Yanar", "Shkumbe", "Habniah", "Skarl", "Mikon", "Pertunda", "Kib", "Broket"];
    let yoff: usize = 0;
    let dpm = 33;
    let dpw = dow.len();
    let mpy = moy.len();
    let mut moff = yoff;

    for dd in 0..dpw {
        let mut abbrd = String::from(dow[dd]);
        abbrd.truncate(2);
        print!("{} ", abbrd);
    }

    moff = print_days(moff, dpm, dpw);
}

fn print_days(moff: usize, dpm: usize, dpw: usize) -> usize {
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

        print!("{:2} ", day);
    }

    println!("");
    return if moff == 0 { 3 } else { 0 };
}

