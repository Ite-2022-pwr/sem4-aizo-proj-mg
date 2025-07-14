use crate::utils::convert::create_listgraph;

use std::collections::LinkedList;
use std::time::Instant;

pub fn run(data: Vec<Vec<i64>>, choice: usize) {
    match choice {
        1 => {
            // Run Matrix implementation for dijkstra algorithm
            let start_time = Instant::now();
            dijkstra_algorithm(data);
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("ListGraph Bellman implementation: {}ms", elapsed);
        }

        2 => {
            // Run ListGraph implementation for dijkstra algorithm
            let start_time = Instant::now();
            // call the function.
            let mut graph_list = create_listgraph(data);
            dijkstra_algorithm_listgraph(graph_list);
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("ListGraph Bellman implementation: {}ms", elapsed);
        }

        _ => {
            println!("[-] Seriously? Exiting...");
            return;
        }
    }
}

fn dijkstra_algorithm(mut data: Vec<Vec<i64>>) {
    let mut verticles: i64 = data.len().try_into().unwrap();
    println!("Dijkstra algorithm for matrix graph");
    let mut distance: Vec<i64> = vec![i64::MAX; verticles as usize];
    let mut visited: Vec<bool> = vec![false; verticles as usize];
    distance[0] = 0;

    for _ in 0..verticles - 1 {
        let u = min_distance(&distance, &visited, verticles);
        visited[u as usize] = true;

        for v in 0..verticles {
            if !visited[v as usize]
                && data[u as usize][v as usize] != 0
                && distance[u as usize] != i64::MAX
                && distance[u as usize] + data[u as usize][v as usize] < distance[v as usize]
            {
                distance[v as usize] = distance[u as usize] + data[u as usize][v as usize];
            }
        }
    }

    for i in 0..verticles {
        println!("Distance from 0 to {}: {}", i, distance[i as usize]);
    }
}

fn min_distance(distance: &Vec<i64>, visited: &Vec<bool>, verticles: i64) -> i64 {
    let mut min: i64 = i64::MAX;
    let mut min_index: i64 = -1;

    for v in 0..verticles {
        if !visited[v as usize] && distance[v as usize] <= min {
            min = distance[v as usize];
            min_index = v;
        }
    }

    min_index
}

fn min_distance_listgraph(distance: &[i64], visited: &[bool]) -> usize {
    distance
        .iter()
        .enumerate()
        .filter(|&(i, _)| !visited[i])
        .min_by_key(|&(_, &dist)| dist)
        .map(|(i, _)| i)
        .unwrap()
}

pub fn dijkstra_algorithm_listgraph(mut data: LinkedList<LinkedList<(usize, i64)>>) {
    let n = data.len();
    println!("Dijkstra algorithm for list graph");

    let mut distance = vec![i64::MAX; n];
    let mut visited = vec![false; n];
    distance[0] = 0;

    // V−1 iteracji
    for _ in 0..n - 1 {
        let u = min_distance_listgraph(&distance, &visited);
        visited[u] = true;

        let neighs = data.iter().nth(u).unwrap();

        for &(v, w) in neighs {
            if !visited[v] && distance[u] != i64::MAX {
                let new_dist = distance[u].saturating_add(w);
                if new_dist < distance[v] {
                    distance[v] = new_dist;
                }
            }
        }
    }

    // wypisz wyniki
    for (i, &d) in distance.iter().enumerate() {
        println!("Distance from 0 to {}: {}", i, d);
    }
}
