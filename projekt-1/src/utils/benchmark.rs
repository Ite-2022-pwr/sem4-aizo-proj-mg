// [WIP]
use std::cmp;
use std::path::Path;

use crate::{
    algorithms::{heap_sort, insert_sort, quick_sort},
    handlers::files_handler::read_dataf,
};

pub fn _run() {
    let filenames_i64_0: [&Path; 7] = [
        Path::new("i64-10k-0"),
        Path::new("i64-40k-0"),
        Path::new("i64-70k-0"),
        Path::new("i64-80k-0"),
        Path::new("i64-100k-0"),
        Path::new("i64-120k-0"),
        Path::new("i64-150k-0"),
    ];

    let _filenames_f64_0: [&Path; 7] = [
        Path::new("f64-10k-0"),
        Path::new("f64-40k-0"),
        Path::new("f64-70k-0"),
        Path::new("f64-80k-0"),
        Path::new("f64-100k-0"),
        Path::new("f64-120k-0"),
        Path::new("f64-150k-0"),
    ];

    let filenames_i64_33: [&Path; 7] = [
        Path::new("i64-10k-33"),
        Path::new("i64-40k-33"),
        Path::new("i64-70k-33"),
        Path::new("i64-80k-33"),
        Path::new("i64-100k-33"),
        Path::new("i64-120k-33"),
        Path::new("i64-150k-33"),
    ];

    let _filenames_f64_33: [&Path; 7] = [
        Path::new("f64-10k-33"),
        Path::new("f64-40k-33"),
        Path::new("f64-70k-33"),
        Path::new("f64-80k-33"),
        Path::new("f64-100k-33"),
        Path::new("f64-120k-33"),
        Path::new("f64-150k-33"),
    ];

    let filenames_i64_66: [&Path; 7] = [
        Path::new("i64-10k-66"),
        Path::new("i64-40k-66"),
        Path::new("i64-70k-66"),
        Path::new("i64-80k-66"),
        Path::new("i64-100k-66"),
        Path::new("i64-120k-66"),
        Path::new("i64-150k-66"),
    ];

    let _filenames_f64_66: [&Path; 7] = [
        Path::new("f64-10k-66"),
        Path::new("f64-40k-66"),
        Path::new("f64-70k-66"),
        Path::new("f64-80k-66"),
        Path::new("f64-100k-66"),
        Path::new("f64-120k-66"),
        Path::new("f64-150k-66"),
    ];

    let data_file_path = Path::new("./data/");
    _benchmark_i64(
        filenames_i64_0,
        filenames_i64_33,
        filenames_i64_66,
        &data_file_path,
    );
}

fn _benchmark_i64(
    filenames_0: [&Path; 7],
    filenames_33: [&Path; 7],
    filenames_66: [&Path; 7],
    data_file_path: &Path,
) {
    let files = vec![filenames_0, filenames_33, filenames_66];
    let mut results: Vec<Vec<f64>> = vec![Vec::new(); 5];

    for _data_file in files {}
    // Insertsort 0
    for p in filenames_0 {
        println!("{:?}", p);
        results[0].push(insert_sort::run(
            read_dataf(data_file_path.join(p).as_path()),
            true,
        ));
    }
    // Heapsort 1
    for p in filenames_0 {
        println!("{:?}", p);
        results[1].push(heap_sort::run(
            read_dataf(data_file_path.join(p).as_path()),
            true,
        ));
    }
    // Quicksort 2 - pivot L
    for p in filenames_0 {
        println!("{:?}", p);
        results[2].push(quick_sort::run(
            read_dataf(data_file_path.join(p).as_path()),
            0,
            true,
        ));
    }

    // Quicksort 2 - pivot R
    for p in filenames_0 {
        println!("{:?}", p);
        results[2].push(quick_sort::run(
            read_dataf(data_file_path.join(p).as_path()),
            cmp::max(filenames_0.len(), filenames_0.len()) - 1,
            true,
        ));
    }
    // Quicksort 2 - pivot Śr
    for p in filenames_0 {
        println!("{:?}", p);
        results[2].push(quick_sort::run(
            read_dataf(data_file_path.join(p).as_path()),
            cmp::max(filenames_0.len(), filenames_0.len()) / 2 - 1,
            true,
        ));
    }
    // Shellsort Ciura 3
    for p in filenames_0 {
        println!("{:?}", p);
        results[3].push(insert_sort::run(
            read_dataf(data_file_path.join(p).as_path()),
            true,
        ));
    }
    // Shellsort Knuth 4
    for p in filenames_0 {
        println!("{:?}", p);
        results[4].push(insert_sort::run(
            read_dataf(data_file_path.join(p).as_path()),
            true,
        ));
    }

    println!("Results: {:?}", results);
}

fn _parse_duration() {}
