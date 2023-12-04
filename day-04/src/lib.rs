use regex::Regex;

pub struct ScratchCard {
    pub id: u32,
    pub numbers: Vec<u32>,
    pub winning_numbers: Vec<u32>,
}

fn is_card_line(input: &str) -> bool {
    let game_regex = Regex::new(r"^Card\s*\d+:(?:\s+\d+)+\s+\|(?:\s+\d+)+$").unwrap();

    game_regex.is_match(input.trim())
}

fn convert_space_and_number_string_to_u32_vector(input: &str) -> Vec<u32> {
    let space_regex = Regex::new(r"\s+").unwrap();

    space_regex.split(input.trim()).map(|x| x.parse::<u32>().unwrap()).collect()
}

fn extract_card_id(game_prefix: &str) -> u32 {
    let card_id_regex = Regex::new(r"Card\s*(?<card_id>\d+)").unwrap();
    let split_card_prefix = card_id_regex.captures(game_prefix).unwrap().name("card_id").unwrap();

    split_card_prefix.as_str().parse::<u32>().unwrap()
}

pub fn parse_card_single_line(input: &str) -> ScratchCard {
    let (card_prefix, card_data) = input.split_once(":").unwrap();

    let card_id = extract_card_id(card_prefix);

    let (numbers, winning_numbers) = card_data.trim().split_once("|").unwrap();

    ScratchCard {
        id: card_id,
        numbers: convert_space_and_number_string_to_u32_vector(numbers),
        winning_numbers: convert_space_and_number_string_to_u32_vector(winning_numbers),
    }
}

pub fn parse_cards(input: &str) -> Vec<ScratchCard> {
    let mut cards: Vec<ScratchCard> = Vec::new();

    for line in input.lines() {
        if is_card_line(line) {
            cards.push(parse_card_single_line(line));
        }
    }

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_card_line_test() {
        let game_line_1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let game_line_2 = "Card    2: 13 32 20    16 61 | 61 30 68 82 17    32 24 19";
        let game_line_3 = "Card3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let not_game_line_1 = "Card 4: 41 92 73 84 69 | 76 51 58  5 92 73 84 69 | 59  58  5 54 83";
        let not_game_line_2 = "41 92 73 84 69 | 59 84 76 51 58  5 54 83  6 | 59  51 58  5 54 83";
        let not_game_line_3 = "Card 6: 41 92 73 84 69 | ";

        assert_eq!(is_card_line(game_line_1), true);
        assert_eq!(is_card_line(game_line_2), true);
        assert_eq!(is_card_line(game_line_3), true);
        assert_eq!(is_card_line(not_game_line_1), false);
        assert_eq!(is_card_line(not_game_line_2), false);
        assert_eq!(is_card_line(not_game_line_3), false);
    }

    #[test]
    fn convert_space_and_number_string_to_u32_vector_test() {
        let input_1_result = convert_space_and_number_string_to_u32_vector("41 48 83 86 17");
        let input_1_expected_result = vec![41, 48, 83, 86, 17];

        let input_2_result = convert_space_and_number_string_to_u32_vector("83 86  6 31 17  9 48");
        let input_2_expected_result = vec![83, 86, 6, 31, 17, 9, 48];

        let input_3_result = convert_space_and_number_string_to_u32_vector("1 21 53 59 44");
        let input_3_expected_result = vec![1, 21, 53, 59, 44];

        assert_eq!(input_1_result, input_1_expected_result);
        assert_eq!(input_2_result, input_2_expected_result);
        assert_eq!(input_3_result, input_3_expected_result);
    }

    #[test]
    fn extract_card_id_test() {
        let game_prefix_1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let game_prefix_2 = "Card    2: 13 32 20    16 61 | 61 30 68 82 17    32 24 19";
        let game_prefix_3 = "Card3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";

        assert_eq!(extract_card_id(game_prefix_1), 1);
        assert_eq!(extract_card_id(game_prefix_2), 2);
        assert_eq!(extract_card_id(game_prefix_3), 3);
    }

    #[test]
    fn parse_card_single_line_test() {
        let game_line_1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let game_line_2 = "Card    2: 13 32 20    16 61 | 61 30 68 82 17    32 24 19";
        let game_line_3 = "Card3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";

        let game_1 = parse_card_single_line(game_line_1);
        let game_2 = parse_card_single_line(game_line_2);
        let game_3 = parse_card_single_line(game_line_3);

        assert_eq!(game_1.id, 1);
        assert_eq!(game_1.numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(game_1.winning_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);

        assert_eq!(game_2.id, 2);
        assert_eq!(game_2.numbers, vec![13, 32, 20, 16, 61]);
        assert_eq!(game_2.winning_numbers, vec![61, 30, 68, 82, 17, 32, 24, 19]);

        assert_eq!(game_3.id, 3);
        assert_eq!(game_3.numbers, vec![1, 21, 53, 59, 44]);
        assert_eq!(game_3.winning_numbers, vec![69, 82, 63, 72, 16, 21, 14, 1]);
    }
}
