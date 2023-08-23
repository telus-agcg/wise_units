// impl<'a> Unit<'a> for crate::Unit {}

use crate::{v2::traits::ucum, Unit};

impl<'a> ucum::ParseUcumStr<'a> for Unit {
    type String = &'a str;
    type Error = crate::Error;

    fn parse_ucum_str(ucum_str: Self::String) -> Result<Self, Self::Error> {
        std::str::FromStr::from_str(ucum_str)
    }
}

impl ucum::Dim for Unit {
    type Dimension = crate::Composition;

    fn dim(&self) -> Self::Dimension {
        // Just delegate to the old trait impl for now.
        crate::Composable::composition(self)
    }
}

impl ucum::DefinitionFlags for Unit {
    fn is_special(&self) -> bool {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::is_special(self)
    }

    fn is_metric(&self) -> bool {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::is_metric(self)
    }

    fn is_arbitrary(&self) -> bool {
        // Just delegate to the old trait impl for now.
        crate::UcumUnit::is_arbitrary(self)
    }
}
