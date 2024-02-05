fn validate_pin(pin: &str) -> bool {
    if pin.len() != 4 && pin.len() != 6 {
        return false;
    }
    if pin.contains("-") {
        return false;
    }
    let _test = pin.chars().all(char::is_numeric);
    return _test;
}

#[cfg(test)]
mod tests {
    use super::validate_pin;

    #[test]
    fn valid_pins() {
        assert_eq!(validate_pin("1234"), true, "Valid 4-digit PIN");
        assert_eq!(validate_pin("567890"), true, "Valid 6-digit PIN");
    }

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false, "Test case 1");
        assert_eq!(validate_pin("12"), false, "Test case 2");
        assert_eq!(validate_pin("123"), false, "Test case 3");
        assert_eq!(validate_pin("12345"), false, "Test case 4");
        assert_eq!(validate_pin("1234567"), false, "Test case 5");
        assert_eq!(validate_pin("-1234"), false, "Test case 6");
        assert_eq!(validate_pin("1.234"), false, "Test case 7");
        assert_eq!(validate_pin("-1.234"), false, "Test case 8");
        assert_eq!(validate_pin("00000000"), false, "Test case 9");
        assert_eq!(validate_pin("aa00"), false, "Test case 10");
    }

    #[test]
    fn valid_with_whitespace() {
        assert_eq!(validate_pin("12 34"), false, "Contains whitespace");
        assert_eq!(validate_pin("123 45"), false, "Contains whitespace");
    }

    #[test]
    fn valid_with_special_characters() {
        assert_eq!(validate_pin("!@#$"), false, "Contains special characters");
        assert_eq!(validate_pin("12@34"), false, "Contains special characters");
    }

    #[test]
    fn valid_with_hyphen() {
        assert_eq!(validate_pin("123-45"), false, "Contains hyphen");
        assert_eq!(validate_pin("123456-"), false, "Contains hyphen");
    }

    #[test]
    fn empty_string() {
        assert_eq!(validate_pin(""), false, "Empty string");
    }
}

fn main() {
    println!("{}", validate_pin("132-2"));
}
