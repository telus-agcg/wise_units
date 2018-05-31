include!(concat!(env!("OUT_DIR"), "/mapper.rs"));
include!(concat!(env!("OUT_DIR"), "/symbol_parser.rs"));

pub mod symbol;

pub use self::symbol::Symbol;
