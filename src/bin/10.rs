use itertools::Itertools;
use std::isize;
advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug)]
enum Tile {
    Start,
    Ground,
    Pipe(Pipe),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

type Coordinate = (isize, isize);

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Self { tiles }
    }

    pub fn pipe_at(&self, coordinate: Coordinate) -> Option<Pipe> {
        let x: usize = coordinate.0.try_into().ok()?;
        let y: usize = coordinate.1.try_into().ok()?;

        match self.tiles.get(y)?.get(x)? {
            Tile::Start => panic!(),
            Tile::Ground => None,
            Tile::Pipe(pipe) => Some(*pipe),
        }
    }
}

fn parse_grid(input: &str) -> (Coordinate, Grid) {
    let mut start = None;

    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Tile::Ground,
                    'S' => {
                        start = Some((x as isize, y as isize));
                        Tile::Start
                    }
                    '|' => Tile::Pipe(Pipe::Vertical),
                    '-' => Tile::Pipe(Pipe::Horizontal),
                    'L' => Tile::Pipe(Pipe::NorthEast),
                    'J' => Tile::Pipe(Pipe::NorthWest),
                    '7' => Tile::Pipe(Pipe::SouthWest),
                    'F' => Tile::Pipe(Pipe::SouthEast),
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    (start.unwrap(), Grid::new(tiles))
}

fn neighbours(coordinate: Coordinate, pipe: Pipe) -> (Coordinate, Coordinate) {
    let (x, y) = coordinate;

    match pipe {
        Pipe::Vertical => ((x, y - 1), (x, y + 1)),
        Pipe::Horizontal => ((x - 1, y), (x + 1, y)),
        Pipe::NorthEast => ((x, y - 1), (x + 1, y)),
        Pipe::NorthWest => ((x, y - 1), (x - 1, y)),
        Pipe::SouthEast => ((x, y + 1), (x + 1, y)),
        Pipe::SouthWest => ((x, y + 1), (x - 1, y)),
    }
}

fn find_cycle_starting_at(
    start_coordinate: Coordinate,
    pipe: Pipe,
    grid: &Grid,
) -> Option<Vec<(isize, isize)>> {
    let mut cycle = vec![start_coordinate];

    let (_, c2) = neighbours(start_coordinate, pipe);
    let mut pipe = grid.pipe_at(c2)?;

    let mut previous_coordinate = start_coordinate;
    let mut coordinate = c2;
    cycle.push(coordinate);

    while start_coordinate != coordinate {
        let (c1, c2) = neighbours(coordinate, pipe);

        let next_coordinate = match (c1 == previous_coordinate, c2 == previous_coordinate) {
            (true, false) => c2,
            (false, true) => c1,
            _ => return None,
        };

        if next_coordinate == start_coordinate {
            break;
        }

        pipe = grid.pipe_at(next_coordinate)?;
        previous_coordinate = coordinate;
        coordinate = next_coordinate;

        cycle.push(coordinate);
    }

    Some(cycle)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (start_coordinate, grid) = parse_grid(input);

    let all_pipes = [
        Pipe::Horizontal,
        Pipe::Vertical,
        Pipe::NorthEast,
        Pipe::NorthWest,
        Pipe::SouthWest,
        Pipe::SouthEast,
    ];

    all_pipes
        .into_iter()
        .filter_map(|pipe| find_cycle_starting_at(start_coordinate, pipe, &grid))
        .next()
        .map(|cycle| cycle.len() / 2)
}

pub fn part_two(input: &str) -> Option<isize> {
    let (start_coordinate, grid) = parse_grid(input);

    let all_pipes = [
        Pipe::Horizontal,
        Pipe::Vertical,
        Pipe::NorthEast,
        Pipe::NorthWest,
        Pipe::SouthWest,
        Pipe::SouthEast,
    ];

    let cycle = all_pipes
        .into_iter()
        .filter_map(|pipe| find_cycle_starting_at(start_coordinate, pipe, &grid))
        .next()
        .unwrap();

    // Pick's theorem enables us to easily compute the number of points based on the area of the
    //  polygon which is obtained using Shoelace's algorithm.
    let area: isize = cycle
        .iter()
        .chain(std::iter::once(&cycle[0]))
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2))
        .sum();

    Some((area.abs() - cycle.len() as isize) / 2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(10));
    }
}
