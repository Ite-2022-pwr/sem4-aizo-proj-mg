// Algorytm Prima - minimalne drzewo rozpinające
use std::cmp::Reverse;
use std::collections::{BinaryHeap, LinkedList};
use std::time::Instant;

use crate::utils::convert::create_listgraph;

pub fn run(graf: Vec<Vec<i64>>, choice: usize) {
    match choice {
        1 => {
            // Matrix Prima
            let start_time = Instant::now();
            prim(0, graf);
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("Matrix Prim implementation: {}ms", elapsed);
        }

        2 => {
            // ListGraph prima
            let data = create_listgraph(graf.clone());
            let start_time = Instant::now();
            prim_listgraph(0, data);
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("ListGraph Prim implementation: {}ms", elapsed);
        }

        _ => {
            println!("[-] Seriously?. Exiting...");
            return;
        }
    }
}

pub fn prim(start: usize, graf: Vec<Vec<i64>>) {
    let n = graf.len();
    let mut min_edge = vec![usize::MAX; n];
    let mut visited = vec![false; n];
    let mut parent = vec![None; n];

    // heap z elementami (Reverse(waga), numer_wierzchokla)
    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();

    // start
    min_edge[start] = 0;
    heap.push((Reverse(0), start));

    let mut total = 0;

    while let Some((Reverse(w), u)) = heap.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        total += w;

        for v in 0..n {
            let wgt = graf[u][v];
            if wgt > 0 {
                let w_usize = wgt as usize;
                if !visited[v] && w_usize < min_edge[v] {
                    min_edge[v] = w_usize;
                    parent[v] = Some(u);
                    heap.push((Reverse(w_usize), v));
                }
            }
        }
    }

    // Wypisz krawedzie MST
    println!("Krawędzie MST:");
    for v in 0..n {
        if let Some(u) = parent[v] {
            println!("{} - {} : {}", u, v, min_edge[v]);
        }
    }
    println!("Łączna waga MST: {}", total);
}

pub fn prim_listgraph(startowy: usize, graf: LinkedList<LinkedList<(usize, i64)>>) {
    let adj: Vec<Vec<(usize, i64)>> = graf
        .into_iter()
        .map(|lst| lst.into_iter().collect())
        .collect();

    let n = adj.len();
    let mut min_edge = vec![usize::MAX; n];
    let mut visited = vec![false; n];
    let mut parent = vec![None; n];

    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();

    // start
    min_edge[startowy] = 0;
    heap.push((Reverse(0), startowy));

    let mut total = 0;

    while let Some((Reverse(w), u)) = heap.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        total += w as usize;

        for &(v, w_i64) in &adj[u] {
            let w_usize = w_i64 as usize;
            if !visited[v] && w_usize < min_edge[v] {
                min_edge[v] = w_usize;
                parent[v] = Some(u);
                heap.push((Reverse(w_usize), v));
            }
        }
    }

    // Wypisz krawedzie MST
    println!("Krawędzie MST:");
    for v in 0..n {
        if let Some(u) = parent[v] {
            println!("{} - {} : {}", u, v, min_edge[v]);
        }
    }
    println!("Łączna waga MST: {}", total);
}
