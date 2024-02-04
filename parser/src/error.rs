#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Unparsable(String),
    PartialMatch { matching: String, remaining: String },
}
