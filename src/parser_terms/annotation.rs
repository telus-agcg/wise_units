use std::fmt;

/// A unit can have an annotation; from the UCUM:
///
/// "The material enclosed in curly braces is called annotation. Annotations do
/// not contribute to the semantics of the unit but are meaningless by
/// definition."
///
#[derive(Debug, PartialEq)]
pub struct Annotation(pub String);

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

#[cfg(test)]
mod tests {
    use super::Annotation;
    use parser::parse_Annotation;

    #[test]
    fn validate_annotation() {
        assert_eq!(
            parse_Annotation("{things123}").unwrap(),
            Annotation("things123".to_string())
        );
    }
}
