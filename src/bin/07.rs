use itertools::Itertools;
advent_of_code::solution!(7);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

type Hand = [usize; 5];

impl From<&Hand> for HandStrength {
    fn from(hand: &Hand) -> Self {
        let occurrence_of_cards = hand.iter().counts();

        match occurrence_of_cards.len() {
            1 => HandStrength::FiveOfAKind,
            2 => match occurrence_of_cards.values().next().unwrap() {
                1 | 4 => HandStrength::FourOfAKind,
                2 | 3 => HandStrength::FullHouse,
                _ => panic!(),
            },
            3 => match occurrence_of_cards.values().max().unwrap() {
                3 => HandStrength::ThreeOfAKind,
                2 => HandStrength::TwoPair,
                _ => panic!(),
            },
            4 => HandStrength::OnePair,
            5 => HandStrength::HighCard,
            _ => panic!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut number_of_lines = 0;

    Some(
        input
            .lines()
            .filter_map(|line| line.split_once(' '))
            .map(|(hand, bid)| (hand, bid.parse::<usize>().unwrap()))
            .map(|(hand, bid)| {
                let hand: Hand = hand
                    .chars()
                    .map(|char| match char {
                        'A' => 1,
                        'K' => 2,
                        'Q' => 3,
                        'J' => 4,
                        'T' => 5,
                        '9' => 6,
                        '8' => 7,
                        '7' => 8,
                        '6' => 9,
                        '5' => 10,
                        '4' => 11,
                        '3' => 12,
                        '2' => 13,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let strength = HandStrength::from(&hand);

                number_of_lines += 1;

                (hand, strength, bid)
            })
            .sorted_by_key(|(hand, strength, _)| (*strength, *hand))
            .enumerate()
            .map(|(index, (_, _, bid))| (number_of_lines - index) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut number_of_lines = 0;

    Some(
        input
            .lines()
            .filter_map(|line| line.split_once(' '))
            .map(|(hand, bid)| (hand, bid.parse::<usize>().unwrap()))
            .map(|(hand, bid)| {
                number_of_lines += 1;

                let hand: Hand = hand
                    .chars()
                    .map(|char| match char {
                        'A' => 1,
                        'K' => 2,
                        'Q' => 3,
                        'T' => 4,
                        '9' => 5,
                        '8' => 6,
                        '7' => 7,
                        '6' => 8,
                        '5' => 9,
                        '4' => 10,
                        '3' => 11,
                        '2' => 12,
                        'J' => 13,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let number_of_jokers: usize = hand.iter().filter(|value| **value == 13).count();

                let strength = match HandStrength::from(&hand) {
                    HandStrength::FiveOfAKind => match number_of_jokers {
                        0 => HandStrength::FiveOfAKind,
                        5 => HandStrength::FiveOfAKind,
                        _ => panic!(),
                    },
                    HandStrength::FourOfAKind => match number_of_jokers {
                        0 => HandStrength::FourOfAKind,
                        1 => HandStrength::FiveOfAKind,
                        4 => HandStrength::FiveOfAKind,
                        _ => panic!(),
                    },
                    HandStrength::FullHouse => match number_of_jokers {
                        0 => HandStrength::FullHouse,
                        2 => HandStrength::FiveOfAKind,
                        3 => HandStrength::FiveOfAKind,
                        _ => panic!(),
                    },
                    HandStrength::ThreeOfAKind => match number_of_jokers {
                        0 => HandStrength::ThreeOfAKind,
                        1 => HandStrength::FourOfAKind,
                        2 => HandStrength::FiveOfAKind,
                        3 => HandStrength::FourOfAKind,
                        _ => panic!(),
                    },
                    HandStrength::TwoPair => match number_of_jokers {
                        0 => HandStrength::TwoPair,
                        1 => HandStrength::FullHouse,
                        2 => HandStrength::FourOfAKind,
                        _ => panic!(),
                    },
                    HandStrength::OnePair => match number_of_jokers {
                        0 => HandStrength::OnePair,
                        1 => HandStrength::ThreeOfAKind,
                        2 => HandStrength::ThreeOfAKind,
                        _ => panic!(),
                    },
                    HandStrength::HighCard => match number_of_jokers {
                        0 => HandStrength::HighCard,
                        1 => HandStrength::OnePair,
                        _ => panic!(),
                    },
                };

                (hand, strength, bid)
            })
            .sorted_by_key(|(hand, strength, _)| (*strength, *hand))
            .enumerate()
            .map(|(index, (_, _, bid))| (number_of_lines - index) * bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
