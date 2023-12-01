/*
    +---------------------+
    | Advent of Code 2023 |
    +---------------------+
    >> Day 01, part 1

    The newly-improved calibration document consists of lines of text; each
    line originally contained a specific calibration value that the Elves now
    need to recover. On each line, the calibration value can be found by
    combining the first digit and the last digit (in that order) to form a
    single two-digit number.

    For example:
    > 1abc2
    > pqr3stu8vwx
    > a1b2c3d4e5f
    > treb7uchet

    In this example, the calibration values of these four lines are 12, 38,
    15, and 77. Adding these together produces 142.

    Consider your entire calibration document. What is the sum of all of the
    calibration values?
*/

use regex::Regex;
use std::fs;

const PART_1_INPUT_FILE_PATH: &str = "src/bin/part-1-input.txt";

fn combine_first_and_last_number(string: &str) -> i32 {
    let first_and_last_number_regex =
        Regex::new(r"^\D*(?<first_number>\d)(?:.*(?<second_number>\d)\D*$)?").unwrap();

    let captures = first_and_last_number_regex.captures(string).unwrap();

    let first_number = captures.get(1).map_or("0", |capture| capture.as_str());
    let second_number = captures
        .get(2)
        .map_or(first_number, |capture| capture.as_str());

    match format!("{}{}", first_number, second_number).parse::<i32>() {
        Ok(result) => result,
        Err(_) => panic!("Regex `REGEX` is faulty, as it captured a non-number"),
    }
}

fn part_1() -> i32 {
    let file = match fs::read_to_string(&PART_1_INPUT_FILE_PATH) {
        Ok(result) => result,
        Err(error) => panic!(
            "Could not read from local file `PART_1_INPUT_FILE_PATH`={}\n  Message: \"{}\"",
            &PART_1_INPUT_FILE_PATH, error
        ),
    };

    let mut result = 0;

    for line in file.split("\n") {
        result += combine_first_and_last_number(&line);
    }

    result
}

fn main() {
    println!("{}", &part_1());
}

#[cfg(test)]
mod tests {
    use crate::{combine_first_and_last_number, part_1};

    #[test]
    fn combine_first_and_last_number_test() {
        assert_eq!(combine_first_and_last_number("1abc2"), 12);
        assert_eq!(combine_first_and_last_number("pqr3stu8vwx"), 38);
        assert_eq!(combine_first_and_last_number("a1b2c3d4e5f"), 15);
        assert_eq!(combine_first_and_last_number("treb7uchet"), 77);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 54990)
    }
}
