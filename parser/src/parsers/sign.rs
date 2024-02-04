use crate::{tokens::Sign, ParseResult};

pub(super) fn parse(input: &str) -> ParseResult<'_, Sign> {
    if let Some(stripped_from_neg) = input.strip_prefix('-') {
        return Ok((Sign::Negative, stripped_from_neg));
    }

    input
        .strip_prefix('+')
        .map_or(Err(input), |stripped_from_pos| {
            Ok((Sign::Positive, stripped_from_pos))
        })
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
