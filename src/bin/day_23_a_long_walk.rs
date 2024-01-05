use std::collections::HashMap;

use anyhow::{ensure, Context};
use itertools::Itertools;
use pathfinding::directed::yen::yen;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let height = grid.len();
    let width = grid[0].len();
    let start = (1, 0);
    let end = (width - 2, height - 1);

    let graph_p1 = build_graph(&grid, start, end, true);
    let graph_p2 = build_graph(&grid, start, end, false);
    let ans_1 = find_longest_path(&graph_p1, start, end)?;
    let ans_2 = find_longest_path(&graph_p2, start, end)?;

    println!("{ans_1} {ans_2}");

    Ok(())
}

type Point = (usize, usize);
type Graph = HashMap<Point, HashMap<Point, u32>>;

fn build_graph(grid: &[Vec<char>], start: Point, end: Point, slippery_slope: bool) -> Graph {
    let mut graph: Graph = HashMap::new();
    let mut unvisited = vec![(start, (start.0, start.1 + 1))];
    graph.insert(start, HashMap::new());

    while let Some((node_pos, next_pos)) = unvisited.pop() {
        // println!("start walk at {node_pos:?} to {next_pos:?}");
        let mut prev_pos = node_pos;
        let (mut x, mut y) = next_pos;
        let mut steps = 1;
        loop {
            if (x, y) == end {
                graph.get_mut(&node_pos).unwrap().insert(end, steps);
                break;
            }
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let neighbors = dirs.map(|(dx, dy)| {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                let Some(tile) = grid.get(ny).and_then(|row| row.get(nx)) else {
                    return None;
                };
                if (nx, ny) == prev_pos {
                    return None;
                }
                match tile {
                    'v' if dy == -1 && slippery_slope => return None,
                    '>' if dx == -1 && slippery_slope => return None,
                    '#' => return None,
                    _ => {}
                };
                Some((nx, ny))
            });

            let valid_neighbors = neighbors.iter().filter_map(|n| *n);
            match valid_neighbors.clone().count() {
                0 => {
                    println!("dead end {x},{y}");
                    break;
                }
                1 => {
                    let (nx, ny) = valid_neighbors.clone().next().unwrap();
                    prev_pos = (x, y);
                    (x, y) = (nx, ny);
                    steps += 1;
                }
                _ => {
                    graph.get_mut(&node_pos).unwrap().insert((x, y), steps);
                    #[allow(clippy::map_entry)]
                    if !graph.contains_key(&(x, y)) {
                        graph.insert((x, y), HashMap::new());
                        for neigh_pos in valid_neighbors {
                            unvisited.push(((x, y), neigh_pos));
                        }
                    }
                    break;
                }
            }
        }
    }

    // for (k, v) in graph.iter() {
    //     println!("{k:?} -> {v:?}");
    // }
    graph
}

fn find_longest_path(graph: &Graph, start: Point, end: Point) -> aoc::Result<u32> {
    let k = 1000;
    let k_shortest_paths = yen(
        &start,
        |&pos| {
            let nodes = graph.get(&pos).unwrap();
            nodes.iter().map(|(&pos, &cost)| (pos, cost))
        },
        |&pos| pos == end,
        k,
    );
    ensure!(k_shortest_paths.len() < k, "Found too many paths");

    let (_longest_path, longest_path_length) = &k_shortest_paths
        .into_iter()
        .max_by_key(|(_path, cost)| *cost)
        .context("no paths found")?;

    Ok(*longest_path_length)
}
