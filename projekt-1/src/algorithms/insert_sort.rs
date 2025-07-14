use owo_colors::OwoColorize;
use std::time::Instant;

pub fn run<T: Copy + std::cmp::PartialOrd + std::fmt::Debug>(
    mut data: Vec<T>,
    benchmarking: bool,
) -> f64 {
    let start_time = Instant::now();
    let n = data.len();
    for i in 1..n {
        let key = data[i];
        let mut j = i;
        while j > 0 && data[j - 1] > key {
            data[j] = data[j - 1];
            j -= 1;
        }
        data[j] = key;
    }
    let elapsed_ns = start_time.elapsed().as_nanos();
    let elapsed = elapsed_ns as f64 / 1_000_000.0;
    if benchmarking {
        println!("Time: {}", elapsed.cyan());
    } else {
        println!("Sorted:{:?}\nTime elapsed: {:?}ms", data, elapsed.cyan());
    }

    elapsed
}
