use crate::ParseResult;

pub(super) fn parse(input: &str) -> ParseResult<'_, u32> {
    // TODO: see if this has any overhead
    if input.is_empty() {
        return Err(input);
    }
    let mut chars = input.chars().peekable();
    let mut digits_str_len = 0_usize;

    // Match first char or return input.
    match chars.peek() {
        Some(next) if next.is_ascii_digit() => {
            digits_str_len += 1;
            chars.next();
        }
        _ => return Err(input),
    }

    while let Some(next) = chars.peek() {
        if next.is_ascii_digit() {
            digits_str_len += 1;
            chars.next();
        } else {
            break;
        }
    }

    let digits_string: &str = &input[..digits_str_len];
    let number = digits_string.parse::<u32>().map_err(|_| input)?;

    Ok((number, &input[digits_str_len..]))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_single_digit() {
        assert_eq!(super::parse("0"), Ok((0, "")));
    }

    #[test]
    fn test_parse_multi_digit_leading_zero() {
        assert_eq!(super::parse("0123"), Ok((123, "")));
    }

    #[test]
    fn test_parse_multi_digit_leading_zero_with_tail() {
        assert_eq!(super::parse("0123four"), Ok((123, "four")));
    }

    #[test]
    fn test_parse_leading_non_digit() {
        assert_eq!(super::parse("a123"), Err("a123"));
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(super::parse(""), Err(""));
    }
}
