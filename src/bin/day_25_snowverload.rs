use anyhow::ensure;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let node_a = &line[0..3];
        for node_b in line[5..].split_whitespace() {
            graph.entry(node_a).or_default().push(node_b);
            graph.entry(node_b).or_default().push(node_a);
        }
    }
    let node_indices: HashMap<_, _> = graph
        .keys()
        .sorted()
        .enumerate()
        .map(|(i, node)| (*node, i))
        .collect();
    let n = graph.len();
    let mut mat = vec![vec![0_i64; n]; n];
    for (node, adjacent_nodes) in graph {
        let i = node_indices[node];
        for adj_node in adjacent_nodes.iter() {
            let j = node_indices[adj_node];
            mat[i][j] = 1;
            mat[j][i] = 1;
        }
    }

    let (min_cut_size, min_cut_nodes) = global_min_cut(mat);
    ensure!(min_cut_size == 3, "expected to cut 3 wires");

    let ans_1 = min_cut_nodes.len() * (n - min_cut_nodes.len());
    println!("{ans_1}");

    Ok(())
}

// Computes the minimum cut of a graph using the Stoer–Wagner algorithm. The graph is given as an
// adjacency matrix.
// See: https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
fn global_min_cut(mut mat: Vec<Vec<i64>>) -> (i64, Vec<i64>) {
    let mut best = (i64::MAX, vec![]);
    let n = mat.len();
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
        best = best.min((w[t] - mat[t][t], co[t].clone()));
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

    (best.0, best.1.into_iter().map(|i| i as i64).collect())
}
