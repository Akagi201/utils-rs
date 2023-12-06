use regex::Regex;

pub fn remove_0x_prefix(input: &str) -> &str {
    if input.starts_with("0x") || input.starts_with("0X") {
        &input[2..]
    } else {
        input
    }
}

pub fn is_hex_str(input: &str) -> bool {
    let re = Regex::new(r"^(0x|0X)?[a-fA-F0-9]+$").unwrap();
    re.is_match(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_0x_prefix() {
        assert_eq!(remove_0x_prefix("0x123"), "123");
        assert_eq!(remove_0x_prefix("0X123"), "123");
        assert_eq!(remove_0x_prefix("123"), "123");
    }

    #[test]
    fn test_is_hex_str() {
        assert_eq!(is_hex_str("0x123"), true);
        assert_eq!(is_hex_str("0X123"), true);
        assert_eq!(is_hex_str("123"), true);
        assert_eq!(is_hex_str("0x123g"), false);
        assert_eq!(is_hex_str("0X123g"), false);
        assert_eq!(is_hex_str("123g"), false);
    }
}
