use itertools::Itertools;

fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let games: Vec<_> = input.lines().map(Game::parse).try_collect()?;

    let ans_1: u32 = games.iter().filter(|g| g.is_possible()).map(|g| g.id).sum();
    let ans_2: u32 = games.iter().map(|g| g.minimum_set().power()).sum();
    println!("{ans_1} {ans_2}");
    Ok(())
}

struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn parse(s: &str) -> aoc::Result<Game> {
        let (id_part, sets_part) = s.split_once(": ").ok_or("malformed game line")?;
        let id = id_part.strip_prefix("Game ").ok_or("malformed game line")?;
        let id = id.parse()?;
        let sets = sets_part.split("; ").map(CubeSet::parse).try_collect()?;
        Ok(Game { id, sets })
    }

    fn is_possible(&self) -> bool {
        self.sets
            .iter()
            .all(|s| s.r <= 12 && s.g <= 13 && s.b <= 14)
    }

    fn minimum_set(&self) -> CubeSet {
        let mut res = CubeSet::default();
        for set in self.sets.iter() {
            res.r = res.r.max(set.r);
            res.g = res.g.max(set.g);
            res.b = res.b.max(set.b);
        }
        res
    }
}

#[derive(Default)]
struct CubeSet {
    r: u32,
    g: u32,
    b: u32,
}

impl CubeSet {
    fn parse(s: &str) -> aoc::Result<CubeSet> {
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
                _ => Err(format!("unexpected color '{color}'"))?,
            }
        }
        Ok(set)
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}
