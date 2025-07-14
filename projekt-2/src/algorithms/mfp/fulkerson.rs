use std::collections::{LinkedList, VecDeque};
use std::time::Instant;

use crate::utils::convert;

/// Start
pub fn run(graf: Vec<Vec<i64>>, choice: usize) {
    match choice {
        1 => {
            // macierzowa wersja
            let start = Instant::now();
            fulkerson(&graf);
            let elapsed_ms = start.elapsed().as_secs_f64() * 1_000.0;
            println!("Matrix implementation: {:.3} ms", elapsed_ms);
        }
        2 => {
            // lista sąsiedztwa
            let list_graph: LinkedList<LinkedList<(usize, i64)>> =
                convert::create_listgraph(graf.clone());
            let start = Instant::now();
            fulkerson_listgraph(list_graph);
            let elapsed_ms = start.elapsed().as_secs_f64() * 1_000.0;
            println!("List implementation: {:.3} ms", elapsed_ms);
        }
        _ => {
            println!("[-] Seriously? Exiting...");
        }
    }
}

/// Matrix
fn fulkerson(cap: &Vec<Vec<i64>>) {
    let mut path = Vec::new();
    let n = cap.len();
    let source = 0;
    let sink = n - 1;
    let original = cap.clone();
    let mut res = cap.clone();
    let mut parent = vec![None; n];
    let mut max_flow = 0;

    // BFS na macierzy
    let mut bfs_mat = |res: &Vec<Vec<i64>>, parent: &mut Vec<Option<usize>>| -> bool {
        parent.fill(None);
        let mut visited = vec![false; n];
        let mut q = VecDeque::new();
        visited[source] = true;
        q.push_back(source);
        while let Some(u) = q.pop_front() {
            for v in 0..n {
                if !visited[v] && res[u][v] > 0 {
                    visited[v] = true;
                    parent[v] = Some(u);
                    if v == sink {
                        return true;
                    }
                    q.push_back(v);
                }
            }
        }
        false
    };

    while bfs_mat(&res, &mut parent) {
        path.clear();
        let mut v = sink;
        while let Some(u) = parent[v] {
            path.push(v);
            v = u;
        }
        path.push(source);
        path.reverse();

        let mut flow = i64::MAX;
        let mut v = sink;
        while let Some(u) = parent[v] {
            flow = flow.min(res[u][v]);
            v = u;
        }

        let mut v = sink;
        while let Some(u) = parent[v] {
            res[u][v] -= flow;
            res[v][u] += flow;
            v = u;
        }

        max_flow += flow;
    }

    println!("\nKOncowy przeplyw:");
    for u in 0..n {
        for v in 0..n {
            if original[u][v] > 0 {
                let flow = res[v][u];

                println!("{} - {} : {}", u, v, flow);
            }
        }
    }
    println!("\nMaksymalny przepływ (macierz): {}", max_flow);
}
fn fulkerson_listgraph(mut graph: LinkedList<LinkedList<(usize, i64)>>) {
    let mut path = Vec::new();
    let n = graph.len();
    let source = 0;
    let sink = n - 1;
    let original_graph = graph.clone();
    let mut res_graph = graph.clone();
    let mut parent = vec![None; n];
    let mut max_flow = 0;

    // BFS na liście
    let mut bfs_list =
        |res: &LinkedList<LinkedList<(usize, i64)>>, parent: &mut Vec<Option<usize>>| -> bool {
            parent.fill(None);
            let mut visited = vec![false; n];
            let mut q = VecDeque::new();
            visited[source] = true;
            q.push_back(source);
            while let Some(u) = q.pop_front() {
                for &(v, cap) in res.iter().nth(u).unwrap() {
                    if cap > 0 && !visited[v] {
                        visited[v] = true;
                        parent[v] = Some(u);
                        if v == sink {
                            return true;
                        }
                        q.push_back(v);
                    }
                }
            }
            false
        };

    while bfs_list(&res_graph, &mut parent) {
        // odbuduj path
        path.clear();
        let mut v = sink;
        while let Some(u) = parent[v] {
            path.push(v);
            v = u;
        }
        path.push(source);
        path.reverse();

        let mut flow = i64::MAX;
        let mut v = sink;
        while let Some(u) = parent[v] {
            let cap_uv = res_graph
                .iter()
                .nth(u)
                .unwrap()
                .iter()
                .find(|&&(vv, _)| vv == v)
                .map(|&(_, c)| c)
                .unwrap_or(0);
            flow = flow.min(cap_uv);
            v = u;
        }

        // aktualizacja pojemności
        let mut v = sink;
        while let Some(u) = parent[v] {
            {
                let list_u = res_graph.iter_mut().nth(u).unwrap();
                for &mut (vv, ref mut c) in list_u {
                    if vv == v {
                        *c -= flow;
                        break;
                    }
                }
            }
            {
                let list_v = res_graph.iter_mut().nth(v).unwrap();
                if let Some(entry) = list_v.iter_mut().find(|&&mut (vv, _)| vv == u) {
                    entry.1 += flow;
                } else {
                    list_v.push_back((u, flow));
                }
            }
            v = u;
        }

        max_flow += flow;
    }

    let res_vec: Vec<Vec<(usize, i64)>> = res_graph
        .iter()
        .map(|lst| lst.iter().copied().collect())
        .collect();

    println!("\nKoncowy przeplyw:");
    for u in 0..n {
        for &(v, cap) in original_graph.iter().nth(u).unwrap() {
            if cap > 0 {
                let flow = res_vec[v]
                    .iter()
                    .find(|&&(vv, _)| vv == u)
                    .map(|&(_, f)| f)
                    .unwrap_or(0);
                println!("{} - {} : {}", u, v, flow);
            }
        }
    }
    println!("\nMaksymalny przepływ (lista): {}", max_flow);
}
