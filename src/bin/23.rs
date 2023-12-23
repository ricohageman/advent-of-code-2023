use rustc_hash::FxHashMap;
use std::cmp::max;
use std::collections::VecDeque;
advent_of_code::solution!(23);

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

type Coordinate = (isize, isize);
type Grid = Vec<Vec<Tile>>;

fn parse_grid(input: &str, ignore_slopes: bool) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '>' => Tile::Slope(Direction::East),
                    'v' => Tile::Slope(Direction::South),
                    '<' => Tile::Slope(Direction::South),
                    '^' => Tile::Slope(Direction::North),
                    _ => panic!(),
                })
                .map(|tile| {
                    if ignore_slopes {
                        return match tile {
                            Tile::Path => Tile::Path,
                            Tile::Slope(_) => Tile::Path,
                            Tile::Forest => Tile::Forest,
                        };
                    }

                    tile
                })
                .collect()
        })
        .collect()
}

fn walkable_neighbours(
    current: Coordinate,
    grid: &Grid,
    visited_coordinates: &[Coordinate],
    previous_direction: Option<Direction>,
) -> Vec<(Direction, Coordinate)> {
    let (x, y) = current;

    [
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::North,
    ]
    .into_iter()
    .filter(|direction| {
        if let Some(previous_direction) = previous_direction {
            let forbidden_move = match previous_direction {
                Direction::North => Direction::South,
                Direction::East => Direction::West,
                Direction::South => Direction::North,
                Direction::West => Direction::East,
            };

            return *direction != forbidden_move;
        }

        true
    })
    .filter_map(|direction| {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        let x = x + dx;
        let y = y + dy;

        let can_follow = match grid[y as usize][x as usize] {
            Tile::Forest => false,
            Tile::Path => true,
            Tile::Slope(slope_direction) => slope_direction == direction,
        };

        if !can_follow {
            return None;
        }

        Some((direction, (x, y)))
    })
    .filter(|(_, coordinate)| !visited_coordinates.contains(coordinate))
    .collect()
}

fn walk_to_next_split(
    coordinate: Coordinate,
    grid: &Grid,
    visited_coordinates: &[Coordinate],
    previous_direction: Option<Direction>,
) -> (Coordinate, usize, Direction) {
    let target = ((grid[0].len() - 2) as isize, (grid.len() - 1) as isize);

    let mut coordinate = coordinate;
    let mut length = 0;
    let mut previous_direction = previous_direction;

    let mut neighbours =
        walkable_neighbours(coordinate, grid, visited_coordinates, previous_direction);

    while neighbours.len() == 1 {
        coordinate = neighbours[0].1;
        previous_direction = Some(neighbours[0].0);
        length += 1;

        if coordinate == target {
            break;
        }

        neighbours = walkable_neighbours(coordinate, grid, visited_coordinates, previous_direction);
    }

    (coordinate, length, previous_direction.unwrap())
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_grid(input, false);

    let mut queue: VecDeque<(Coordinate, usize, Vec<Coordinate>, Direction)> = VecDeque::new();
    queue.push_front(((1, 0), 0, vec![], Direction::South));

    let target = ((grid[0].len() - 2) as isize, (grid.len() - 1) as isize);
    let mut maximum_duration = 0;

    while let Some((current, length, mut visited_coordinates, previous_direction)) =
        queue.pop_front()
    {
        let (current, additional_length, previous_direction) = walk_to_next_split(
            current,
            &grid,
            &visited_coordinates,
            Some(previous_direction),
        );

        if current == target {
            maximum_duration = max(length + additional_length, maximum_duration);
            continue;
        }

        let neighbours = walkable_neighbours(
            current,
            &grid,
            &visited_coordinates,
            Some(previous_direction),
        );

        if neighbours.is_empty() {
            continue;
        }

        visited_coordinates.push(current);

        // There is a decision to be made
        for (direction, coordinate) in neighbours {
            queue.push_front((
                coordinate,
                length + additional_length + 1,
                visited_coordinates.clone(),
                direction,
            ))
        }
    }

    Some(maximum_duration)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_grid(input, true);
    let target = ((grid[0].len() - 2) as isize, (grid.len() - 1) as isize);

    // (1) Find the first point where a decision must be made.
    let (initial_coordinate, initial_length, previous_direction) =
        walk_to_next_split((1, 0), &grid, &[], Some(Direction::South));

    // (2) Reduce the grid into a smaller grid
    let mut possible_ways: FxHashMap<Coordinate, Vec<(Coordinate, usize)>> = FxHashMap::default();

    let mut queue: VecDeque<(Coordinate, Option<Direction>)> = VecDeque::new();
    queue.push_front((initial_coordinate, Some(previous_direction)));

    while let Some((coordinate, previous_direction)) = queue.pop_front() {
        let targets: Vec<(Coordinate, usize)> =
            walkable_neighbours(coordinate, &grid, &[], previous_direction)
                .into_iter()
                .map(|(direction, coordinate)| {
                    let (coordinate, length, _) =
                        walk_to_next_split(coordinate, &grid, &[], Some(direction));

                    (coordinate, length)
                })
                .collect();

        for (coordinate, _) in &targets {
            if !possible_ways.contains_key(coordinate) && *coordinate != target {
                queue.push_back((*coordinate, None));
            }
        }

        possible_ways.insert(coordinate, targets);
    }

    // (3) Find the longest path using DFS
    let mut queue: VecDeque<(Coordinate, Vec<Coordinate>, usize)> = VecDeque::new();
    queue.push_front((initial_coordinate, vec![], initial_length));

    let mut maximum_length = 0;

    while let Some((coordinate, previous_coordinates, length)) = queue.pop_front() {
        for (next_coordinate, additional_length) in &possible_ways[&coordinate] {
            if *next_coordinate == target {
                maximum_length = max(maximum_length, length + additional_length + 1);
                continue;
            }

            if previous_coordinates.contains(next_coordinate) {
                continue;
            }

            let mut previous_coordinates = previous_coordinates.clone();
            previous_coordinates.push(coordinate);

            queue.push_front((
                *next_coordinate,
                previous_coordinates,
                length + additional_length + 1,
            ))
        }
    }

    Some(maximum_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
