use itertools::Itertools;
use std::collections::HashMap;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let height = grid.len();
    let width = grid[0].len();
    let start = (1, 0);
    let end = (width - 2, height - 1);

    let graph_p1 = build_graph(&grid, start, end, true);
    let graph_p2 = build_graph(&grid, start, end, false);
    let ans_1 = find_longest_path(&graph_p1, start, end);
    let ans_2 = find_longest_path(&graph_p2, start, end);

    println!("{ans_1} {ans_2}");

    Ok(())
}

type Point = (usize, usize);
type Graph = HashMap<Point, Vec<(Point, u32)>>;

fn build_graph(grid: &[Vec<char>], start: Point, end: Point, slippery_slope: bool) -> Graph {
    let mut graph: Graph = HashMap::new();
    graph.insert(start, vec![]);

    let mut unvisited = vec![(start, (start.0, start.1 + 1))];
    while let Some((node_pos, next_pos)) = unvisited.pop() {
        let mut prev_pos = node_pos;
        let (mut x, mut y) = next_pos;
        let mut steps = 1;
        loop {
            if (x, y) == end {
                graph.get_mut(&node_pos).unwrap().push((end, steps));
                break;
            }
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let walkable_neighbors = dirs.map(|(dx, dy)| {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                let Some(tile) = grid.get(ny).and_then(|row| row.get(nx)) else {
                    return None;
                };
                match tile {
                    'v' if dy == -1 && slippery_slope => return None,
                    '>' if dx == -1 && slippery_slope => return None,
                    '#' => return None,
                    _ => {}
                };
                Some((nx, ny))
            });

            let walkable_non_prev_neighbors = walkable_neighbors
                .map(|neigh| neigh.and_then(|pos| if pos == prev_pos { None } else { Some(pos) }));
            match walkable_non_prev_neighbors.iter().flatten().count() {
                0 => {
                    // Dead end.
                    break;
                }
                1 => {
                    let (nx, ny) = walkable_non_prev_neighbors.iter().find_map(|n| *n).unwrap();
                    prev_pos = (x, y);
                    (x, y) = (nx, ny);
                    steps += 1;
                }
                _ => {
                    graph.get_mut(&node_pos).unwrap().push(((x, y), steps));
                    graph.entry((x, y)).or_insert_with(|| {
                        for neigh_pos in walkable_neighbors.into_iter().flatten() {
                            unvisited.push(((x, y), neigh_pos));
                        }
                        vec![]
                    });
                    break;
                }
            }
        }
    }

    graph
}

fn find_longest_path(graph: &Graph, start: Point, end: Point) -> u32 {
    let mut max_cost = 0;
    let mut stack = vec![(vec![start], 0)];
    while let Some((path, path_cost)) = stack.pop() {
        let last_node = *path.last().unwrap();
        if last_node == end {
            max_cost = max_cost.max(path_cost);
            continue;
        }
        for &(node, node_cost) in graph[&last_node]
            .iter()
            .filter(|(node, _cost)| !path.contains(node))
        {
            let mut new_path = path.clone();
            new_path.push(node);
            stack.push((new_path, path_cost + node_cost));
        }
    }
    max_cost
}
