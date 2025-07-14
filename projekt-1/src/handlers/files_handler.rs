use owo_colors::OwoColorize;
use std::fs;
use std::io::Write;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use rand::Rng;
use std::io::{BufRead, BufReader};

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
pub fn list_files(addpath: String) {
    let path = Path::new("./data/").join(addpath);
    let paths = fs::read_dir(path).unwrap();
    let mut counter = 1;
    for path in paths {
        println!("{} {:?}", counter, path.unwrap().file_name().green());
        counter += 1;
    }
}

pub fn __verify_data_type(path: &Path) -> bool {
    let contents = fs::read_to_string(path).expect("Failed to read a file.");
    let lines: Vec<&str> = contents.trim().split('\n').collect();

    if lines[1].contains(".") {
        return true;
    }
    false
}

// Wczytywanie danych dla MST
pub fn read_data_mst(path: &Path) -> Vec<Vec<i64>> {
    let mut amount: i64 = 0;

    let file = fs::File::open(path).expect("Failed to read a file.");
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut data: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        if let Ok(l) = line {
            let edge: Vec<i64> = l
                .split_whitespace()
                .take(3)
                .map(|x| x.parse::<i64>().expect("Error"))
                .collect();
            if edge.len() == 3 {
                data.push(edge.clone());
            } else if edge.len() == 2 {
                amount = edge[0];
            }
        }
    }

    let mut result: Vec<Vec<i64>> = vec![vec![-1; amount as usize]; amount as usize];
    for i in data {
        let x: usize = i[0] as usize;
        let y: usize = i[1] as usize;
        result[x][y] = i[2];
        result[y][x] = i[2];
    }

    result
}

pub fn read_data_mpf(path: &Path) -> Vec<Vec<i64>> {
    let file = fs::File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader
        .lines()
        .map(|l| l.expect("IO error"))
        .collect::<Vec<_>>();

    let header = lines.remove(0);
    let parts = header.split_whitespace().collect::<Vec<_>>();
    let e_count: usize = parts[0].parse().expect("Invalid edge count");
    let v_count: usize = parts[1].parse().expect("Invalid vertex count");

    let mut edges = Vec::with_capacity(e_count);
    for line in lines.into_iter().take(e_count) {
        let nums = line.split_whitespace().collect::<Vec<_>>();
        let u = nums[0].parse::<usize>().expect("Invalid u");
        let v = nums[1].parse::<usize>().expect("Invalid v");
        let w = nums[2].parse::<i64>().expect("Invalid weight");
        edges.push((u, v, w));
    }

    let mut mat = vec![vec![0; v_count]; v_count];
    for (u, v, w) in edges {
        mat[u][v] = w;
    }
    mat
}

pub fn read_data_spf(path: &Path) -> Vec<Vec<i64>> {
    let file = fs::File::open(path).expect("Nie udało się otworzyć pliku");
    let mut lines = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Błąd I/O"))
        .collect::<Vec<_>>();

    let header = lines.remove(0);
    let mut parts = header.split_whitespace();
    let e_count: usize = parts.next().unwrap().parse().unwrap();
    let v_count: usize = parts.next().unwrap().parse().unwrap();
    let mut mat = vec![vec![0; v_count]; v_count];

    for line in lines.into_iter().take(e_count) {
        let mut p = line.split_whitespace();
        let u: usize = p.next().unwrap().parse().unwrap();
        let v: usize = p.next().unwrap().parse().unwrap();
        let w: i64 = p.next().unwrap().parse().unwrap();
        mat[u][v] = w;
    }

    mat
}
