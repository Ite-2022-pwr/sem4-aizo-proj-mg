use owo_colors::OwoColorize;
use std::fs;
use std::io::Write;
use std::path::Path;

use rand::Rng;

pub fn read_datai(path: &Path) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Failed to read a file.");
    let lines: Vec<&str> = contents.trim().split('\n').collect();
    let size = lines[0].trim().parse::<i64>().expect("REASON");
    let mut data: Vec<i64> = Vec::new();

    // println!("{:?}", lines);

    for i in 1..size {
        let tmp = lines[i as usize].trim().parse::<i64>().expect("REASON");
        data.push(tmp);
    }

    data
}

pub fn read_dataf(path: &Path) -> Vec<f64> {
    // Check if its going to work
    let contents = fs::read_to_string(path).expect("Failed to read a file.");
    let lines: Vec<&str> = contents.trim().split('\n').collect();
    let size = lines[0].trim().parse::<i64>().expect("REASON");
    let mut data: Vec<f64> = Vec::new();

    // println!("{:?}", lines);

    for i in 1..size {
        let tmp = lines[i as usize].trim().parse::<f64>().expect("REASON");
        data.push(tmp);
    }

    data
}

pub fn write_results<T: std::fmt::Display>(data: Vec<T>, mut filename: String) {
    let random_filename = format!("./results/{}", rand::rng().random::<u32>());
    if filename == String::from("random") {
        filename = random_filename;
    } else {
        filename = format!("./results/{}", filename);
    }
    let mut file = fs::File::create(&filename).expect("[-] Failed");

    writeln!(file, "{}", data.len()).expect("[-] Failed");

    for val in data {
        writeln!(file, "{}", val).expect("[-] Failed");
    }

    println!("[+] Zapisano dane do: {}", filename.cyan());
}
pub fn list_files() {
    let paths = fs::read_dir("./data/").unwrap();
    let mut counter = 1;
    for path in paths {
        println!("{} {:?}", counter, path.unwrap().file_name().green());
        counter += 1;
    }
}

pub fn verify_data_type(path: &Path) -> bool {
    let contents = fs::read_to_string(path).expect("Failed to read a file.");
    let lines: Vec<&str> = contents.trim().split('\n').collect();

    if lines[1].contains(".") {
        return true;
    }
    false
}
