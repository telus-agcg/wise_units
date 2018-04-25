#[derive(Parser)]
#[grammar = "symbol.pest"]
pub struct SymbolParser;

#[cfg(test)]
mod tests {
    use super::{Rule, SymbolParser};

    #[test]
    fn parse_yotta() {
        parses_to! {
            parser: SymbolParser,
            input: "Y",
            rule: Rule::pri_yotta,
            tokens: [
                pri_yotta(0, 1)
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "Y",
            rule: Rule::pri_prefix,
            tokens: [
                pri_prefix(0, 1, [pri_yotta(0, 1)])
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "YA",
            rule: Rule::sec_yotta,
            tokens: [
                sec_yotta(0, 2)
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "YA",
            rule: Rule::sec_prefix,
            tokens: [
                sec_prefix(0, 2, [sec_yotta(0, 2)])
            ]
        }
    }
}
