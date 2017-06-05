use std::{env, io, process};
use std::io::Write;

fn main() {
    let mut i: u64 = env::args().nth(1).unwrap_or_else(|| {
        writeln!(io::stderr(), "[!] Please supply starting number!").unwrap();
        process::exit(1);
    }).parse().unwrap();

//    let mut i = 36;
    let mut count = 0;

    loop {
        if i == 1 {
            break;
        }

        count += 1;

        if i % 2 == 0 {
            i /= 2;
        } else {
            i = (i * 3) + 1;
        }
    }

    println!("Steps: {}", count);
}
