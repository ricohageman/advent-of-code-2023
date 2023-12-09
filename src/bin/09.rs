use itertools::Itertools;
advent_of_code::solution!(9);

fn parse_and_create_derivatives(
    input: &str,
) -> impl Iterator<Item = (Vec<isize>, Vec<Vec<isize>>)> + '_ {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|history| {
            let mut derivatives: Vec<Vec<isize>> =
                vec![history.iter().tuple_windows().map(|(a, b)| b - a).collect()];

            while !derivatives.last().unwrap().iter().all_equal() {
                derivatives.push(
                    derivatives
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect(),
                );
            }

            (history, derivatives)
        })
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(
        parse_and_create_derivatives(input)
            .map(|(history, derivatives)| {
                derivatives
                    .iter()
                    .rev()
                    .chain(std::iter::once(&history))
                    .map(|derivative| derivative.last().unwrap())
                    .sum::<isize>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(
        parse_and_create_derivatives(input)
            .map(|(history, derivatives)| {
                derivatives
                    .iter()
                    .rev()
                    .chain(std::iter::once(&history))
                    .map(|derivative| derivative[0])
                    .fold(0, |acc, last| last - acc)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(28));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(68));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(-3));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(0));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(5));
    }
}
