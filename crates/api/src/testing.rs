pub(crate) mod const_units {
    use std::borrow::Cow;

    use crate::{Atom, Prefix, Term, Unit};

    pub(crate) const METER: Unit = Unit {
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: None,
            atom: Some(Atom::Meter),
            exponent: None,
            annotation: None,
        }]),
    };

    pub(crate) const KILOMETER: Unit = Unit {
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: Some(Prefix::Kilo),
            atom: Some(Atom::Meter),
            exponent: None,
            annotation: None,
        }]),
    };

    pub(crate) const GRAM_METER: Unit = Unit {
        terms: Cow::Borrowed(&[
            Term {
                factor: None,
                prefix: None,
                atom: Some(Atom::Gram),
                exponent: None,
                annotation: None,
            },
            Term {
                factor: None,
                prefix: None,
                atom: Some(Atom::Meter),
                exponent: None,
                annotation: None,
            },
        ]),
    };

    pub(crate) const PER_GRAM_METER: Unit = Unit {
        terms: Cow::Borrowed(&[
            Term {
                factor: None,
                prefix: None,
                atom: Some(Atom::Gram),
                exponent: Some(-1),
                annotation: None,
            },
            Term {
                factor: None,
                prefix: None,
                atom: Some(Atom::Meter),
                exponent: Some(-1),
                annotation: None,
            },
        ]),
    };

    pub(crate) const SECOND: Unit = Unit {
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: None,
            atom: Some(Atom::Second),
            exponent: None,
            annotation: None,
        }]),
    };

    pub(crate) const PER_SECOND: Unit = Unit {
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: None,
            atom: Some(Atom::Second),
            exponent: Some(-1),
            annotation: None,
        }]),
    };

    pub(crate) const METER_PER_SECOND: Unit = Unit {
        terms: Cow::Borrowed(&[
            Term {
                factor: None,
                prefix: None,
                atom: Some(Atom::Meter),
                exponent: None,
                annotation: None,
            },
            Term {
                factor: None,
                prefix: None,
                atom: Some(Atom::Second),
                exponent: Some(-1),
                annotation: None,
            },
        ]),
    };

    pub(crate) const ACRE: Unit = Unit {
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: None,
            atom: Some(Atom::AcreUS),
            exponent: None,
            annotation: None,
        }]),
    };
}
