use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::cmp::{max, min};
advent_of_code::solution!(11);

fn solve(input: &str, expansion: isize) -> isize {
    let galaxies: Vec<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| *char == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    let columns_with_galaxies: FxHashSet<isize> = galaxies.iter().map(|(x, _)| *x).collect();

    let rows_with_galaxies: FxHashSet<isize> = galaxies.iter().map(|(_, y)| *y).collect();

    let columns_without_galaxies: Vec<isize> = (0..*columns_with_galaxies.iter().max().unwrap())
        .filter(|x| !columns_with_galaxies.contains(x))
        .collect();

    let rows_without_galaxies: Vec<isize> = (0..*rows_with_galaxies.iter().max().unwrap())
        .filter(|y| !rows_with_galaxies.contains(y))
        .collect();

    galaxies
        .iter()
        .map(|(x, y)| {
            let dx = columns_without_galaxies.iter().filter(|c| *c < x).count() as isize;

            let dy = rows_without_galaxies.iter().filter(|c| *c < y).count() as isize;

            ((x, dx), (y, dy))
        })
        .tuple_combinations()
        .map(|(((x1, dx1), (y1, dy1)), ((x2, dx2), (y2, dy2)))| {
            (x1 - x2).abs()
                + (y1 - y2).abs()
                + ((dx1 - dx2).abs() + (dy1 - dy2).abs()) * (expansion - 1)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(solve(input, 1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(result, 374);

        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 1030);

        let result = solve(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, 8410);
    }
}
