use crate::ParseResult;

pub(crate) fn parse(input: &str) -> ParseResult<'_, u32> {
    crate::digits::parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        assert_eq!(super::parse("0"), Ok((0, "")));
        assert_eq!(super::parse("0123"), Ok((123, "")));
        assert_eq!(super::parse("0123four"), Ok((123, "four")));
        assert_eq!(super::parse("a123"), Err("a123"));
        assert_eq!(super::parse(""), Err(""));
    }
}
