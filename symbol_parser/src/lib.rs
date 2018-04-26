// Only include macros for testing
#[cfg(test)]
#[macro_use(parses_to, consumes_to, fails_with)]
extern crate pest;

#[cfg(not(test))]
extern crate pest;

#[macro_use]
extern crate pest_derive;

pub mod parser;
