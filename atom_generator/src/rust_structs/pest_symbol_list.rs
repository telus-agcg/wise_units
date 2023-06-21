use serde::Serialize;

use super::RustAtomList;

#[derive(Debug, Serialize)]
pub(crate) struct PestSymbolList {
    pub(crate) primary_rule_names: Vec<PestSymbol>,
    pub(crate) secondary_rule_names: Vec<PestSymbol>,
}

impl<'a> From<&'a RustAtomList> for PestSymbolList {
    fn from(atom_list: &'a RustAtomList) -> Self {
        let mut primary_rule_names = atom_list
            .atoms
            .iter()
            .map(|a| {
                let s = a.type_name.clone();
                let code = a.primary_code.clone();

                PestSymbol::new(super::build_pest_rule_name("pri", &s), code)
            })
            .collect::<Vec<PestSymbol>>();

        sort_symbols(&mut primary_rule_names);

        let mut secondary_rule_names = atom_list
            .atoms
            .iter()
            .filter_map(|atom| {
                atom.secondary_code.as_ref().map(|secondary_code| {
                    let code = secondary_code.clone();
                    let s = atom.type_name.clone();

                    PestSymbol::new(super::build_pest_rule_name("sec", &s), code)
                })
            })
            .collect::<Vec<PestSymbol>>();

        sort_symbols(&mut secondary_rule_names);

        Self {
            primary_rule_names,
            secondary_rule_names,
        }
    }
}

//#[allow(clippy::all)]
fn sort_symbols(symbols: &mut [PestSymbol]) {
    symbols.sort_by(|a, b| a.code.cmp(&b.code));
    symbols.sort_by(|a, b| b.code.len().cmp(&a.code.len()));
}

#[derive(Debug, Serialize)]
pub(crate) struct PestSymbol {
    pub(crate) rule_name: String,
    pub(crate) code: String,
}

impl PestSymbol {
    pub(crate) const fn new(rule_name: String, code: String) -> Self {
        Self { rule_name, code }
    }
}
