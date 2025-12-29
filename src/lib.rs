pub mod huffman;
pub mod bits;
pub mod queue;

use std::time::Instant;


fn print_time(label: &str, start: Instant) {
    let nanos = start.elapsed().as_nanos();

    match nanos {
        n if n < 1_000 => println!("{}ns\t{}", n, label),
        n if n < 1_000_000 => println!("{:.0}µs\t{}", n as f64 / 1_000.0, label),
        n if n < 1_000_000_000 => println!("{:.0}ms\t{}", n as f64 / 1_000_000.0, label),
        n => println!("{:.2}s\t{}", n as f64 / 1_000_000_000.0, label),
    }
}