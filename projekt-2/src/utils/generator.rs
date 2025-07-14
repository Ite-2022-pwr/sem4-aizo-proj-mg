use rand::{rng, seq::SliceRandom, Rng};

type Graph = Vec<Vec<i64>>;

fn add_extra_edges(graph: &mut Graph, density: u8, directed: bool) {
    let mut rng = rng();

    let n = graph.len();
    let mut candidates = Vec::new();

    if directed {
        for u in 0..n {
            for v in 0..n {
                if u != v && graph[u][v] == 0 {
                    candidates.push((u, v));
                }
            }
        }
    } else {
        for u in 0..n {
            for v in (u + 1)..n {
                if graph[u][v] == 0 {
                    candidates.push((u, v));
                }
            }
        }
    }

    candidates.shuffle(&mut rng);
    let max_extra = candidates.len() as u32;
    let extra = (u32::from(density) * max_extra / 100) as usize;

    for &(u, v) in &candidates[..extra] {
        let w = rng.random_range(1..=1000);
        graph[u][v] = w;
        if !directed {
            graph[v][u] = w;
        }
    }
}

pub fn generate_graph_mfp(num_vertices: i64, density: i64) -> Graph {
    let mut rng = rng();
    let n = if num_vertices >= 0 {
        num_vertices as usize
    } else {
        panic!("num_vertices must be non-negative");
    };
    let density_u8 = density.clamp(0, 100) as u8;

    let mut graph = vec![vec![0; n]; n];

    let mut order: Vec<usize> = (0..n).collect();
    order.shuffle(&mut rng);
    for window in order.windows(2) {
        let (u, v) = (window[0], window[1]);
        graph[u][v] = rng.random_range(1..=1000);
    }

    add_extra_edges(&mut graph, density_u8, true);
    graph
}

pub fn generate_graph_mst(num_vertices: i64, density: i64) -> Graph {
    let mut rng = rng();
    let n = if num_vertices >= 0 {
        num_vertices as usize
    } else {
        panic!("num_vertices must be non-negative");
    };
    let density_u8 = density.clamp(0, 100) as u8;

    let mut graph = vec![vec![0; n]; n];

    let mut order: Vec<usize> = (0..n).collect();
    order.shuffle(&mut rng);
    for window in order.windows(2) {
        let (u, v) = (window[0], window[1]);
        let w = rng.random_range(1..=1000);
        graph[u][v] = w;
        graph[v][u] = w;
    }

    add_extra_edges(&mut graph, density_u8, false);
    graph
}

pub fn generate_graph_spf(num_vertices: i64, density: i64) -> Graph {
    let mut rng = rng();
    let n = if num_vertices >= 0 {
        num_vertices as usize
    } else {
        panic!("num_vertices must be non-negative");
    };
    let density_u8 = density.clamp(0, 100) as u8;

    let mut graph = vec![vec![0; n]; n];

    for v in 1..n {
        graph[0][v] = rng.random_range(1..=1000);
    }

    add_extra_edges(&mut graph, density_u8, true);
    graph
}
