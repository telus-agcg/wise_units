#[derive(PartialEq)]
pub struct Annotation<'a>(pub &'a str);

#[cfg(test)]
mod tests {
    use super::Annotation;
    use parser::parse_Annotation;

    #[test]
    fn validate_annotation() {
        assert_eq!(parse_Annotation("{things123}").unwrap(), Annotation("things123"));
    }
}
