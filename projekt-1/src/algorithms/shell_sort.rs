// https://oeis.org/A102549
// https://oeis.org/A003462

use owo_colors::OwoColorize;
use std::time::Instant;

pub fn run<T: std::fmt::Debug + std::clone::Clone + std::cmp::PartialOrd>(
    data: Vec<T>,
    choice: i64,
    benchmarking: bool,
) -> f64 {
    let sorted_data: Vec<T>;
    let elapsed_ns;
    let elapsed_time: f64;
    if choice == 1 {
        let start_time = Instant::now();
        sorted_data = marcin_ciura(data.clone());
        elapsed_ns = start_time.elapsed().as_nanos();
        elapsed_time = elapsed_ns as f64 / 1_000_000.0;
    } else if choice == 2 {
        let start_time = Instant::now();
        sorted_data = knuth(data.clone());
        elapsed_ns = start_time.elapsed().as_nanos();
        elapsed_time = elapsed_ns as f64 / 1_000_000.0;
    } else {
        println!("{}", "[-] Error".red());
        return 0.0;
    }
    if benchmarking {
        println!("Time: {}", elapsed_time.cyan());
    } else {
        println!(
            "Sorted:{:?}\nTime elapsed: {:?}ms",
            sorted_data,
            elapsed_time.cyan()
        );
    }
    elapsed_time
}

fn shell_sort<T: std::cmp::PartialOrd + std::clone::Clone>(
    mut data: Vec<T>,
    seq: Vec<i64>,
) -> Vec<T> {
    let length = data.len();
    for gap in seq {
        for i in gap..length as i64 {
            let temp = data[i as usize].clone();
            let mut j = i;
            while j >= gap && data[(j - gap) as usize] > temp {
                data[j as usize] = data[(j - gap) as usize].clone();
                j -= gap;
            }
            data[j as usize] = temp.clone();
        }
    }
    data
}

fn marcin_ciura<T: std::clone::Clone + std::cmp::PartialOrd>(data: Vec<T>) -> Vec<T> {
    let ciura_seq = [1750, 701, 301, 132, 57, 23, 10, 4, 1];
    shell_sort(data.clone(), ciura_seq.to_vec())
}

fn knuth<T: std::clone::Clone + std::cmp::PartialOrd>(data: Vec<T>) -> Vec<T> {
    // a(n) = (3^n - 1)/2
    let mut a_n: i64 = 0;
    let mut n: u32 = 0;
    let mut seq: Vec<i64> = Vec::new();
    while a_n < data.len() as i64 {
        n += 1;
        a_n = (3_i64.pow(n as u32) - 1) / 2;
        seq.push(a_n);
        // println!("Data: {:?}", seq.yellow());
    }
    seq.reverse();
    shell_sort(data.clone(), seq)
}
