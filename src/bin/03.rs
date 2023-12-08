use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
advent_of_code::solution!(3);

#[derive(Debug)]
enum Observation {
    Symbol(Symbol),
    Value(Number),
}

#[derive(Debug, Clone)]
struct Symbol {
    point: (isize, isize),
    symbol: char,
}

#[derive(Debug, Clone)]
struct Number {
    points: Vec<(isize, isize)>,
    number: usize,
}

fn neighbours(coordinate: (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    let (x, y) = coordinate;

    [
        (x, y - 1),
        (x, y + 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x - 1, y - 1),
    ]
    .into_iter()
}

fn get_observations(input: &str) -> Vec<Observation> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut observations = Vec::new();
            let mut numbers = Vec::new();

            line.chars()
                .chain(std::iter::once('.'))
                .enumerate()
                .for_each(|(x, value)| {
                    if value.is_numeric() {
                        numbers.push(value.to_digit(10).unwrap());
                    } else if !numbers.is_empty() {
                        observations.push(Observation::Value(Number {
                            points: (0..numbers.len())
                                .map(|diff| ((x - diff - 1) as isize, y as isize))
                                .collect(),
                            number: numbers
                                .iter()
                                .rev()
                                .enumerate()
                                .map(|(index, number)| 10_u32.pow(index as u32) * number)
                                .sum::<u32>() as usize,
                        }));

                        numbers = vec![];
                    }

                    if value != '.' && !value.is_numeric() {
                        observations.push(Observation::Symbol(Symbol {
                            point: (x as isize, y as isize),
                            symbol: value,
                        }));
                    }
                });

            observations
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let observations = get_observations(input);

    let symbols: HashSet<(isize, isize)> = observations
        .iter()
        .filter_map(|observation| {
            let Observation::Symbol(symbol) = observation else {
                return None;
            };

            Some(symbol.point)
        })
        .collect();

    Some(
        observations
            .iter()
            .filter_map(|observation| {
                let Observation::Value(Number { points, number }) = observation else {
                    return None;
                };

                points
                    .iter()
                    .any(|point| neighbours(*point).any(|other| symbols.contains(&other)))
                    .then_some(number)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let observations = get_observations(input);

    let numbers: HashMap<(isize, isize), (usize, usize)> = observations
        .iter()
        .enumerate()
        .filter_map(|(index, observation)| match observation {
            Observation::Symbol(_) => None,
            Observation::Value(number) => Some(
                number
                    .points
                    .iter()
                    .map(move |point| (*point, (index, number.number))),
            ),
        })
        .flatten()
        .collect();

    Some(
        observations
            .iter()
            .filter_map(|observation| match observation {
                Observation::Value(_) => None,
                Observation::Symbol(symbol) => Some(symbol),
            })
            .filter(|symbol| symbol.symbol == '*')
            .filter_map(|symbol| {
                let neighbours: Vec<&(usize, usize)> = neighbours(symbol.point)
                    .filter_map(|point| numbers.get(&point))
                    .unique()
                    .collect();

                if neighbours.len() != 2 {
                    return None;
                }

                Some(neighbours[0].1 * neighbours[1].1)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
