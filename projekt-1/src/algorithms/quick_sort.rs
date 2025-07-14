use owo_colors::OwoColorize;
use std::time::Instant;

pub fn run<T: Copy + std::cmp::PartialOrd + std::fmt::Debug>(
    data: Vec<T>,
    pivot_index: usize,
    benchmarking: bool,
) -> f64 {
    let start_time = Instant::now();
    let sorted_data: Vec<T> = quicksort(data.clone(), pivot_index);
    let elapsed_ns = start_time.elapsed().as_nanos();
    let elapsed = elapsed_ns as f64 / 1_000_000.0;

    if benchmarking {
        println!("Time: {}", elapsed.cyan());
    } else {
        println!(
            "Sorted:{:?}\nTime elapsed: {:?}ms",
            sorted_data,
            elapsed.cyan()
        );
    }
    elapsed
}

fn partition<T: std::cmp::PartialOrd>(data: &mut [T], pivot_index: usize) -> usize {
    if pivot_index >= data.len() {
        panic!("Pivot index out of bounds");
    }
    data.swap(pivot_index, data.len() - 1);
    let pivot_index = data.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if data[j] <= data[pivot_index] {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, pivot_index);
    i
}

fn quicksort<T: Copy + std::cmp::PartialOrd>(data_1: Vec<T>, pivot_index: usize) -> Vec<T> {
    let mut data: Vec<T> = data_1.clone();
    if data.len() <= 1 {
        return data;
    }

    let pivot_index = partition(&mut data, pivot_index);

    let (left, right) = data.split_at_mut(pivot_index);

    let mut sorted_left: Vec<T> = quicksort(left.to_vec(), 0);
    let sorted_right: Vec<T> = quicksort(right[1..].to_vec(), 0);

    sorted_left.push(data[pivot_index]);
    sorted_left.extend(sorted_right);

    sorted_left
}
