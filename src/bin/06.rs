use itertools::Itertools;
advent_of_code::solution!(6);

fn line_as_numbers(line: &str) -> Vec<usize> {
    line.split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .filter(|time| !time.is_empty())
        .map(|time| time.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let times = line_as_numbers(lines.next().unwrap());
    let records = line_as_numbers(lines.next().unwrap());

    Some(
        times
            .into_iter()
            .zip_eq(records)
            .map(|(time, record)| {
                (1..time)
                    .filter(|hold| (time - hold) * hold > record)
                    .count()
            })
            .product(),
    )
}

fn line_as_single_number(line: &str) -> usize {
    line.split_once(": ")
        .unwrap()
        .1
        .chars()
        .filter(|char| *char != ' ')
        .map(|time| time.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, value)| 10_usize.pow(index as u32) * value)
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let time = line_as_single_number(lines.next().unwrap());
    let record = line_as_single_number(lines.next().unwrap());

    let mut low: usize = 0;
    let mut high: usize = time;

    while low <= high {
        let hold = ((high - low) / 2) + low;

        if (time - hold) * hold > record {
            // If the current value yields a win, we can wait less.
            high = hold - 1;
        } else {
            // If the current value loses, we need to wait longer.
            low = hold + 1;
        }
    }

    Some(time - (2 * low) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
