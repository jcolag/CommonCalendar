fn main() {
    let dow = ["Duinday", "Sitaday", "Wikiday", "Tuxday", "Gnuday", "Commonday"];
    let moy = ["Jabim", "Zodrak", "Trogool", "Yanar", "Shkumbe", "Habniah", "Skarl", "Mikon", "Pertunda", "Kib", "Broket"];
    let yoff = 0;
    let dpm = 33;
    let dpw = dow.len();
    let mpy = moy.len();
    
    for dd in 0..dpw {
        let mut abbrd = String::from(dow[dd]);
        abbrd.truncate(2);
        print!("{} ", abbrd);
    }
    
    for day in 1 .. dpm + 1 {
        if (day + yoff) % dpw == 1 {
            println!("");
        }

        print!("{:2} ", day);
    }

    println!("");
}
