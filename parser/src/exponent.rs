use crate::{tokens::Sign, ParseResult};

pub(crate) fn parse(input: &str) -> ParseResult<'_, i32> {
    let (sign, sign_tail) = crate::sign::parse(input)
        .map(|(sign, tail)| (Some(sign), tail))
        .unwrap_or_else(|_| (None, input));

    crate::digits::parse(sign_tail)
        .map(|(digits, digits_tail)| {
            let exponent: i32 = match sign {
                Some(Sign::Positive) | None => digits as i32,
                Some(Sign::Negative) => -(digits as i32),
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
