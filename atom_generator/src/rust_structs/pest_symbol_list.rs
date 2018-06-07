use super::RustAtomList;

#[derive(Debug, Serialize)]
pub(crate) struct PestSymbolList {
    pub primary_rule_names: Vec<PestSymbol>,
    pub secondary_rule_names: Vec<PestSymbol>,
}

impl<'a> From<&'a RustAtomList> for PestSymbolList {
    fn from(atom_list: &'a RustAtomList) -> PestSymbolList {
        let mut primary_rule_names = atom_list
            .atoms
            .iter()
            .map(|a| {
                let s = a.type_name.clone();
                let code = a.primary_code.clone();

                PestSymbol::new(super::build_pest_rule_name("pri", &s), code)
            })
            .collect();

        sort_symbols(&mut primary_rule_names);

        let mut secondary_rule_names = atom_list
            .atoms
            .iter()
            .filter(|ref a| a.secondary_code.is_some())
            .map(|ref a| {
                let s = a.type_name.clone();
                let code = a.secondary_code.clone().unwrap();

                PestSymbol::new(super::build_pest_rule_name("sec", &s), code)
            })
            .collect();

        sort_symbols(&mut secondary_rule_names);

        PestSymbolList {
            primary_rule_names,
            secondary_rule_names,
        }
    }
}

fn sort_symbols(symbols: &mut Vec<PestSymbol>) {
    symbols.sort_by(|ref a, ref b| a.code.cmp(&b.code));
    symbols.sort_by(|ref a, ref b| b.code.len().cmp(&a.code.len()));
}

#[derive(Debug, Serialize)]
pub(crate) struct PestSymbol {
    pub rule_name: String,
    pub code: String,
}

impl PestSymbol {
    pub fn new(rule_name: String, code: String) -> Self {
        PestSymbol { rule_name, code }
    }
}
