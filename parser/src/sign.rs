use crate::{tokens::Sign, ParseResult};

pub(crate) fn parse(input: &str) -> ParseResult<'_, Sign> {
    let sign = if input.starts_with('-') {
        Sign::Negative
    } else if input.starts_with('+') {
        Sign::Positive
    } else {
        return Err(input);
    };

    Ok((sign, &input[1..]))
}

#[cfg(test)]
mod tests {
    use crate::tokens::Sign;

    #[test]
    fn test_parse() {
        assert_eq!(super::parse("+"), Ok((Sign::Positive, "")));
        assert_eq!(super::parse("-"), Ok((Sign::Negative, "")));
        assert_eq!(super::parse("+123"), Ok((Sign::Positive, "123")));
        assert_eq!(super::parse("123"), Err("123"));
        assert_eq!(super::parse(""), Err(""));
    }
}
