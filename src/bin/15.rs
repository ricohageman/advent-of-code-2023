use rustc_hash::FxHashMap;
advent_of_code::solution!(15);

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, char| ((acc + char as u32) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split(',').map(hash).sum())
}

#[derive(Debug)]
enum Command {
    RemoveLensWithLabel {
        boks: u32,
        label: String,
    },
    AddLens {
        boks: u32,
        label: String,
        focal_length: u32,
    },
}

pub fn part_two(input: &str) -> Option<usize> {
    let commands: Vec<Command> = input
        .split(',')
        .map(|command| {
            if command.ends_with('-') {
                let label = command[0..command.len() - 1].to_string();

                return Command::RemoveLensWithLabel {
                    boks: hash(&label),
                    label,
                };
            }

            let (label, focus_length) = command.split_once('=').unwrap();
            Command::AddLens {
                boks: hash(label),
                label: label.to_string(),
                focal_length: focus_length.parse().unwrap(),
            }
        })
        .collect();

    let mut boxes: Vec<FxHashMap<String, (usize, u32)>> =
        (0..=255).map(|_| FxHashMap::default()).collect();

    for command in commands {
        match command {
            Command::AddLens {
                boks,
                label,
                focal_length,
            } => {
                let boks = &mut boxes[boks as usize];
                let current_size = boks.len();

                boks.entry(label)
                    .and_modify(|(_, current_focal_length)| *current_focal_length = focal_length)
                    .or_insert((current_size, focal_length));
            }
            Command::RemoveLensWithLabel { boks, label } => {
                let boks = &mut boxes[boks as usize];

                let Some((previous_index, _)) = boks.remove(&label) else {
                    // If no lens in that box has the given label, nothing happens.
                    continue;
                };

                boks.iter_mut().for_each(|(_, (index, _))| {
                    if *index > previous_index {
                        *index -= 1;
                    }
                });
            }
        }
    }

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(box_index, lenses)| {
                lenses
                    .iter()
                    .map(|(_, (lens_index, focal_length))| {
                        (1 + box_index) * (1 + lens_index) * (*focal_length as usize)
                    })
                    .sum::<usize>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("HASH");
        assert_eq!(result, Some(52));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
