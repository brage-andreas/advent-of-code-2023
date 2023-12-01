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

const PART_1_INPUT_FILE_PATH: &str = "src/bin/part-1-2-input.txt";

fn get_first_and_last_number(string: &str) -> (&str, &str) {
    let first_and_last_number_regex =
        Regex::new(r"^\D*(?<first_number>\d)(?:.*(?<second_number>\d)\D*$)?").unwrap();

    let captures = first_and_last_number_regex.captures(string).unwrap();

    let first_number = captures.get(1).map_or("0", |capture| capture.as_str());
    let second_number = captures
        .get(2)
        .map_or(first_number, |capture| capture.as_str());

    (first_number, second_number)
}

pub fn combine_first_and_last_number(first_number: &str, second_number: &str) -> i32 {
    match format!("{}{}", first_number, second_number).parse::<i32>() {
        Ok(result) => result,
        Err(error) => panic!("Could not convert result to number: {:?}", error),
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
        let (first_number, last_number) = get_first_and_last_number(line);

        result += combine_first_and_last_number(&first_number, &last_number);
    }

    result
}

fn main() {
    println!("{}", part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_and_last_number_test() {
        assert_eq!(get_first_and_last_number("1abc2"), ("1", "2"));
        assert_eq!(get_first_and_last_number("pqr3stu8vwx"), ("3", "8"));
        assert_eq!(get_first_and_last_number("a1b2c3d4e5f"), ("1", "5"));
        assert_eq!(get_first_and_last_number("treb7uchet"), ("7", "7"));
    }

    #[test]
    fn combine_first_and_last_number_test() {
        assert_eq!(combine_first_and_last_number("1", "2"), 12);
        assert_eq!(combine_first_and_last_number("3", "8"), 38);
        assert_eq!(combine_first_and_last_number("1", "5"), 15);
        assert_eq!(combine_first_and_last_number("7", "7"), 77);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 54990)
    }
}
