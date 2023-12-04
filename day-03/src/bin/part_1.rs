/*
    +---------------------+
    | Advent of Code 2023 |
    +---------------------+
    >> Day 02, part 1

    The engine schematic (your puzzle input) consists of a visual
    representation of the engine. There are lots of numbers and symbols you
    don't really understand, but apparently any number adjacent to a symbol,
    even diagonally, is a "part number" and should be included in your sum.
    (Periods (.) do not count as a symbol.)

    Here is an example engine schematic:
    > 467..114..
    > ...*......
    > ..35..633.
    > ......#...
    > 617*......
    > .....+.58.
    > ..592.....
    > ......755.
    > ...$.*....
    > .664.598..

    In this schematic, two numbers are not part numbers because they are not
    adjacent to a symbol: 114 (top right) and 58 (middle right). Every other
    number is adjacent to a symbol and so is a part number; their sum is 4361.

    Of course, the actual engine schematic is much larger. What is the sum of
    all of the part numbers in the engine schematic?
*/

use regex::Regex;

use common::read_input;

// logic could be improved by
//   - merging overlapping ranges
//   - bucketing searches for each row; so instead of searching each row up to
//     three times, you search them once
//   - ignoring the hit itself

fn is_number_adjacent_to_symbol(
    lines: &[&str],
    line_index: usize,
    number_match: &regex::Match,
    symbol_regex: &Regex,
) -> bool {
    let range = line_index.saturating_sub(1)..usize::min(line_index + 2, lines.len());

    for test_line_index in range {
        if let Some(test_line) = lines.get(test_line_index) {
            let line_chars: Vec<char> = test_line.chars().collect();
            let line_length = line_chars.len();

            let index_start = usize::min(number_match.start().saturating_sub(1), line_length);
            let index_end = usize::min(number_match.end() + 1, line_length);

            let string_slice: String = line_chars[index_start..index_end].iter().collect();
            
            if symbol_regex.is_match(&string_slice) {
                return true;
            }
        }
    }

    false
}

fn part_1(number_regex: &Regex, symbol_regex: &Regex) -> u32 {
    let file = read_input(3);
    let lines: Vec<&str> = file.split('\n').collect();

    let mut sum = 0;

    for (line_index, line_content) in lines.iter().enumerate() {
        for number_match in number_regex.find_iter(line_content) {
            if is_number_adjacent_to_symbol(&lines, line_index, &number_match, symbol_regex) {
                sum += number_match.as_str().parse::<u32>().unwrap_or(0);
            }
        }
    }

    sum
}

fn main() {
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();

    println!("{}", part_1(&number_regex, &symbol_regex));
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{is_number_adjacent_to_symbol, part_1};

    #[test]
    fn is_number_adjacent_to_symbol_test() {
        let number_regex = Regex::new(r"\d+").unwrap();
        let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();

        let lines = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let result_a = is_number_adjacent_to_symbol(
            &lines,
            0,
            &number_regex.find("467").unwrap(),
            &symbol_regex,
        );

        let result_b = is_number_adjacent_to_symbol(
            &lines,
            0,
            &number_regex.find(".....114").unwrap(),
            &symbol_regex,
        );

        assert_eq!(result_a, true);
        assert_eq!(result_b, false);
    }

    #[test]
    fn part_1_test() {
        let number_regex = Regex::new(r"\d+").unwrap();
        let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();

        assert_eq!(part_1(&number_regex, &symbol_regex), 546563); // Update with correct expected value
    }
}
