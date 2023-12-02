advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let numbers: Vec<u32> = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();

                numbers[0] * 10 + numbers.last().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mapping = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    Some(
        input
            .lines()
            .map(|line| {
                let mut line = line.to_string();

                for (from, to) in mapping {
                    line = line.replace(from, to);
                }

                let numbers: Vec<u32> = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();

                numbers[0] * 10 + numbers.last().unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
