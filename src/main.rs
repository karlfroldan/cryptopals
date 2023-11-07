#![feature(iter_array_chunks)]
#![feature(ascii_char)]
#![feature(array_chunks)]
mod encoding;
mod simple_xor;
mod set1;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    // I want to skip the program name.
    let mut args = std::env::args();
    _ = args.next();

    if let Some(prog) = args.next() {
        let progn = prog.as_str().parse::<usize>().unwrap();
        let now = Instant::now();

        match progn {
            1 => set1::challenge1(&mut args),
            2 => set1::challenge2(&mut args),
            3 => set1::challenge3(&mut args),
            4 => set1::challenge4(&mut args),
            5 => set1::challenge5(&mut args),
            _ => println!("Not a valid program!"),
        }

        let elapsed = now.elapsed();
        if elapsed.as_micros() <= 10000 {
            println!("Execution: {} us", elapsed.as_micros());
        } else {
            println!("Execution: {} ms", elapsed.as_millis());
        }
    }

    Ok(())
}
