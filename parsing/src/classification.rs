//-----------------------------------------------------------------------------
// DO NOT EDIT THIS FILE!
// This is generated at compile time.
//-----------------------------------------------------------------------------

/// Classification signifies the system of units from which a unit is defined
/// in.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Apoth,
    Avoirdupois,
    BritLength,
    BritVolumes,
    Cgs,
    Chemical,
    Clinical,
    Const,
    Dimless,
    Heat,
    Infotech,
    Intcust,
    Iso1000,
    Levels,
    Misc,
    Si,
    Troy,
    Typeset,
    UsLengths,
    UsVolumes,
}

impl Default for Classification {
    fn default() -> Classification {
        Classification::Si
    }
}
