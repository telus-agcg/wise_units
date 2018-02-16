/// Classification signifies the system of units from which a unit is defined
/// in.
/// 
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Apoth,
    Avoirdupois,
    BritLength,
    BritVolumes,
    CGS,
    Chemical,
    Clinical,
    Const,
    Dimless,
    Heat,
    Infotech,
    Intcust,
    ISO1000,
    Levels,
    Misc,
    SI,
    Troy,
    Typeset,
    USLengths,
    USVolumes,
}

impl Default for Classification {
    fn default() -> Classification {
        Classification::SI
    }
}
