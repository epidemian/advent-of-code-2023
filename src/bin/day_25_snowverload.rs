use priority_queue::PriorityQueue;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let graph = parse_graph(&input);
    let (cut_size, cut_nodes) = global_min_cut(to_weighted_graph(&graph));
    anyhow::ensure!(
        cut_size == 3,
        "expected the minimum cut of 3, got {cut_size}"
    );
    let answer = cut_nodes * (graph.len() - cut_nodes);
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

/// Converts the string nodes to numeric indices and adds a weight of 1 to all edges.
fn to_weighted_graph(graph: &HashMap<&str, Vec<&str>>) -> HashMap<usize, HashMap<usize, i64>> {
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
/// [wiki]: https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
/// [cpp_impl]: https://github.com/kth-competitive-programming/kactl/blob/782a5f4e38fff0efb2ae83761e18fb829d6aa00c/content/graph/GlobalMinCut.h
fn global_min_cut(mut graph: HashMap<usize, HashMap<usize, i64>>) -> (i64, usize) {
    let n = graph.len();
    let mut best = (i64::MAX, 0);
    let mut combined_count = vec![1; n];

    while graph.len() > 1 {
        let a = *graph.keys().next().unwrap();
        // Using a priority queue makes each phase O(E*log(V)) instead of O(V²).
        let mut q: PriorityQueue<_, _> = graph[&a].iter().map(|(&i, &e)| (i, e)).collect();
        let mut s = a;
        let mut t = a;
        let mut min_cut_of_phase = 0;
        for _ in 0..graph.len() - 1 {
            q.push_decrease(t, i64::MIN);
            s = t;
            // Queue should not be empty, unless input graph is already disjoint.
            (t, min_cut_of_phase) = q.pop().unwrap();
            for (&i, &e) in graph[&t].iter() {
                if !q.change_priority_by(&i, |w| *w += e) {
                    q.push(i, e);
                }
            }
        }
        best = best.min((min_cut_of_phase, combined_count[t]));
        combined_count[s] += combined_count[t];

        // Contract node t into node s.
        for (i, e) in graph[&t].clone() {
            // It'd be nice if Rust allowed to do `graph[&i].remove(&t)`
            graph.get_mut(&i).unwrap().remove(&t);
            if i != s {
                *graph.get_mut(&s).unwrap().entry(i).or_insert(0) += e;
                *graph.get_mut(&i).unwrap().entry(s).or_insert(0) += e;
            }
        }
        graph.remove(&t);
    }

    best
}
