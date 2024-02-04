use crate::{tokens::Annotation, ParseResult};

pub(super) fn parse(input: &str) -> ParseResult<'_, Annotation<'_>> {
    // Match first char or return input.
    input.strip_prefix('{').map_or(Err(input), |stripped| {
        parse_inner(stripped)
            .map(|(annotation_string, tail)| (Annotation(annotation_string), tail))
            .map_err(|_| input)
    })
}

fn parse_inner(input: &str) -> ParseResult<'_, &str> {
    let mut chars = input.chars();
    let mut found_end = false;

    for next in chars.by_ref() {
        match next {
            '!'..='z'       // 33-122
                | '|'       // 124
                | '~'       // 126
                => continue,
            '}' => {
                found_end = true;
                break;
            }
            _ => break,
        }
    }

    let annotation: &str = &input[..input.len() - chars.as_str().len() - 1];

    if found_end {
        let next_index = annotation.len() + 1;

        Ok((annotation, &input[next_index..]))
    } else {
        Err(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::Annotation;

    #[test]
    fn test_parse() {
        assert_eq!(super::parse("{stuff}"), Ok((Annotation("stuff"), "")));

        assert_eq!(
            super::parse("{stuff}123asdf"),
            Ok((Annotation("stuff"), "123asdf"))
        );

        assert_eq!(super::parse("{!@#'asdf}"), Ok((Annotation("!@#'asdf"), "")));

        // Must contain both opening and closing braces
        assert_eq!(super::parse("asdf}"), Err("asdf}"));
        assert_eq!(super::parse("{asdf"), Err("{asdf"));

        // Can't contain nested curly braces
        assert_eq!(super::parse("{{}}"), Err("{{}}"));
        assert_eq!(super::parse("{a{b}c}"), Err("{a{b}c}"));

        // Can't be empty
        assert_eq!(super::parse(""), Err(""));
    }
}
