use crate::{number::Number, prefix::Prefix};

pub static PREFIXES: [Prefix; 2] = [KILO, MILLI];

pub const KILO: Prefix = Prefix {
    primary_code: "k",
    value: Number::Integer(1_000),
};

pub const MILLI: Prefix = Prefix {
    primary_code: "m",
    value: Number::Float(1e-3),
};
