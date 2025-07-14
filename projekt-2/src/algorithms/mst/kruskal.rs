use crate::utils::convert;

use std::collections::LinkedList;
use std::time::Instant;

pub fn run(graf: Vec<Vec<i64>>, choice: usize) {
    match choice {
        1 => {
            // Matrix Kruskal
            let start_time = Instant::now();
            kruskal(&graf);
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("Matrix Kruskal for matrix implementation: {}ms", elapsed);
        }

        2 => {
            //ListGraph Kruskal
            let data: LinkedList<LinkedList<(usize, i64)>> =
                convert::create_listgraph(graf.clone());
            let start_time = Instant::now();
            kruskal_listgraph(data.clone());
            let elapsed_ns = start_time.elapsed().as_nanos();
            let elapsed = elapsed_ns as f64 / 1_000_000.0;
            println!("ListGraph Kruskal implementation: {}ms", elapsed);
        }

        _ => {
            println!("[-] Seriously?. Exiting...");
            return;
        }
    }
}

fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn union(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) {
    let pa = find(parent, a);
    let pb = find(parent, b);
    if pa != pb {
        if rank[pa] < rank[pb] {
            parent[pa] = pb;
        } else if rank[pa] > rank[pb] {
            parent[pb] = pa;
        } else {
            parent[pb] = pa;
            rank[pa] += 1;
        }
    }
}

fn kruskal(graf: &Vec<Vec<i64>>) {
    let n = graf.len();
    let mut edges = Vec::new();
    for u in 0..n {
        for v in (u + 1)..n {
            let weight = graf[u][v];
            if weight >= 0 {
                edges.push((weight, u, v));
            }
        }
    }
    edges.sort_unstable_by_key(|e| e.0);

    let mut parent = (0..n).collect::<Vec<usize>>();
    let mut rank = vec![0; n];
    let mut mst = Vec::new();
    let mut total_weight = 0;

    for (weight, u, v) in edges {
        let pu = find(&mut parent, u);
        let pv = find(&mut parent, v);
        if pu != pv {
            union(&mut parent, &mut rank, u, v);
            mst.push((u, v, weight));
            total_weight += weight;
        }
    }

    println!("Minimalne drzewo rozpinarozpinajace (MST):");
    for (u, v, w) in mst {
        println!("{} - {} : {}", u, v, w);
    }
    println!("Laczna waga MST: {}", total_weight);
}

pub fn kruskal_listgraph(data: LinkedList<LinkedList<(usize, i64)>>) {
    let n = data.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();

    // niedoblowanie
    for (u, neighs) in data.iter().enumerate() {
        for &(v, w) in neighs {
            if w >= 0 && u < v {
                edges.push((w, u, v));
            }
        }
    }

    // Sortuj po wadze
    edges.sort_unstable_by_key(|e| e.0);

    // Przygotuj DSU
    let mut parent = (0..n).collect::<Vec<usize>>();
    let mut rank = vec![0; n];

    let mut mst = Vec::new();
    let mut total_weight: i64 = 0;

    // Kruskal
    for (w, u, v) in edges {
        let pu = find(&mut parent, u);
        let pv = find(&mut parent, v);
        if pu != pv {
            union(&mut parent, &mut rank, pu, pv);
            mst.push((u, v, w));
            total_weight += w;
        }
    }

    // Wyniki
    println!("Minimalne drzewo rozpinające (MST):");
    for (u, v, w) in &mst {
        println!("{} - {} : {}", u, v, w);
    }
    println!("Łączna waga MST: {}", total_weight);
}
