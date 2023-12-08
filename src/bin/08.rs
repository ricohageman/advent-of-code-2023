use gcd::Gcd;
use rustc_hash::{FxHashMap, FxHashSet};
advent_of_code::solution!(8);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialOrd, PartialEq, Ord)]
struct Node {
    inner: [char; 3],
}

impl Node {
    pub fn new(values: [char; 3]) -> Self {
        Self { inner: values }
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Self::new(
            value
                .chars()
                .filter(|c| *c != '(' && *c != ')')
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
        )
    }
}

fn solve<F>(
    from: usize,
    map: &[(usize, usize)],
    sequence: &[Direction],
    target_evaluation: F,
) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut current = from;
    let mut steps = 0;

    for direction in sequence.iter().cycle() {
        steps += 1;

        let (left, right) = map[current];

        current = match direction {
            Direction::Left => left,
            Direction::Right => right,
        };

        if target_evaluation(current) {
            break;
        }
    }

    steps
}

struct NodeIdentifierRegistry {
    inner: FxHashMap<Node, usize>,
    starts: FxHashSet<usize>,
    targets: FxHashSet<usize>,
    start_function: Box<fn(&Node) -> bool>,
    target_function: Box<fn(&Node) -> bool>,
}

impl NodeIdentifierRegistry {
    pub fn new(
        start_function: Box<fn(&Node) -> bool>,
        target_function: Box<fn(&Node) -> bool>,
    ) -> Self {
        Self {
            inner: FxHashMap::default(),
            starts: FxHashSet::default(),
            targets: FxHashSet::default(),
            start_function,
            target_function,
        }
    }

    pub fn register(&mut self, node: Node) {
        let value = self.inner.len();
        self.inner.insert(node, value);

        if (self.target_function)(&node) {
            self.targets.insert(value);
        }

        if (self.start_function)(&node) {
            self.starts.insert(value);
        }
    }

    pub fn get(&mut self, node: &Node) -> usize {
        *self.inner.get(node).unwrap()
    }
}

fn parse(
    input: &str,
    registry: &mut NodeIdentifierRegistry,
) -> (Vec<Direction>, Vec<(usize, usize)>) {
    let mut lines = input.lines();
    let sequence: Vec<Direction> = lines.next().unwrap().chars().map(Direction::from).collect();

    let transitions: Vec<(usize, usize)> = lines
        .skip(1)
        .map(|line| {
            let (departure, targets) = line.split_once(" = ").unwrap();
            let (left, right) = targets.split_once(", ").unwrap();

            registry.register(departure.into());

            (left.into(), right.into())
        })
        .collect::<Vec<(Node, Node)>>()
        .into_iter()
        .map(|(left, right)| (registry.get(&left), registry.get(&right)))
        .collect();

    (sequence, transitions)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut registry = NodeIdentifierRegistry::new(
        Box::new(|node| node.inner == ['A', 'A', 'A']),
        Box::new(|node| node.inner == ['Z', 'Z', 'Z']),
    );

    let (sequence, transitions) = parse(input, &mut registry);

    Some(solve(
        *registry.starts.iter().next().unwrap(),
        &transitions,
        &sequence,
        |node| registry.targets.contains(&node),
    ))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut registry = NodeIdentifierRegistry::new(
        Box::new(|node| node.inner[2] == 'A'),
        Box::new(|node| node.inner[2] == 'Z'),
    );

    let (sequence, transitions) = parse(input, &mut registry);

    Some(
        registry
            .starts
            .iter()
            .map(|starting_position| {
                solve(*starting_position, &transitions, &sequence, |node| {
                    registry.targets.contains(&node)
                })
            })
            .fold(1, |acc, next| (acc * next) / acc.gcd(next)),
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
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
