include!(concat!(env!("OUT_DIR"), "/mapper.rs"));
include!(concat!(env!("OUT_DIR"), "/symbol_parser.rs"));

pub (super) mod symbol;

#[cfg(test)]
mod symbol_parser_test;

pub (super) use self::symbol::Symbol;
