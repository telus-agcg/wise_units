use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Annotation<'a>(pub &'a str);

impl<'a> fmt::Display for Annotation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Annotation;
    use parser::parse_Annotation;

    #[test]
    fn validate_annotation() {
        assert_eq!(parse_Annotation("{things123}").unwrap(),
                   Annotation("things123"));
    }
}
