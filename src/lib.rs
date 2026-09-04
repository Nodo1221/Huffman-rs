pub mod huffman;
pub mod bits;
pub mod queue;
mod config;

use std::time::Instant;

#[allow(dead_code)]
fn print_time(label: &str, start: Instant) {
    // #[cfg(debug_assertions)]
    {
        let nanos = start.elapsed().as_nanos();
    
        match nanos {
            n if n < 1_000 => eprintln!("{}ns\t{}", n, label),
            n if n < 1_000_000 => eprintln!("{:.0}µs\t{}", n as f64 / 1_000.0, label),
            n if n < 1_000_000_000 => eprintln!("{:.0}ms\t{}", n as f64 / 1_000_000.0, label),
            n => eprintln!("{:.2}s\t{}", n as f64 / 1_000_000_000.0, label),
        }
    }
}

pub fn print_throughput(label: &str, bytes: usize, elapsed: std::time::Duration) {
    let secs = elapsed.as_secs_f64();
    if secs == 0.0 { return; }
    let bps = bytes as f64 / secs;

    let (val, unit) = match bps {
        b if b < 1_024.0 => (b, "B/s"),
        b if b < 1_048_576.0 => (b / 1_024.0, "KB/s"),
        b if b < 1_073_741_824.0 => (b / 1_048_576.0, "MB/s"),
        b => (b / 1_073_741_824.0, "GB/s"),
    };

    eprintln!("{:.1} {}\t{} ({} bytes)", val, unit, label, bytes);
}