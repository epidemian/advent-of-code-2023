use anyhow::ensure;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let graph = parse_graph(&input);
    let mat = create_adjacency_matrix(&graph);
    let (cut_size, cut_nodes) = global_min_cut(mat);
    ensure!(
        cut_size == 3,
        "expected the minimum cut of 3, got {cut_size}"
    );
    let answer = cut_nodes.len() * (graph.len() - cut_nodes.len());
    println!("{answer}");
    Ok(())
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let node_a = &line[0..3];
        for node_b in line[5..].split_whitespace() {
            graph.entry(node_a).or_default().push(node_b);
            graph.entry(node_b).or_default().push(node_a);
        }
    }
    graph
}

fn create_adjacency_matrix(graph: &HashMap<&str, Vec<&str>>) -> Vec<Vec<i64>> {
    let node_indices: HashMap<_, _> = graph.keys().zip(0..).collect();
    let mut mat = vec![vec![0; graph.len()]; graph.len()];
    for (node, adjacent_nodes) in graph {
        let i = node_indices[node];
        for adj_node in adjacent_nodes.iter() {
            let j = node_indices[adj_node];
            mat[i][j] = 1;
            mat[j][i] = 1;
        }
    }
    mat
}

// Computes the minimum cut of a graph using the Stoer–Wagner algorithm. The graph is given as an
// adjacency matrix.
// See: https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
// This implementation is just a translation of this C++ code: https://github.com/kth-competitive-programming/kactl/blob/782a5f4e38fff0efb2ae83761e18fb829d6aa00c/content/graph/GlobalMinCut.h
#[allow(clippy::needless_range_loop)]
fn global_min_cut(mut mat: Vec<Vec<i64>>) -> (i64, Vec<usize>) {
    let n = mat.len();
    let mut best = (i64::MAX, vec![]);
    let mut co = (0..n).map(|i| vec![i]).collect_vec();

    for phase in 1..n {
        let mut w = mat[0].clone();
        let mut s = 0;
        let mut t = 0;
        for _ in 0..n - phase {
            w[t] = i64::MIN;
            s = t;
            t = w.iter().position_max().unwrap();
            for i in 0..n {
                w[i] += mat[t][i]
            }
        }
        let min_cut_of_phase = w[t] - mat[t][t];
        if min_cut_of_phase < best.0 {
            best = (min_cut_of_phase, co[t].clone());
        }
        let co_t = co[t].clone();
        co[s].extend(co_t);
        for i in 0..n {
            mat[s][i] += mat[t][i]
        }
        for i in 0..n {
            mat[i][s] = mat[s][i]
        }
        mat[0][t] = i64::MIN;
    }

    best
}
