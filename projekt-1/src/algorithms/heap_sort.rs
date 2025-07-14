use owo_colors::OwoColorize;
use std::time::Instant;

pub fn run<T: Clone + std::cmp::PartialOrd + std::fmt::Debug + std::marker::Copy>(
    data: Vec<T>,
    benchmarking: bool,
) -> f64 {
    // Przetlumaczenie Vec na arr
    let data_box: Box<[T]> = data.into_boxed_slice();

    let start_time = Instant::now();
    let sorted_data: Box<[T]> = heapsort(data_box);
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

fn heapify<T: std::cmp::PartialOrd>(data: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && data[left] > data[largest] {
        largest = left;
    }

    if right < n && data[right] > data[largest] {
        largest = right;
    }

    if largest != i {
        data.swap(i, largest);
        heapify(data, n, largest);
    }
}

fn heapsort<T: Clone + std::cmp::PartialOrd>(data_1: Box<[T]>) -> Box<[T]> {
    let mut data = data_1.clone();
    let n = data.len();

    if n == 0 {
        return data;
    }

    for i in (0..n / 2).rev() {
        heapify(&mut data, n, i);
    }

    for i in (1..n).rev() {
        data.swap(0, i);
        heapify(&mut data, i, 0);
    }

    data
}
