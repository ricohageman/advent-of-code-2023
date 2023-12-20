use rustc_hash::FxHashMap;
use std::cmp::{max, min};
use std::collections::VecDeque;
advent_of_code::solution!(19);

#[derive(Debug)]
enum Value {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Condition {
    Higher,
    Lower,
}

#[derive(Debug)]
enum Reference {
    Conditional(Value, Condition, usize, String),
    Other(String),
    Accept,
    Reject,
}

type Rating = [usize; 4];

fn parse_input(input: &str) -> (FxHashMap<String, Vec<Reference>>, Vec<Rating>) {
    let (rules, ratings) = input.split_once("\n\n").unwrap();

    let rules: FxHashMap<String, Vec<Reference>> = rules
        .lines()
        .map(|line| {
            let (name, data) = line.split_once('{').unwrap();
            let references = data
                .split(',')
                .map(|reference| {
                    let reference = {
                        if reference.ends_with('}') {
                            reference[0..reference.len() - 1].to_string()
                        } else {
                            reference.to_string()
                        }
                    };

                    if reference == "A" {
                        return Reference::Accept;
                    }

                    if reference == "R" {
                        return Reference::Reject;
                    }

                    if !reference.contains(':') {
                        return Reference::Other(reference);
                    }

                    let (value_and_condition, name) = reference.split_once(':').unwrap();
                    let value = match value_and_condition.chars().next().unwrap() {
                        'x' => Value::X,
                        'm' => Value::M,
                        'a' => Value::A,
                        's' => Value::S,
                        _ => panic!("{value_and_condition}"),
                    };

                    let condition = match value_and_condition.chars().nth(1).unwrap() {
                        '<' => Condition::Lower,
                        '>' => Condition::Higher,
                        _ => panic!(),
                    };

                    Reference::Conditional(
                        value,
                        condition,
                        value_and_condition[2..value_and_condition.len()]
                            .parse()
                            .unwrap(),
                        name.to_string(),
                    )
                })
                .collect();

            (name.to_string(), references)
        })
        .chain(std::iter::once(("A".to_string(), vec![Reference::Accept])))
        .chain(std::iter::once(("R".to_string(), vec![Reference::Reject])))
        .collect();

    let ratings: Vec<Rating> = ratings
        .lines()
        .map(|line| {
            let values: Vec<usize> = line[1..line.len() - 1]
                .split(',')
                .map(|value| value.split_once('=').unwrap().1)
                .map(|value| value.parse().unwrap())
                .collect();

            [values[0], values[1], values[2], values[3]]
        })
        .collect();

    (rules, ratings)
}

fn solve_for_rule(rating: Rating, rules: &FxHashMap<String, Vec<Reference>>, name: &str) -> bool {
    for rule in rules.get(name).unwrap() {
        match rule {
            Reference::Other(name) => return solve_for_rule(rating, rules, name),
            Reference::Accept => return true,
            Reference::Reject => return false,
            Reference::Conditional(value, condition, threshold, target) => {
                let value = match value {
                    Value::X => rating[0],
                    Value::M => rating[1],
                    Value::A => rating[2],
                    Value::S => rating[3],
                };

                let condition_met = match condition {
                    Condition::Higher => value > *threshold,
                    Condition::Lower => value < *threshold,
                };

                if condition_met {
                    return solve_for_rule(rating, rules, target);
                }

                continue;
            }
        };
    }

    panic!();
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, ratings) = parse_input(input);

    Some(
        ratings
            .into_iter()
            .filter(|rating| solve_for_rule(*rating, &rules, "in"))
            .map(|rating| rating.into_iter().sum::<usize>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, _) = parse_input(input);

    let mut queue: VecDeque<([(usize, usize); 4], String)> = VecDeque::new();
    let mut feasible_ranges: Vec<[(usize, usize); 4]> = Vec::new();
    queue.push_front(([(1, 4000); 4], "in".to_string()));

    while let Some((values, name)) = queue.pop_front() {
        let mut remaining_values = Some(values);
        let rules = rules.get(&name).unwrap();

        for reference in rules {
            let Some(values) = remaining_values else {
                break;
            };

            match reference {
                Reference::Other(name) => queue.push_front((values, name.to_string())),
                Reference::Accept => feasible_ranges.push(values),
                Reference::Reject => continue,
                Reference::Conditional(value, condition, threshold, target) => {
                    let index = match value {
                        Value::X => 0,
                        Value::M => 1,
                        Value::A => 2,
                        Value::S => 3,
                    };

                    let (current_min, current_max) = values[index];

                    let mut accepted_range = None;
                    let mut remaining_range = None;

                    match condition {
                        Condition::Higher => {
                            if current_max > *threshold {
                                accepted_range =
                                    Some((max(*threshold + 1, current_min), current_max));
                            }

                            if current_min <= *threshold {
                                remaining_range = Some((current_min, min(*threshold, current_max)));
                            }
                        }
                        Condition::Lower => {
                            if current_min < *threshold {
                                accepted_range =
                                    Some((current_min, min(*threshold - 1, current_max)));
                            }

                            if current_max >= *threshold {
                                remaining_range = Some((max(*threshold, current_min), current_max));
                            }
                        }
                    };

                    if let Some(accepted_range) = accepted_range {
                        let mut temp = values;
                        temp[index] = accepted_range;
                        queue.push_front((temp, target.clone()));
                    }

                    if let Some(remaining_range) = remaining_range {
                        let mut temp = values;
                        temp[index] = remaining_range;

                        remaining_values = Some(temp);
                    } else {
                        remaining_values = None;
                    }
                }
            };
        }
    }

    Some(
        feasible_ranges
            .iter()
            .map(|range| {
                range
                    .iter()
                    .map(|(min, max)| max - min + 1)
                    .product::<usize>()
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
