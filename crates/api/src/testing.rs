pub(crate) mod const_units {
    use std::borrow::Cow;

    use crate::{
        term::variants::{AtomExponent, PrefixAtom},
        Atom, Prefix, Term, Unit,
    };

    pub(crate) const METER: Unit = Unit {
        terms: Cow::Borrowed(&[Term::Atom(Atom::Meter)]),
    };

    pub(crate) const KILOMETER: Unit = Unit {
        terms: Cow::Borrowed(&[Term::PrefixAtom(PrefixAtom {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
        })]),
    };

    pub(crate) const GRAM: Unit = Unit {
        terms: Cow::Borrowed(&[Term::Atom(Atom::Gram)]),
    };

    pub(crate) const GRAM_METER: Unit = Unit {
        terms: Cow::Borrowed(&[Term::Atom(Atom::Gram), Term::Atom(Atom::Meter)]),
    };

    pub(crate) const PER_GRAM_METER: Unit = Unit {
        terms: Cow::Borrowed(&[
            Term::AtomExponent(AtomExponent {
                atom: Atom::Gram,
                exponent: -1,
            }),
            Term::AtomExponent(AtomExponent {
                atom: Atom::Meter,
                exponent: -1,
            }),
        ]),
    };

    pub(crate) const SECOND: Unit = Unit {
        terms: Cow::Borrowed(&[Term::Atom(Atom::Second)]),
    };

    pub(crate) const PER_SECOND: Unit = Unit {
        terms: Cow::Borrowed(&[Term::AtomExponent(AtomExponent {
            atom: Atom::Second,
            exponent: -1,
        })]),
    };

    pub(crate) const METER_PER_SECOND: Unit = Unit {
        terms: Cow::Borrowed(&[
            Term::Atom(Atom::Meter),
            Term::AtomExponent(AtomExponent {
                atom: Atom::Second,
                exponent: -1,
            }),
        ]),
    };

    pub(crate) const ACRE: Unit = Unit {
        terms: Cow::Borrowed(&[Term::Atom(Atom::AcreUS)]),
    };
}
