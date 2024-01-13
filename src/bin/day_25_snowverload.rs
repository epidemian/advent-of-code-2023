use anyhow::ensure;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let graph = parse_graph(&input);
    let (cut_size, cut_nodes) = global_min_cut(to_index_graph(&graph));
    ensure!(
        cut_size == 3,
        "expected the minimum cut of 3, got {cut_size}"
    );
    let answer = cut_nodes.len() * (graph.len() - cut_nodes.len());
    println!("{answer}");
    Ok(())
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();
    for line in input.lines() {
        let node_a = &line[0..3];
        for node_b in line[5..].split_whitespace() {
            graph.entry(node_a).or_default().push(node_b);
            graph.entry(node_b).or_default().push(node_a);
        }
    }
    graph
}

fn to_index_graph(graph: &HashMap<&str, Vec<&str>>) -> Vec<HashSet<usize>> {
    let node_indices: HashMap<_, _> = graph.keys().sorted().zip(0..).collect();
    let mut index_graph = vec![HashSet::new(); graph.len()];
    for (n, adj) in graph.iter() {
        index_graph[node_indices[&n]] = adj.iter().map(|m| node_indices[m]).collect();
    }
    index_graph
}

// Computes the minimum cut of a graph using the Stoer–Wagner algorithm. The graph is given as an
// adjacency matrix.
// See: https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
// This implementation is based on this C++ code: https://github.com/kth-competitive-programming/kactl/blob/782a5f4e38fff0efb2ae83761e18fb829d6aa00c/content/graph/GlobalMinCut.h
fn global_min_cut(mut graph: Vec<HashSet<usize>>) -> (i64, Vec<usize>) {
    let n = graph.len();
    let mut best = (i64::MAX, vec![]);
    let mut co = (0..n).map(|i| vec![i]).collect_vec();
    let mut nodes = (0..n).collect_vec();
    let mut mat = vec![vec![0; n]; n];
    for (n, adj) in graph.iter().enumerate() {
        for &m in adj.iter() {
            mat[n][m] = 1;
            mat[m][n] = 1;
        }
    }

    while nodes.len() > 1 {
        let a = nodes[0];
        let mut q: PriorityQueue<_, _> = graph[a].iter().map(|&n| (n, mat[a][n])).collect();
        let mut s = a;
        let mut t = a;
        let mut cut = 0;
        for _ in 0..nodes.len() - 1 {
            q.push_decrease(t, i64::MIN);
            s = t;
            (t, cut) = q.pop().expect("queue should contain a node");
            for &n in &graph[t] {
                if !q.change_priority_by(&n, |w| *w += mat[t][n]) {
                    q.push(n, mat[t][n]);
                }
            }
        }
        let min_cut_of_phase = cut;
        if min_cut_of_phase < best.0 {
            best = (min_cut_of_phase, co[t].clone());
        }
        let co_t = co[t].clone();
        co[s].extend(co_t);

        for n in graph[t].clone() {
            graph[n].remove(&t);
            if n != s {
                graph[s].insert(n);
                graph[n].insert(s);
                mat[s][n] += mat[t][n];
                mat[n][s] = mat[s][n];
            }
        }
        nodes.remove(nodes.iter().position(|&n| n == t).unwrap());
        graph[t].clear();
    }

    best
}
