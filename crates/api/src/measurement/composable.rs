use crate::{
    measurement::Measurement,
    parser::{Composable, Composition},
};

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl Composable for Measurement {
    #[inline]
    fn composition(&self) -> Composition {
        self.unit.composition()
    }
}

// Since the composition of a Measurement is just the composition of its Unit,
// most of the tests are with the Unit implementation.
//
#[cfg(test)]
mod tests {
    use crate::{
        measurement::Measurement,
        parser::{Composable, Composition, Dimension},
    };

    #[test]
    fn validate_composition() {
        let m = Measurement::try_new(1.0, "m").unwrap();
        let expected = Composition::new(Dimension::Length, 1);
        assert_eq!(m.composition(), expected);

        let m = Measurement::try_new(1.0, "m2/s").unwrap();
        let mut expected = Composition::new(Dimension::Length, 2);
        expected.insert(Dimension::Time, -1);

        assert_eq!(m.composition(), expected);
    }
}
