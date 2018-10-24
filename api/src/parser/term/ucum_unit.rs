use super::Term;
use reducible::Reducible;
use ucum_unit::UcumUnit;

impl UcumUnit for Term {
    fn is_special(&self) -> bool {
        match self.atom {
            Some(ref a) => a.is_special(),
            None => false,
        }
    }

    /// The UCUM defines "arbitrary units" using three points. First:
    ///
    /// > units whose meaning entirely depends on the measurement procedure
    /// (assay). These units > have no general meaning in relation with any
    /// other unit in the SI.
    ///
    /// Second:
    ///
    /// > An arbitrary unit has no further definition in the semantic framework
    /// of The Unified Code > for Units of Measure.
    ///
    /// Third:
    ///
    /// > Arbitrary units are not “of any specific dimension” and are not
    /// “commensurable with” any > other unit.
    ///
    fn is_arbitrary(&self) -> bool {
        match self.atom {
            Some(ref a) => a.is_arbitrary(),
            None => false,
        }
    }

    /// A `Term` is metric if it has some `Atom` that is metric.
    ///
    fn is_metric(&self) -> bool {
        match self.atom {
            Some(ref a) => a.is_metric(),
            None => false,
        }
    }

    fn scalar(&self) -> f64 {
        self.reduce_value(1.0)
    }

    fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.scalar())
    }
}
