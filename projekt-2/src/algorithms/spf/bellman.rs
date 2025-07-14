use crate::utils::convert;
use std::collections::LinkedList;
use std::time::Instant;

pub fn run(graph: Vec<Vec<i64>>, choice: usize) {
    match choice {
        1 => {
            // Macierz
            let start_time = Instant::now();
            ford_bellman_algorithm(graph);
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("Matrix Bellman implementation: {}ms", elapsed);
        }
        2 => {
            // Lista sasiedztw
            let graph_list: LinkedList<LinkedList<(usize, i64)>> = convert::create_listgraph(graph);
            let start_time = Instant::now();
            ford_bellman_algorithm_listgraph(graph_list);
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("ListGraph Bellman implementation: {}ms", elapsed);
        }
        _ => {
            println!("[-] Seriously? Exiting...");
        }
    }
}

fn ford_bellman_algorithm(data: Vec<Vec<i64>>) {
    println!("Ford–Bellman algorithm for matrix graph");
    let verticles = data.len();

    let mut distance = vec![i64::MAX; verticles];
    if verticles > 0 {
        distance[0] = 0;
    }

    for _ in 0..verticles.saturating_sub(1) {
        for i in 0..verticles {
            if distance[i] == i64::MAX {
                continue;
            }
            for j in 0..verticles {
                let w = data[i][j];
                if w != 0 && distance[i] + w < distance[j] {
                    distance[j] = distance[i] + w;
                }
            }
        }
    }

    // Wyniki
    for (i, &d) in distance.iter().enumerate() {
        println!("Distance from 0 to {}: {}", i, d);
    }
}

fn ford_bellman_algorithm_listgraph(data: LinkedList<LinkedList<(usize, i64)>>) {
    println!("Ford–Bellman algorithm for list graph");
    let verticles = data.len();

    let mut distance = vec![i64::MAX; verticles];
    if verticles > 0 {
        distance[0] = 0;
    }

    for _ in 0..verticles.saturating_sub(1) {
        for (i, neighbors) in data.iter().enumerate() {
            if distance[i] == i64::MAX {
                continue;
            }

            for &(v, w) in neighbors {
                if distance[i] + w < distance[v] {
                    distance[v] = distance[i] + w;
                }
            }
        }
    }

    // Wyniki
    for (i, &d) in distance.iter().enumerate() {
        println!("Distance from 0 to {}: {}", i, d);
    }
}
