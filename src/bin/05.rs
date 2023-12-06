use itertools::Itertools;
use std::cmp::{max, min};
use std::ops::Range;
advent_of_code::solution!(5);

#[derive(Clone, Debug)]
struct Map {
    range: Range<usize>,
    delta: isize,
}

fn parse_input(input: &str) -> (Vec<usize>, impl Iterator<Item = Vec<Map>> + '_) {
    let mut chunks = input.split("\n\n");
    let seeds: Vec<usize> = chunks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|number| number.parse::<usize>().unwrap())
        .collect();

    let maps = chunks.map(|maps| {
        maps.lines()
            .skip(1)
            .map(|map| {
                let mut data = map.split(' ');
                let destination_range_start: isize = data.next().unwrap().parse().unwrap();
                let source_range_start: usize = data.next().unwrap().parse().unwrap();
                let length: usize = data.next().unwrap().parse().unwrap();

                Map {
                    range: source_range_start..source_range_start + length,
                    delta: destination_range_start - source_range_start as isize,
                }
            })
            .collect()
    });

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, maps) = parse_input(input);

    maps.fold(seeds, |mut acc, maps| {
        acc.iter_mut().for_each(|current| {
            let relevant_map = maps.iter().find(|map| map.range.contains(current));

            if let Some(map) = relevant_map {
                *current = (*current as isize + map.delta) as usize
            }
        });

        acc
    })
    .into_iter()
    .min()
}

#[allow(clippy::type_complexity)]
fn cut_at(
    range: &Range<usize>,
    cut: &Range<usize>,
) -> Option<(Option<Range<usize>>, Range<usize>, Option<Range<usize>>)> {
    let overlap_start = max(range.start, cut.start);
    let overlap_end = min(range.end, cut.end);

    if overlap_start > overlap_end {
        return None;
    }

    Some((
        (overlap_start > range.start).then_some(range.start..overlap_start),
        (overlap_start..overlap_end),
        (overlap_end < range.end).then_some(overlap_end..range.end),
    ))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (seeds, maps) = parse_input(input);
    let seeds: Vec<Range<usize>> = seeds
        .into_iter()
        .tuples()
        .map(|(start, length)| (start..start + length))
        .collect();

    maps.fold(seeds, |ranges, maps| {
        let mut mapped_ranges = vec![];
        let mut ranges = ranges.clone();

        for map in maps {
            let mut unmapped_ranges = vec![];

            for range in ranges {
                let Some((start, middle, end)) = cut_at(&range, &map.range) else {
                    unmapped_ranges.push(range);
                    continue;
                };

                if let Some(start) = start {
                    unmapped_ranges.push(start);
                };

                if let Some(end) = end {
                    unmapped_ranges.push(end);
                }

                mapped_ranges.push(
                    (middle.start as isize + map.delta) as usize
                        ..(middle.end as isize + map.delta) as usize,
                )
            }

            ranges = unmapped_ranges;
        }

        mapped_ranges.extend(ranges);
        mapped_ranges
    })
    .into_iter()
    .map(|range| range.start)
    .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
