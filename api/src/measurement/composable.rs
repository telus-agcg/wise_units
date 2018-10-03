use measurement::Measurement;
use parser::{Composable, Composition};

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl Composable for Measurement {
    fn composition(&self) -> Composition {
        self.unit.composition()
    }
}

impl<'a> Composable for &'a Measurement {
    fn composition(&self) -> Composition {
        self.unit.composition()
    }
}

#[cfg(test)]
mod tests {
    use measurement::Measurement;
    use parser::Composable;

    // The method is really just a convenience wrapper to check the Measurement's Unit, so most of
    // the testing can be done on Unit.
    #[test]
    fn validate_is_compatible_with() {
        let subject = Measurement::new(1.0, "m").unwrap();
        let other = Measurement::new(1.0, "m").unwrap();

        assert!(subject.is_compatible_with(&other));
    }
}
