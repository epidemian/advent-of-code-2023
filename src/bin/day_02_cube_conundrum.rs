use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let games: Vec<_> = input.lines().map(parse_game).collect::<Result<_, _>>()?;
    let ans_1: u32 = games
        .iter()
        .filter(|g| is_possible_game(*g))
        .map(|g| g.id)
        .sum();
    println!("{ans_1}");
    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

#[derive(Debug, Default)]
struct CubeSet {
    r: u32,
    g: u32,
    b: u32,
}

fn is_possible_game(game: &Game) -> bool {
    game.sets
        .iter()
        .all(|s| s.r <= 12 && s.g <= 13 && s.b <= 14)
}

fn parse_game(s: &str) -> Result<Game, Box<dyn Error>> {
    let (id_part, sets_part) = s.split_once(": ").ok_or("expected line to have a ':'")?;
    let (_, id) = id_part
        .split_once(' ')
        .ok_or("expected to have a space before id")?;
    let id = id.parse()?;
    let sets = sets_part
        .split("; ")
        .map(parse_set)
        .collect::<Result<_, _>>()?;
    Ok(Game { id, sets })
}

fn parse_set(s: &str) -> Result<CubeSet, Box<dyn Error>> {
    let mut set = CubeSet::default();
    for cubes_str in s.split(", ") {
        let (n, color) = cubes_str
            .split_once(' ')
            .ok_or("expected space between number and color")?;
        let n: u32 = n.parse()?;
        match color {
            "red" => set.r += n,
            "green" => set.g += n,
            "blue" => set.b += n,
            _ => return Err(format!("unexpected color '{color}'").into()),
        }
    }
    Ok(set)
}
