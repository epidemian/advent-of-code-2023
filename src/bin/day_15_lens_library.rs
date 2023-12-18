fn main() -> aoc::Result<()> {
    let input = aoc::read_stdin()?;
    let hash_sum: usize = input.trim_end().split(',').map(hash).sum();

    let mut boxes = vec![vec![]; 256];
    for step in input.trim_end().split(',') {
        let (label, step_type) = parse_step(step)?;
        let box_lenses = &mut boxes[hash(label)];
        match step_type {
            StepType::Remove => box_lenses.retain(|&(l, _)| l != label),
            StepType::Put(focal_length) => {
                if let Some(lens) = box_lenses.iter_mut().find(|(l, _)| *l == label) {
                    lens.1 = focal_length;
                } else {
                    box_lenses.push((label, focal_length))
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

enum StepType {
    Remove,
    Put(u64),
}

fn parse_step(s: &str) -> aoc::Result<(&str, StepType)> {
    if let Some(label) = s.strip_suffix('-') {
        return Ok((label, StepType::Remove));
    }
    if let Some((label, focal_length)) = s.split_once('=') {
        let focal_length = focal_length.parse()?;
        return Ok((label, StepType::Put(focal_length)));
    }
    Err(format!("invalid step: '{s}'"))?
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |acc, b| (acc + b as usize) * 17 % 256)
}
