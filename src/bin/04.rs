use rustc_hash::FxHashSet as HashSet;
advent_of_code::solution!(4);

fn number_of_winning_numbers_per_game(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|game| {
        let game = game.replace("  ", " ");

        let (winning_numbers, numbers) =
            game.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let winning_numbers: HashSet<usize> = winning_numbers
            .split(' ')
            .map(|number| number.parse::<usize>().unwrap())
            .collect();

        numbers
            .split(' ')
            .map(|number| number.parse::<usize>().unwrap())
            .filter(|number| winning_numbers.contains(number))
            .count()
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        number_of_winning_numbers_per_game(input)
            .map(
                |number_of_winning_numbers| match number_of_winning_numbers {
                    0 => 0,
                    1 => 1,
                    _ => 2_usize.pow((number_of_winning_numbers - 1) as u32),
                },
            )
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        number_of_winning_numbers_per_game(input)
            .enumerate()
            .fold(
                vec![1; input.lines().count()],
                |mut acc, (offset, winning_numbers)| {
                    let number_of_cards_available = acc[offset];

                    (1..=winning_numbers)
                        .for_each(|index| acc[offset + index] += number_of_cards_available);

                    acc
                },
            )
            .iter()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
