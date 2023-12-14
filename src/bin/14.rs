use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::cmp::min;
use std::collections::VecDeque;
advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Space {
    FixedRock,
    RollingRock,
}

type Coordinate = (isize, isize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

fn roll_towards(
    grid: &mut FxHashMap<Coordinate, Space>,
    height: isize,
    width: isize,
    direction: Direction,
) {
    let mut movable_rocks: VecDeque<Coordinate> = grid
        .iter()
        .filter(|(_, space)| match space {
            Space::FixedRock => false,
            Space::RollingRock => true,
        })
        .map(|(coordinate, _)| *coordinate)
        .sorted_by_key(|(x, y)| match direction {
            Direction::North => *y,
            Direction::East => -x,
            Direction::South => -y,
            Direction::West => *x,
        })
        .collect();

    while let Some((x, y)) = movable_rocks.pop_front() {
        let next_coordinate = match direction {
            Direction::North => {
                let next_obstacle = (0..y)
                    .rev()
                    .find(|possible_y| grid.contains_key(&(x, *possible_y)));

                match next_obstacle {
                    None => (x, 0),
                    Some(next_obstacle) => (x, next_obstacle + 1),
                }
            }
            Direction::West => {
                let next_obstacle = (0..x)
                    .rev()
                    .find(|possible_x| grid.contains_key(&(*possible_x, y)));

                match next_obstacle {
                    None => (0, y),
                    Some(next_obstacle) => (next_obstacle + 1, y),
                }
            }
            Direction::East => {
                let next_obstacle =
                    (x + 1..=width).find(|possible_x| grid.contains_key(&(*possible_x, y)));

                match next_obstacle {
                    None => (width, y),
                    Some(next_obstacle) => (next_obstacle - 1, y),
                }
            }
            Direction::South => {
                let next_obstacle =
                    (y + 1..=height).find(|possible_y| grid.contains_key(&(x, *possible_y)));

                match next_obstacle {
                    None => (x, height),
                    Some(next_obstacle) => (x, next_obstacle - 1),
                }
            }
        };

        assert!(grid.remove(&(x, y)).is_some());
        assert!(grid.insert(next_coordinate, Space::RollingRock).is_none());
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut grid: FxHashMap<Coordinate, Space> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '.' => None,
                    '#' => Some(((x as isize, y as isize), Space::FixedRock)),
                    'O' => Some(((x as isize, y as isize), Space::RollingRock)),
                    _ => panic!(),
                })
        })
        .collect();

    let height = grid.keys().map(|(_, y)| *y).max().unwrap();
    let width = grid.keys().map(|(x, _)| *x).max().unwrap();

    roll_towards(&mut grid, height, width, Direction::North);

    Some(
        grid.iter()
            .filter(|(_, rock)| match rock {
                Space::FixedRock => false,
                Space::RollingRock => true,
            })
            .map(|((_, y), _)| height - *y + 1)
            .sorted()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut grid: FxHashMap<Coordinate, Space> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '.' => None,
                    '#' => Some(((x as isize, y as isize), Space::FixedRock)),
                    'O' => Some(((x as isize, y as isize), Space::RollingRock)),
                    _ => panic!(),
                })
        })
        .collect();

    let height = grid.keys().map(|(_, y)| *y).max().unwrap();
    let width = grid.keys().map(|(_, y)| *y).max().unwrap();

    let mut history_of_load: Vec<isize> = vec![];

    let cycles_required = 1000000000;
    let mut cycles_remaining = 1000000000;
    let mut cycle_found = false;

    while cycles_remaining > 0 {
        cycles_remaining -= 1;

        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            roll_towards(&mut grid, height, width, direction);
        }

        if cycle_found {
            continue;
        }

        let load = grid
            .iter()
            .filter(|(_, rock)| match rock {
                Space::FixedRock => false,
                Space::RollingRock => true,
            })
            .map(|((_, y), _)| height - *y + 1)
            .sum::<isize>();

        history_of_load.push(load);

        let cycle_length = (2..min(20, history_of_load.len() / 2)).find(|cycle_length| {
            if history_of_load.len() <= cycle_length * 2 {
                return false;
            }

            (0..*cycle_length).all(|offset| {
                let current = history_of_load[history_of_load.len() - offset - 1];
                let predecessor =
                    history_of_load[history_of_load.len() - cycle_length - offset - 1];

                current == predecessor
            })
        });

        if let Some(cycle_length) = cycle_length {
            let iteration = cycles_required - cycles_remaining;
            let number_of_cycles_remaining = (cycles_required - iteration) % cycle_length;

            cycles_remaining = number_of_cycles_remaining;
            cycle_found = true;
        }
    }

    Some(
        grid.iter()
            .filter(|(_, rock)| match rock {
                Space::FixedRock => false,
                Space::RollingRock => true,
            })
            .map(|((_, y), _)| height - *y + 1)
            .sum::<isize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
