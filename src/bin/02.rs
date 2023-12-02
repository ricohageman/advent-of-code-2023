use std::cmp::max;
advent_of_code::solution!(2);

enum Colour {
    Blue,
    Green,
    Red,
}

enum Observation {
    Cubes { colour: Colour, amount: usize },
    EndOfHand,
}

fn parse_observations_in_game(game: &str) -> impl Iterator<Item = Observation> + '_ {
    game.split(": ")
        .nth(1)
        .unwrap()
        .split("; ")
        .flat_map(|sample| {
            sample
                .split(", ")
                .map(|observation| {
                    let (amount, colour) = observation.split_once(' ').unwrap();

                    let colour = match colour {
                        "green" => Colour::Green,
                        "blue" => Colour::Blue,
                        "red" => Colour::Red,
                        _ => panic!(),
                    };

                    Observation::Cubes {
                        colour,
                        amount: amount.parse().unwrap(),
                    }
                })
                .chain(std::iter::once(Observation::EndOfHand))
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .enumerate()
            .filter(|(_, game)| {
                let mut blue = 0;
                let mut green = 0;
                let mut red = 0;

                for observation in parse_observations_in_game(game) {
                    match observation {
                        Observation::EndOfHand => {
                            if blue > 14 || green > 13 || red > 12 {
                                return false;
                            }

                            blue = 0;
                            green = 0;
                            red = 0;
                        }
                        Observation::Cubes { amount, colour } => match colour {
                            Colour::Blue => blue += amount,
                            Colour::Green => green += amount,
                            Colour::Red => red += amount,
                        },
                    }
                }

                true
            })
            .map(|(index, _)| index + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|game| {
                let mut blue = 0;
                let mut green = 0;
                let mut red = 0;

                for observation in parse_observations_in_game(game) {
                    match observation {
                        Observation::EndOfHand => {}
                        Observation::Cubes { amount, colour } => match colour {
                            Colour::Blue => blue = max(blue, amount),
                            Colour::Green => green = max(green, amount),
                            Colour::Red => red = max(red, amount),
                        },
                    }
                }

                blue * green * red
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
