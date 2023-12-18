use itertools::Itertools;
advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn transition(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}

fn content_of_lagoon(directions: &[(Direction, isize)]) -> isize {
    let (points, exterior) = directions.iter().fold(
        (vec![(0, 0)], 0),
        |(mut acc, exterior), (direction, length)| {
            let (x, y) = acc.last().unwrap();
            let (dx, dy) = direction.transition();

            acc.push((x + dx * length, y + dy * length));

            (acc, exterior + length)
        },
    );

    let area: isize = points
        .iter()
        .tuple_windows()
        .map(|((x_minus, _), (_, y), (x_plus, _))| y * (x_minus - x_plus))
        .sum();

    area / 2 + exterior / 2 + 1
}

pub fn part_one(input: &str) -> Option<isize> {
    let data: Vec<(Direction, isize)> = input
        .lines()
        .map(|line| {
            let mut data = line.split(' ');
            let direction = match data.next() {
                Some("U") => Direction::Up,
                Some("D") => Direction::Down,
                Some("R") => Direction::Right,
                Some("L") => Direction::Left,
                _ => panic!(),
            };

            let length: isize = data.next().unwrap().parse().unwrap();

            (direction, length)
        })
        .collect();

    Some(content_of_lagoon(&data))
}

pub fn part_two(input: &str) -> Option<isize> {
    let data: Vec<(Direction, isize)> = input
        .lines()
        .map(|line| {
            let hexadecimal_number = line.split(' ').nth(2).unwrap();
            let length = isize::from_str_radix(&hexadecimal_number[2..7], 16).unwrap();
            let direction = match hexadecimal_number.chars().nth(7) {
                Some('0') => Direction::Right,
                Some('1') => Direction::Down,
                Some('2') => Direction::Left,
                Some('3') => Direction::Up,
                _ => panic!(),
            };

            (direction, length)
        })
        .collect();

    Some(content_of_lagoon(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
