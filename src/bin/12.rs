use rustc_hash::FxHashMap;
advent_of_code::solution!(12);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

const REPETITION_FACTOR: usize = 5;

fn solve_recursively_with_broken_spring_at_index(
    springs: &[Spring],
    groups: &[usize],
    spring_index: usize,
    group_index: usize,
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    // If there is no more group to add, then we should not encounter a broken spring.
    //  This means we made an invalid step previously and this branch can be discarded.
    let Some(&required_group_size) = groups.get(group_index) else {
        return 0;
    };

    // This branch can be discarded because the group is larger than the remaining number of springs.
    if spring_index + required_group_size > springs.len() {
        return 0;
    }

    let subsequent_springs_can_be_broken =
        (1..required_group_size).all(|offset| match springs[spring_index + offset] {
            Spring::Operational => false,
            Spring::Broken => true,
            Spring::Unknown => true,
        });

    if !subsequent_springs_can_be_broken {
        return 0;
    }

    let group_can_be_closed = match springs.get(spring_index + required_group_size) {
        None => true,
        Some(Spring::Operational) => true,
        Some(Spring::Unknown) => true,
        Some(Spring::Broken) => false,
    };

    if !group_can_be_closed {
        return 0;
    }

    solve_recursively(
        springs,
        groups,
        spring_index + required_group_size + 1,
        group_index + 1,
        cache,
    )
}

fn solve_recursively(
    springs: &[Spring],
    groups: &[usize],
    spring_index: usize,
    group_index: usize,
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if let Some(result) = cache.get(&(spring_index, group_index)) {
        return *result;
    }

    if spring_index >= springs.len() {
        let result = groups.get(group_index).is_none() as usize;
        cache.insert((spring_index, group_index), result);
        return result;
    }

    let number_of_valid_branches_without_adding_broken_springs = match springs[spring_index] {
        Spring::Broken => 0,
        Spring::Operational | Spring::Unknown => {
            solve_recursively(springs, groups, spring_index + 1, group_index, cache)
        }
    };

    let number_of_valid_branches_with_adding_broken_springs = match springs[spring_index] {
        Spring::Operational => 0,
        Spring::Broken | Spring::Unknown => solve_recursively_with_broken_spring_at_index(
            springs,
            groups,
            spring_index,
            group_index,
            cache,
        ),
    };

    let result = number_of_valid_branches_without_adding_broken_springs
        + number_of_valid_branches_with_adding_broken_springs;

    cache.insert((spring_index, group_index), result);

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let (springs, groups) = line.split_once(' ').unwrap();
                let springs: Vec<Spring> = springs
                    .chars()
                    .map(|char| match char {
                        '?' => Spring::Unknown,
                        '.' => Spring::Operational,
                        '#' => Spring::Broken,
                        _ => panic!(),
                    })
                    .collect();

                let groups: Vec<usize> = groups
                    .split(',')
                    .map(|amount| amount.parse().unwrap())
                    .collect();

                solve_recursively(&springs, &groups, 0, 0, &mut FxHashMap::default())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let (springs, groups) = line.split_once(' ').unwrap();
                let springs: Vec<Spring> = springs
                    .chars()
                    .map(|char| match char {
                        '?' => Spring::Unknown,
                        '.' => Spring::Operational,
                        '#' => Spring::Broken,
                        _ => panic!(),
                    })
                    .chain(std::iter::once(Spring::Unknown))
                    .cycle()
                    .take((springs.len() + 1) * REPETITION_FACTOR - 1)
                    .collect();

                let groups: Vec<usize> = groups
                    .split(',')
                    .map(|amount| amount.parse().unwrap())
                    .collect();

                let number_of_groups = groups.len();

                let groups: Vec<usize> = groups
                    .into_iter()
                    .cycle()
                    .take(number_of_groups * REPETITION_FACTOR)
                    .collect();

                solve_recursively(&springs, &groups, 0, 0, &mut FxHashMap::default())
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("???.### 1,1,3"), Some(1));
        assert_eq!(part_one(".??..??...?##. 1,1,3"), Some(4));
        assert_eq!(part_one("?#?#?#?#?#?#?#? 1,3,1,6"), Some(1));
        assert_eq!(part_one("????.#...#... 4,1,1"), Some(1));
        assert_eq!(part_one("????.######..#####. 1,6,5"), Some(4));
        assert_eq!(part_one("?###???????? 3,2,1"), Some(10));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("???.### 1,1,3"), Some(1));
        assert_eq!(part_two(".??..??...?##. 1,1,3"), Some(16384));
        assert_eq!(part_two("?#?#?#?#?#?#?#? 1,3,1,6"), Some(1));
        assert_eq!(part_two("????.#...#... 4,1,1"), Some(16));
        assert_eq!(part_two("????.######..#####. 1,6,5"), Some(2500));
        assert_eq!(part_two("?###???????? 3,2,1"), Some(506250));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
