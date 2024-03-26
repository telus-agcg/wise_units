use crate::{Atom, Composable, Composition, Dimension};

impl Composable for Atom {
    fn composition(&self) -> Composition {
        match self {
            Self::Candela => Composition::new(Dimension::LuminousIntensity, 1),
            Self::Coulomb => Composition::new(Dimension::ElectricCharge, 1),
            Self::Gram => Composition::new(Dimension::Mass, 1),
            Self::Kelvin => Composition::new(Dimension::Temperature, 1),
            Self::Meter => Composition::new(Dimension::Length, 1),
            Self::Radian => Composition::new(Dimension::PlaneAngle, 1),
            Self::Second => Composition::new(Dimension::Time, 1),
            _ => self.definition().terms().composition(),
        }
    }
}
