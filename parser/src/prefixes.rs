use num_rational::Ratio;

use crate::{tokens::PrefixSymbol, PrefixCollection, SymbolKey};

pub const YOTTA: PrefixSymbol = PrefixSymbol {
    name: "yotta",
    primary_code: "Y",
    value: Ratio::new_raw(1000000000000000000000000, 1),
};
pub const ZETTA: PrefixSymbol = PrefixSymbol {
    name: "zetta",
    primary_code: "Z",
    value: Ratio::new_raw(1000000000000000000000, 1),
};
pub const EXA: PrefixSymbol = PrefixSymbol {
    name: "exa",
    primary_code: "E",
    value: Ratio::new_raw(1000000000000000000, 1),
};
pub const PETA: PrefixSymbol = PrefixSymbol {
    name: "peta",
    primary_code: "P",
    value: Ratio::new_raw(1000000000000000, 1),
};
pub const TERA: PrefixSymbol = PrefixSymbol {
    name: "tera",
    primary_code: "T",
    value: Ratio::new_raw(1000000000000, 1),
};
pub const GIGA: PrefixSymbol = PrefixSymbol {
    name: "giga",
    primary_code: "G",
    value: Ratio::new_raw(1000000000, 1),
};
pub const MEGA: PrefixSymbol = PrefixSymbol {
    name: "mega",
    primary_code: "M",
    value: Ratio::new_raw(1000000, 1),
};
pub const KILO: PrefixSymbol = PrefixSymbol {
    name: "kilo",
    primary_code: "k",
    value: Ratio::new_raw(1000, 1),
};
pub const HECTO: PrefixSymbol = PrefixSymbol {
    name: "hecto",
    primary_code: "h",
    value: Ratio::new_raw(100, 1),
};
pub const DEKA: PrefixSymbol = PrefixSymbol {
    name: "deka",
    primary_code: "da",
    value: Ratio::new_raw(10, 1),
};
pub const DECI: PrefixSymbol = PrefixSymbol {
    name: "deci",
    primary_code: "d",
    value: Ratio::new_raw(1, 10),
};
pub const CENTI: PrefixSymbol = PrefixSymbol {
    name: "centi",
    primary_code: "c",
    value: Ratio::new_raw(1, 100),
};
pub const MILLI: PrefixSymbol = PrefixSymbol {
    name: "milli",
    primary_code: "m",
    value: Ratio::new_raw(1, 1000),
};
pub const MICRO: PrefixSymbol = PrefixSymbol {
    name: "micro",
    primary_code: "u",
    value: Ratio::new_raw(1, 1000000),
};
pub const NANO: PrefixSymbol = PrefixSymbol {
    name: "nano",
    primary_code: "n",
    value: Ratio::new_raw(1, 1000000000),
};
pub const PICO: PrefixSymbol = PrefixSymbol {
    name: "pico",
    primary_code: "p",
    value: Ratio::new_raw(1, 1000000000000),
};
pub const FEMTO: PrefixSymbol = PrefixSymbol {
    name: "femto",
    primary_code: "f",
    value: Ratio::new_raw(1, 1000000000000000),
};
pub const ATTO: PrefixSymbol = PrefixSymbol {
    name: "atto",
    primary_code: "a",
    value: Ratio::new_raw(1, 1000000000000000000),
};
pub const ZEPTO: PrefixSymbol = PrefixSymbol {
    name: "zepto",
    primary_code: "z",
    value: Ratio::new_raw(1, 1000000000000000000000),
};
pub const YOCTO: PrefixSymbol = PrefixSymbol {
    name: "yocto",
    primary_code: "y",
    value: Ratio::new_raw(1, 1000000000000000000000000),
};
pub const KIBI: PrefixSymbol = PrefixSymbol {
    name: "kibi",
    primary_code: "Ki",
    value: Ratio::new_raw(1024, 1),
};
pub const MEBI: PrefixSymbol = PrefixSymbol {
    name: "mebi",
    primary_code: "Mi",
    value: Ratio::new_raw(1048576, 1),
};
pub const GIBI: PrefixSymbol = PrefixSymbol {
    name: "gibi",
    primary_code: "Gi",
    value: Ratio::new_raw(1073741824, 1),
};
pub const TEBI: PrefixSymbol = PrefixSymbol {
    name: "tebi",
    primary_code: "Ti",
    value: Ratio::new_raw(1099511627776, 1),
};
pub const PREFIXES: PrefixCollection = PrefixCollection {
    inner: [
        (SymbolKey("Gi"), GIBI),
        (SymbolKey("Ki"), KIBI),
        (SymbolKey("Mi"), MEBI),
        (SymbolKey("Ti"), TEBI),
        (SymbolKey("da"), DEKA),
        (SymbolKey("E"), EXA),
        (SymbolKey("G"), GIGA),
        (SymbolKey("M"), MEGA),
        (SymbolKey("P"), PETA),
        (SymbolKey("T"), TERA),
        (SymbolKey("Y"), YOTTA),
        (SymbolKey("Z"), ZETTA),
        (SymbolKey("a"), ATTO),
        (SymbolKey("c"), CENTI),
        (SymbolKey("d"), DECI),
        (SymbolKey("f"), FEMTO),
        (SymbolKey("h"), HECTO),
        (SymbolKey("k"), KILO),
        (SymbolKey("m"), MILLI),
        (SymbolKey("n"), NANO),
        (SymbolKey("p"), PICO),
        (SymbolKey("u"), MICRO),
        (SymbolKey("y"), YOCTO),
        (SymbolKey("z"), ZEPTO),
    ],
};
