use anyhow::{ensure, Context};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let graph = parse_graph(&input)?;
    let (cut_size, group_a, group_b) =
        global_min_cut(to_weighted_graph(&graph)).context("no minimum cut found")?;
    ensure!(
        cut_size == 3,
        "expected the minimum cut of 3, got {cut_size}"
    );
    let answer = group_a * group_b;
    println!("{answer}");
    Ok(())
}

fn parse_graph(input: &str) -> aoc::Result<HashMap<&str, Vec<&str>>> {
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();
    for line in input.lines() {
        let (node_a, rest) = line.split_once(": ").context("invalid line")?;
        for node_b in rest.split_whitespace() {
            graph.entry(node_a).or_default().push(node_b);
            graph.entry(node_b).or_default().push(node_a);
        }
    }
    Ok(graph)
}

type WeightedGraph = HashMap<usize, HashMap<usize, u32>>;

/// Converts the string nodes to numeric indices and adds a weight of 1 to all edges.
fn to_weighted_graph(graph: &HashMap<&str, Vec<&str>>) -> WeightedGraph {
    let node_indices: HashMap<_, _> = graph.keys().zip(0..).collect();
    graph
        .iter()
        .map(|(v, edges)| {
            let i = node_indices[&v];
            (i, edges.iter().map(|w| (node_indices[w], 1)).collect())
        })
        .collect()
}

/// Computes the minimum cut of a graph using the [Stoer–Wagner algorithm][wiki].
///
/// This code is based on [this C++ implementation][cpp_impl], with some modifications. Since the
/// puzzle graph has nodes with low degree (i.e. edges), a nested HashMap is used for the weighed
/// graph instead of an adjacency matrix, to avoid iterating over a relatively big and very sparse
/// matrix. Also a priority queue was added to speed up the selection of nodes in each phase.
///
/// Returns the size of of the minimum cut (i.e. the sum of the cut edges' weights) and the amount
/// of nodes on each side of the cut. The input graph must be connected and bigger than 1 node,
/// otherwise it returns None.
///
/// [wiki]: https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
/// [cpp_impl]: https://github.com/kth-competitive-programming/kactl/blob/782a5f4e38fff0efb2ae83761e18fb829d6aa00c/content/graph/GlobalMinCut.h
fn global_min_cut(mut graph: WeightedGraph) -> Option<(u32, usize, usize)> {
    let n = graph.len();
    if n <= 1 {
        return None;
    }

    let mut min_cut = (u32::MAX, 0, 0);
    let mut combined_count = vec![1; n];

    while graph.len() > 1 {
        let a = *graph.keys().next().unwrap();
        let mut visited = HashSet::from([a]);
        // Using a priority queue makes each phase O(E*log(V)) instead of O(V²).
        let mut to_visit: PriorityQueue<_, _> = graph[&a].iter().map(|(i, e)| (*i, *e)).collect();
        let mut s = a;
        let mut t = a;
        let mut min_cut_of_phase = 0;
        while visited.len() < graph.len() {
            s = t;
            // Queue can only be empty if the original graph was disconnected.
            (t, min_cut_of_phase) = to_visit.pop()?;
            for (i, e) in graph[&t].iter() {
                if visited.contains(i) {
                    continue;
                }
                if !to_visit.change_priority_by(i, |w| *w += e) {
                    to_visit.push(*i, *e);
                }
            }
            visited.insert(t);
        }
        min_cut = min_cut.min((min_cut_of_phase, combined_count[t], n - combined_count[t]));
        combined_count[s] += combined_count[t];

        // Contract node t into node s.
        let t_edges = graph.remove(&t).unwrap();
        for (i, e) in t_edges {
            // It'd be nice if Rust allowed `graph[&i].remove(&t)` (i.e. impl IndexMut for HashMap)
            graph.get_mut(&i).unwrap().remove(&t);
            if i != s {
                *graph.get_mut(&s).unwrap().entry(i).or_insert(0) += e;
                *graph.get_mut(&i).unwrap().entry(s).or_insert(0) += e;
            }
        }
    }

    Some(min_cut)
}
