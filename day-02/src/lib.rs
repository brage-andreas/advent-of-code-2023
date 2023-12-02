use regex::Regex;
use std::fs;

pub fn read_from_file(path_from_crate_root: &str) -> String {
    match fs::read_to_string(&path_from_crate_root) {
        Ok(result) => result,
        Err(error) => panic!(
            "Could not read from local file `path_from_crate_root`={}\n  Message: \"{:?}\"",
            &path_from_crate_root, &error
        ),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Round {
    pub green_cubes: i32,
    pub blue_cubes: i32,
    pub red_cubes: i32,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: i32,
    pub rounds: Vec<Round>,
}

fn parse_game_id(string: &str) -> i32 {
    let game_id_regex = Regex::new(r"Game\s(?<id>\d+)").unwrap();
    let captures = game_id_regex.captures(string).unwrap();
    let capture = captures.name("id").unwrap().as_str();

    capture.parse::<i32>().unwrap()
}

pub fn parse_game(string: &str) -> Game {
    let parts: Vec<&str> = string.split(":").collect();
    let [game_vector, game_data] = match parts.as_slice() {
        [gv, gd] => [gv.trim(), gd.trim()],
        _ => panic!("Expected exactly two parts in the string"),
    };

    let mut game = Game {
        id: parse_game_id(game_vector),
        rounds: Vec::new(),
    };

    for round in game_data.replace(",", "").split(";") {
        let round_vector: Vec<&str> = round.trim().split(" ").collect();

        if round_vector.len() % 2 != 0 {
            panic!("Round is not of even length: {round}");
        }

        let mut current_round = Round {
            green_cubes: 0,
            blue_cubes: 0,
            red_cubes: 0,
        };

        for half_i in 0..round_vector.len() / 2 {
            let first_element = round_vector[half_i * 2];
            let second_element = round_vector[half_i * 2 + 1];

            let amount = first_element
                .parse::<i32>()
                .expect("Could not parse amount as i32");

            match second_element.get(0..1) {
                Some("g") => {
                    current_round.green_cubes += amount;
                }
                Some("b") => {
                    current_round.blue_cubes += amount;
                }
                Some("r") => {
                    current_round.red_cubes += amount;
                }
                _ => panic!("Second element does not start with 'g', 'b', or 'r'"),
            };
        }

        game.rounds.push(current_round);
    }

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_id_test() {
        assert_eq!(parse_game_id("Game 1"), 1);
        assert_eq!(parse_game_id("Game 29"), 29);
    }

    #[test]
    fn parse_game_test_1() {
        let game_1_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_1 = Game {
            id: 1,
            rounds: vec![
                Round {
                    green_cubes: 0,
                    blue_cubes: 3,
                    red_cubes: 4,
                },
                Round {
                    green_cubes: 2,
                    blue_cubes: 6,
                    red_cubes: 1,
                },
                Round {
                    green_cubes: 2,
                    blue_cubes: 0,
                    red_cubes: 0,
                },
            ],
        };

        assert_eq!(parse_game(game_1_string), game_1);
    }

    #[test]
    fn parse_game_test_2() {
        let game_2_string = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game_2 = Game {
            id: 2,
            rounds: vec![
                Round {
                    green_cubes: 2,
                    blue_cubes: 1,
                    red_cubes: 0,
                },
                Round {
                    green_cubes: 3,
                    blue_cubes: 4,
                    red_cubes: 1,
                },
                Round {
                    green_cubes: 1,
                    blue_cubes: 1,
                    red_cubes: 0,
                },
            ],
        };

        assert_eq!(parse_game(game_2_string), game_2);
    }
}
