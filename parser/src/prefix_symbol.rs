use crate::{tokens::PrefixSymbol, ParseResult};

static PRIMARY: [&str; 24] = [
    "m",  // milli
    "k",  // kilo
    "c",  // centi
    "d",  // deci
    "u",  // micro
    "h",  // hecto
    "M",  // mega
    "G",  // giga
    "p",  // pico
    "Y",  // yotta
    "Z",  // zetta
    "E",  // exa
    "P",  // peta
    "T",  // tera
    "da", // deka
    "n",  // nano
    "f",  // femto
    "a",  // atto
    "z",  // zepto
    "y",  // yocto
    "Ki", // kibi
    "Mi", // mebi
    "Gi", // Gibi
    "Ti", // tebi
];

// static SECONDARY: [&str; 20] = [
//     "YA", // yotta
//     "ZA", // zetta
//     "EX", // exa
//     "PT", // peta
//     "TR", // tera
//     "GA", // giga
//     "MA", // mega
//     "K",  // kilo
//     "H",  // hecto
//     "DA", // deka
//     "D",  // deci
//     "C",  // centi
//     "M",  // milli
//     "U",  // micro
//     "N",  // nano
//     "P",  // pico
//     "F",  // femto
//     "A",  // atto
//     "ZO", // zepto
//     "YO", // yocto
// ];

pub(crate) fn parse(input: &str) -> ParseResult<'_, PrefixSymbol<'_>> {
    PRIMARY
        .iter()
        .find_map(|valid_prefix| {
            if input.starts_with(*valid_prefix) {
                Some((PrefixSymbol(valid_prefix), &input[valid_prefix.len()..]))
            } else {
                None
            }
        })
        .ok_or_else(|| input)
}

#[cfg(test)]
mod tests {
    use crate::tokens::PrefixSymbol;

    #[test]
    fn test_parse() {
        assert_eq!(super::parse("k"), Ok((PrefixSymbol("k"), "")));
        assert_eq!(super::parse("0"), Err("0"));
        assert_eq!(super::parse(""), Err(""));
    }
}
