use anyhow::ensure;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let ans_1 = find_longest_path(&build_graph(&grid, true))?;
    let ans_2 = find_longest_path(&build_graph(&grid, false))?;
    println!("{ans_1} {ans_2}");

    Ok(())
}

type Grid = Vec<Vec<char>>;
type Graph = Vec<Vec<(usize, u32)>>;

const START: usize = 0;
const END: usize = 1;

fn build_graph(grid: &Grid, slippery_slope: bool) -> Graph {
    let height = grid.len();
    let width = grid[0].len();
    let start_pos = (1, 0);
    let end_pos = (width - 2, height - 1);

    let mut node_indices = HashMap::<_, _>::from_iter([(start_pos, START), (end_pos, END)]);
    let mut graph = vec![vec![], vec![]];

    let mut unvisited = vec![(start_pos, (start_pos.0, start_pos.1 + 1))];
    while let Some((node_pos, next_pos)) = unvisited.pop() {
        let node_idx = node_indices[&node_pos];
        let mut prev_pos = node_pos;
        let (mut x, mut y) = next_pos;
        let mut steps = 1;
        loop {
            if (x, y) == end_pos {
                graph[node_idx].push((END, steps));
                break;
            }
            let walkable_neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(|(dx, dy)| {
                    let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                    let tile = grid.get(ny)?.get(nx)?;
                    match tile {
                        'v' if dy == -1 && slippery_slope => None,
                        '>' if dx == -1 && slippery_slope => None,
                        '#' => None,
                        _ => Some((nx, ny)),
                    }
                })
                .collect_vec();

            let non_prev = walkable_neighbors.iter().filter(|&&pos| pos != prev_pos);
            if let Some((neigh_pos,)) = non_prev.collect_tuple() {
                // Inside a corridor; keep waling.
                prev_pos = (x, y);
                (x, y) = *neigh_pos;
                steps += 1;
            } else {
                // On an intersection.
                let new_node_idx = *node_indices.entry((x, y)).or_insert_with(|| {
                    for neigh_pos in walkable_neighbors {
                        unvisited.push(((x, y), neigh_pos));
                    }
                    graph.push(vec![]);
                    graph.len() - 1
                });
                graph[node_idx].push((new_node_idx, steps));
                break;
            }
        }
    }

    graph
}

fn find_longest_path(graph: &Graph) -> aoc::Result<u32> {
    ensure!(graph.len() <= 64, "graph is too big for bitmask size");
    let mut max_cost = 0;
    let mut to_visit = vec![(START, 1_u64, 0)];
    while let Some((last_node, path_bitmask, path_cost)) = to_visit.pop() {
        if last_node == END {
            max_cost = max_cost.max(path_cost);
            continue;
        }
        for &(node, node_cost) in graph[last_node].iter() {
            if (path_bitmask & (1 << node)) == 0 {
                let new_bitmask = path_bitmask | (1 << node);
                let new_cost = path_cost + node_cost;
                to_visit.push((node, new_bitmask, new_cost));
            }
        }
    }
    Ok(max_cost)
}
