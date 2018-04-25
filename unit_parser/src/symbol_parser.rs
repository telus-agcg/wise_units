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

    #[test]
    fn parse_thermochemical_calorie() {
        parses_to! {
            parser: SymbolParser,
            input: "cal_th",
            rule: Rule::pri_calorie_th,
            tokens: [
                pri_calorie_th(0, 6)
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "cal_th",
            rule: Rule::pri_atom,
            tokens: [
                pri_atom(0, 6, [pri_calorie_th(0, 6)])
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "CAL_TH",
            rule: Rule::sec_calorie_th,
            tokens: [
                sec_calorie_th(0, 6)
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "CAL_TH",
            rule: Rule::sec_atom,
            tokens: [
                sec_atom(0, 6, [sec_calorie_th(0, 6)])
            ]
        }
    }

    #[test]
    fn parse_calorie() {
        parses_to! {
            parser: SymbolParser,
            input: "cal",
            rule: Rule::pri_calorie,
            tokens: [
                pri_calorie(0, 3)
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "cal",
            rule: Rule::pri_atom,
            tokens: [
                pri_atom(0, 3, [pri_calorie(0, 3)])
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "cal",
            rule: Rule::symbol,
            tokens: [
                symbol(0, 3, [pri_atom(0, 3, [pri_calorie(0, 3)])])
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "CAL",
            rule: Rule::sec_calorie,
            tokens: [
                sec_calorie(0, 3)
            ]
        }

        parses_to! {
            parser: SymbolParser,
            input: "CAL",
            rule: Rule::sec_atom,
            tokens: [
                sec_atom(0, 3, [sec_calorie(0, 3)])
            ]
        }
    }
}
