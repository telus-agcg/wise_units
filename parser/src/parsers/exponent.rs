use super::{digits, sign};
use crate::{tokens::Sign, ParseResult};

pub(super) fn parse(input: &str) -> ParseResult<'_, i8> {
    let (sign, sign_tail) =
        sign::parse(input).map_or_else(|_| (None, input), |(sign, tail)| (Some(sign), tail));

    digits::parse(sign_tail)
        .map(|(digits, digits_tail)| {
            // TODO: For UCUM units, an i8 is probably sufficient, but for others, we should
            // probably return an error here. Allowing the clippy for now.
            #[allow(clippy::cast_possible_truncation)]
            let exponent: i8 = match sign {
                Some(Sign::Positive) | None => digits as i8,
                Some(Sign::Negative) => -(digits as i8),
            };

            (exponent, digits_tail)
        })
        .map_err(|_| input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        assert_eq!(super::parse("0"), Ok((0, "")));
        assert_eq!(super::parse("123"), Ok((123, "")));
        assert_eq!(super::parse("-0"), Ok((0, "")));
        assert_eq!(super::parse("-1"), Ok((-1, "")));
        assert_eq!(super::parse("+0"), Ok((0, "")));
        assert_eq!(super::parse("+1"), Ok((1, "")));

        assert_eq!(super::parse("123abc"), Ok((123, "abc")));

        assert_eq!(super::parse("a123"), Err("a123"));
        assert_eq!(super::parse(""), Err(""));
    }
}
