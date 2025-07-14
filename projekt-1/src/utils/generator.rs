use rand::{rng, Rng};

use crate::handlers::files_handler;

pub fn generate_data_i64(data_len: usize, filename: String) -> Vec<i64> {
    let mut generated_data: Vec<i64> = Vec::new();
    for _i in 0..data_len {
        let tmp = rand::rng().random_range(1..=100);
        generated_data.push(tmp);
    }
    files_handler::write_results(generated_data.clone(), filename);
    generated_data
}

pub fn generate_data_f64(data_len: usize, filename: String) -> Vec<f64> {
    let mut generated_data: Vec<f64> = Vec::new();
    for _i in 0..data_len {
        let tmp: f64 = rand::rng().random_range(0.0..100.0);
        generated_data.push(tmp);
    }
    files_handler::write_results(generated_data.clone(), filename);
    generated_data
}

pub fn sort_percent<T: Clone + std::fmt::Debug + PartialOrd>(
    mut data: Vec<T>,
    part: usize,
) -> Vec<T> {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let rsite: usize;
    if part == 1 {
        rsite = data.len() / 3;
    } else if part == 2 {
        rsite = 0;
    } else {
        rsite = data.len() * 2 / 3;
    }
    let sorted_part: Vec<T> = (&data[0..rsite]).to_vec();
    let mut shuffled_part: Vec<T> = (&data[rsite..]).to_vec();
    shuffled_part = shuffle(shuffled_part);
    println!("Sorted: {:?}\nShuffled: {:?}", sorted_part, shuffled_part);
    let returndata: Vec<T> = sorted_part
        .into_iter()
        .chain(shuffled_part.into_iter())
        .collect();
    returndata
}

fn shuffle<T>(mut data: Vec<T>) -> Vec<T> {
    for i in 0..data.len() {
        let j = rng().random_range(0..data.len());
        data.swap(i, j);
    }
    data
}
