pub fn combine_first_and_last_number(first_number: &str, second_number: &str) -> i32 {
    match format!("{}{}", first_number, second_number).parse::<i32>() {
        Ok(result) => result,
        Err(error) => panic!("Could not convert result to number: {:?}", error),
    }
}

#[cfg(test)]
mod tests {
    use crate::combine_first_and_last_number;

    #[test]
    fn combine_first_and_last_number_test() {
        assert_eq!(combine_first_and_last_number("1", "2"), 12);
        assert_eq!(combine_first_and_last_number("3", "8"), 38);
        assert_eq!(combine_first_and_last_number("1", "5"), 15);
        assert_eq!(combine_first_and_last_number("7", "7"), 77);
    }

    #[test]
    #[should_panic(
        expected = "Could not convert result to number: ParseIntError { kind: InvalidDigit }"
    )]
    fn combine_first_and_last_number_panic_test() {
        assert_eq!(combine_first_and_last_number("a", "b"), 12);
        assert_eq!(combine_first_and_last_number("1", "b"), 12);
        assert_eq!(combine_first_and_last_number("h", "7"), 12);
    }
}
