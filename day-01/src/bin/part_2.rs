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

use regex::Regex;

use common::read_input;
use day_01_lib::combine_first_and_last_number;

fn match_capture(string: &str) -> &str {
    match string {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        rest => rest,
    }
}

fn get_first_and_last_number(string: &str) -> (&str, &str) {
    let first_and_last_number_regex = Regex::new(r"^.*?(?<first_number>\d|one|two|three|four|five|six|seven|eight|nine)(?:.*(?<second_number>\d|one|two|three|four|five|six|seven|eight|nine))?.*?$").unwrap();

    let captures = first_and_last_number_regex.captures(string).unwrap();

    let first_number = captures
        .name("first_number")
        .map_or("0", |capture| match_capture(capture.as_str()));

    let second_number = captures
        .name("second_number")
        .map_or(first_number, |capture| match_capture(capture.as_str()));

    (first_number, second_number)
}

fn part_2() -> i32 {
    let file = read_input(1);

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
    fn match_capture_test() {
        assert_eq!(match_capture("one"), "1");
        assert_eq!(match_capture("two"), "2");
        assert_eq!(match_capture("three"), "3");
        assert_eq!(match_capture("four"), "4");
        assert_eq!(match_capture("five"), "5");
        assert_eq!(match_capture("six"), "6");
        assert_eq!(match_capture("seven"), "7");
        assert_eq!(match_capture("eight"), "8");
        assert_eq!(match_capture("nine"), "9");
        assert_eq!(match_capture("1"), "1");
        assert_eq!(match_capture("4"), "4");
        assert_eq!(match_capture("5"), "5");
        assert_eq!(match_capture("9"), "9");
    }

    #[test]
    fn get_first_and_last_number_test() {
        assert_eq!(get_first_and_last_number("two1nine"), ("2", "9"));
        assert_eq!(get_first_and_last_number("eightwothree"), ("8", "3"));
        assert_eq!(get_first_and_last_number("abcone2threexyz"), ("1", "3"));
        assert_eq!(get_first_and_last_number("7pqrstsixteen"), ("7", "6"));
        assert_eq!(get_first_and_last_number("6zfxp"), ("6", "6"));
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_2(), 54473)
    }
}
