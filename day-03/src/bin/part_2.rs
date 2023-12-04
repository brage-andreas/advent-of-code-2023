/*
    +---------------------+
    | Advent of Code 2023 |
    +---------------------+
    >> Day 03, part 2

    The missing part wasn't the only issue - one of the gears in the engine is
    wrong. A gear is any * symbol that is adjacent to exactly two part numbers.
    Its gear ratio is the result of multiplying those two numbers together.

    This time, you need to find the gear ratio of every gear and add them all up
    so that the engineer can figure out which gear needs to be replaced.

    Consider the same engine schematic again:
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

    In this schematic, there are two gears. The first is in the top left; it has
    part numbers 467 and 35, so its gear ratio is 16345. The second gear is in
    the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a
    gear because it is only adjacent to one part number.) Adding up all of the
    gear ratios produces 467835.
*/

use regex::{Match, Regex};

use common::read_input;

fn matches_overlap(a: &Match, b: &Match) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

fn find_adjacent_numbers(
    lines: &[&str],
    line_index: usize,
    symbol_match: &Match,
    number_regex: &Regex,
) -> Vec<u64> {
    let mut adjacent_numbers = Vec::new();

    for i in line_index.saturating_sub(1)..=usize::min(line_index + 1, lines.len() - 1) {
        if let Some(line) = lines.get(i) {
            for number_match in number_regex.find_iter(line) {
                if matches_overlap(symbol_match, &number_match) {
                    if let Ok(number) = number_match.as_str().parse::<u64>() {
                        adjacent_numbers.push(number);
                    }
                }
            }
        }
    }

    adjacent_numbers
}

fn part_2(number_regex: &Regex, symbol_regex: &Regex) -> u64 {
    let file = read_input(3);
    let lines: Vec<&str> = file.split('\n').collect();

    let mut sum = 0;

    for (line_index, line_content) in lines.iter().enumerate() {
        for symbol_match in symbol_regex.find_iter(line_content) {
            let adjacent_numbers =
                find_adjacent_numbers(&lines, line_index, &symbol_match, number_regex);

            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
    }

    sum
}

fn main() {
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"\*").unwrap();

    println!("{}", part_2(&number_regex, &symbol_regex));
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::part_2;

    #[test]
    fn test_part_2() {
        let number_regex = Regex::new(r"\d+").unwrap();
        let symbol_regex = Regex::new(r"\*").unwrap();

        assert_eq!(part_2(&number_regex, &symbol_regex), 91031374);
    }
}
