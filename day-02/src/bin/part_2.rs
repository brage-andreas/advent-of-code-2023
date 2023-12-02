/*
    +---------------------+
    | Advent of Code 2023 |
    +---------------------+
    >> Day 02, part 2

    As you continue your walk, the Elf poses a second question: in each game
    you played, what is the fewest number of cubes of each color that could
    have been in the bag to make the game possible?

    Again consider the example games from earlier:
    > Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    > Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    > Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    > Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    > Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    In game 1, the game could have been played with as few as 4 red, 2 green,
    and 6 blue cubes. If any color had even one fewer cube, the game would have
    been impossible.
    Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue
    cubes.
    Game 3 must have been played with at least 20 red, 13 green, and 6 blue
    cubes.
    Game 4 required at least 14 red, 3 green, and 15 blue cubes.
    Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
    The power of a set of cubes is equal to the numbers of red, green, and blue
    cubes multiplied together. The power of the minimum set of cubes in game 1
    is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up
    these five powers produces the sum 2286.

    For each game, find the minimum set of cubes that must have been present.
    What is the sum of the power of these sets?
*/

use common::read_from_file;
use day_02_lib::parse_game;

const PART_2_INPUT_FILE_PATH: &str = "day-02/src/part-1-2-input.txt";

fn part_2() -> i32 {
    let file = read_from_file(PART_2_INPUT_FILE_PATH);

    let mut sum: i32 = 0;

    for line in file.split("\n") {
        let game = parse_game(&line);

        let mut min_green_cubes = 0;
        let mut min_blue_cubes = 0;
        let mut min_red_cubes = 0;

        for round in game.rounds.iter() {
            if round.green_cubes > min_green_cubes {
                min_green_cubes = round.green_cubes;
            }

            if round.blue_cubes > min_blue_cubes {
                min_blue_cubes = round.blue_cubes;
            }

            if round.red_cubes > min_red_cubes {
                min_red_cubes = round.red_cubes;
            }
        }

        sum += min_green_cubes * min_blue_cubes * min_red_cubes;
    }

    sum
}

fn main() {
    println!("{}", part_2());
}

#[cfg(test)]
mod tests {
    use crate::part_2;

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(), 69629)
    }
}
