fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let mut grid: Grid = input
        .lines()
        .map(|l| l.chars().map(|ch| (ch, EMPTY)).collect())
        .collect();
    let mut beams = vec![(0, 0, RIGHT)];
    while let Some(beam) = beams.pop() {
        let (mut x, mut y, mut dir) = beam;
        loop {
            let tile = grid.get_mut(y).and_then(|row| row.get_mut(x));
            let Some((ch, tile_beams)) = tile else {
                // Beam has gone out of bounds.
                break;
            };
            if *tile_beams & dir != 0 {
                break;
            }
            *tile_beams |= dir;
            match (dir, *ch) {
                (_, '.') | (LEFT | RIGHT, '-') | (UP | DOWN, '|') => {}
                (RIGHT, '/') | (LEFT, '\\') => dir = UP,
                (RIGHT, '\\') | (LEFT, '/') => dir = DOWN,
                (UP, '/') | (DOWN, '\\') => dir = RIGHT,
                (UP, '\\') | (DOWN, '/') => dir = LEFT,
                (LEFT | RIGHT, '|') => {
                    beams.push((x, y.wrapping_sub(1), UP));
                    beams.push((x, y + 1, DOWN));
                    break;
                }
                (UP | DOWN, '-') => {
                    beams.push((x.wrapping_sub(1), y, LEFT));
                    beams.push((x + 1, y, RIGHT));
                    break;
                }
                _ => Err(format!("unexpected tile '{ch}' in direction {dir}"))?,
            }
            match dir {
                LEFT => x = x.wrapping_sub(1),
                RIGHT => x += 1,
                UP => y = y.wrapping_sub(1),
                DOWN => y += 1,
                _ => unreachable!(),
            }
        }
    }
    let mut energized_tiles_count = 0;
    for row in grid.iter() {
        for (_, tile_beams) in row.iter() {
            if *tile_beams != 0 {
                energized_tiles_count += 1;
            }
        }
    }
    println!("{energized_tiles_count}");
    Ok(())
}

type Grid = Vec<Vec<(char, u32)>>;
const EMPTY: u32 = 0;
const UP: u32 = 1;
const LEFT: u32 = 2;
const DOWN: u32 = 4;
const RIGHT: u32 = 8;
