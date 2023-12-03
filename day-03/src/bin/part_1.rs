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

// could be improved by merging overlapping ranges
// could be improved by bucketing searches for each row; so instead of
// searching each row up to three times, you search them once
// could also be improved by ignoring the hit itself
fn part_1(number_regex: &Regex, symbol_regex: &Regex) -> u32 {
    let file = read_input(3);
    let lines: Vec<&str> = file.split("\n").collect();

    let mut sum = 0;

    for (line, line_content) in lines.iter().enumerate() {
        for line_match in number_regex.find_iter(line_content) {
            let lines_to_test_range_start = line.saturating_sub(1);
            let lines_to_test_range_end = usize::min(line + 1, lines.len() - 1);
            let lines_to_test_range = lines_to_test_range_start..=lines_to_test_range_end;

            let mut hit = false;

            for line_to_test in lines_to_test_range.filter_map(|i| lines.get(i)) {
                let line_length_minus_one = line_to_test
                    .chars()
                    .collect::<Vec<char>>()
                    .len()
                    .saturating_sub(1);

                if line_length_minus_one == 0 {
                    continue;
                }

                let index_start = usize::min(line_match.start().saturating_sub(1), line_length_minus_one);
                let index_end = usize::min(line_match.end() + 1, line_length_minus_one);

                let string_slice = &line_to_test[index_start..index_end];

                if symbol_regex.is_match(string_slice) {
                    hit = true;
                    break;
                }
            }

            if hit {
                sum += line_match.as_str().parse::<u32>().unwrap_or(0);
            }
        }
    }

    sum
}

fn main() {
    let number_regex = Regex::new(r"(?<number>\d+)").unwrap();
    let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();

    println!("{}", part_1(&number_regex, &symbol_regex));
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::part_1;

    #[test]
    fn part_1_test() {
        let number_regex = Regex::new(r"(?<number>\d+)").unwrap();
        let symbol_regex = Regex::new(r"[^0-9.\s]").unwrap();

        assert_eq!(part_1(&number_regex, &symbol_regex), 546563)
    }
}
