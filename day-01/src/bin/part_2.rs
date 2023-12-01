/*
    +---------------------+
    | Advent of Code 2023 |
    +---------------------+
    >> Day 01, part 2

    Your calculation isn't quite right. It looks like some of the digits are
    actually spelled out with letters: one, two, three, four, five, six, seven,
    eight, and nine also count as valid "digits".

    Equipped with this new information, you now need to find the real first and
    last digit on each line. For example:
    > two1nine
    > eightwothree
    > abcone2threexyz
    > xtwone3four
    > 4nineeightseven2
    > zoneight234
    > 7pqrstsixteen

    In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
    Adding these together produces 281.

    What is the sum of all of the calibration values?
*/

mod part_1;

use regex::Regex;
use std::fs;

const PART_2_INPUT_FILE_PATH: &str = "src/bin/part-1-2-input.txt";

fn get_first_and_last_number(string: &str) -> (&str, &str) {
    const SPELLED_OUT_DIGITS_REGEX: &str =
        r"(?:one)|(?:two)|(?:three)|(?:four)|(?:five)|(?:six)|(?:seven)|(?:eight)|(?:nine)";

    let first_and_last_number_regex =
        Regex::new(r"^\D*(?<first_number>\d)(?:.*(?<second_number>\d)\D*$)?").unwrap();

    let captures = first_and_last_number_regex.captures(string).unwrap();

    let first_number = captures.get(1).map_or("0", |capture| capture.as_str());
    let second_number = captures
        .get(2)
        .map_or(first_number, |capture| capture.as_str());

    (first_number, second_number)
}

fn combine_first_and_last_number(first_number: &str, second_number: &str) -> i32 {
    match format!("{}{}", first_number, second_number).parse::<i32>() {
        Ok(result) => result,
        Err(_) => panic!("Regex `REGEX` is faulty, as it captured a non-number"),
    }
}

fn part_2() -> i32 {
    let file = match fs::read_to_string(&PART_2_INPUT_FILE_PATH) {
        Ok(result) => result,
        Err(error) => panic!(
            "Could not read from local file `PART_1_INPUT_FILE_PATH`={}\n  Message: \"{}\"",
            &PART_2_INPUT_FILE_PATH, error
        ),
    };

    let mut result = 0;

    for line in file.split("\n") {
        let (first_number, last_number) = get_first_and_last_number(line);

        result += combine_first_and_last_number(&first_number, &last_number);
    }

    result
}

fn main() {
    println!("{}", &part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_2(), 54990)
    }
}
