fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let hash_sum: usize = input.trim_end().split(',').map(hash).sum();

    let mut boxes = vec![vec![]; 256];
    for step in input.trim_end().split(',') {
        let step = parse_step(step)?;
        match step {
            Step::Remove(label) => boxes[hash(label)].retain(|&(l, _)| l != label),
            Step::Put(label, focal_length) => {
                let lens = boxes[hash(label)].iter_mut().find(|(l, _)| *l == label);
                if let Some(existing_lens) = lens {
                    existing_lens.1 = focal_length;
                } else {
                    boxes[hash(label)].push((label, focal_length))
                }
            }
        }
    }

    let mut total_focusing_power = 0;
    for (lenses, box_index) in boxes.iter().zip(1..) {
        for ((_l, focal_length), lens_index) in lenses.iter().zip(1..) {
            total_focusing_power += box_index * lens_index * focal_length;
        }
    }

    println!("{hash_sum} {total_focusing_power}");
    Ok(())
}

enum Step<'a> {
    Remove(&'a str),
    Put(&'a str, u64),
}

fn parse_step(s: &str) -> aoc::Result<Step> {
    if let Some(label) = s.strip_suffix('-') {
        return Ok(Step::Remove(label));
    }
    if let Some((label, focal_length)) = s.split_once('=') {
        let focal_length = focal_length.parse()?;
        return Ok(Step::Put(label, focal_length));
    }
    Err(format!("invalid step: '{s}'"))?
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |acc, b| (acc + b as usize) * 17 % 256)
}
