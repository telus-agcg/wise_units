pub(crate) mod const_units {
    use std::borrow::Cow;

    use crate::{
        term::variants::{AtomExponent, PrefixAtom, PrefixAtomExponent},
        Atom, Prefix, Term, Unit,
    };

    pub(crate) mod l1 {
        use super::*;

        pub(crate) const METER: Unit = Unit {
            terms: Cow::Borrowed(&[Term::Atom(Atom::Meter)]),
        };

        pub(crate) const NANOMETER: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtom(PrefixAtom::new(Prefix::Nano, Atom::Meter))]),
        };

        pub(crate) const DECIMETER: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtom(PrefixAtom::new(Prefix::Deci, Atom::Meter))]),
        };

        pub(crate) const KILOMETER: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtom(PrefixAtom {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
            })]),
        };

        pub(crate) const NANOPARSEC: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtom(PrefixAtom {
                prefix: Prefix::Nano,
                atom: Atom::Parsec,
            })]),
        };

        pub(crate) const FOOT: Unit = Unit {
            terms: Cow::Borrowed(&[Term::Atom(Atom::FootInternational)]),
        };
    }

    pub(crate) mod l2 {
        use super::*;

        pub(crate) const METER_SQUARED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::AtomExponent(AtomExponent::new(Atom::Meter, 2))]),
        };

        pub(crate) const CENTIMETER_SQUARED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtomExponent(PrefixAtomExponent::new(
                Prefix::Centi,
                Atom::Meter,
                2,
            ))]),
        };

        pub(crate) const DECIMETER_SQUARED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtomExponent(PrefixAtomExponent::new(
                Prefix::Deci,
                Atom::Meter,
                2,
            ))]),
        };

        pub(crate) const YOCTOPARSEC_SQUARED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtomExponent(PrefixAtomExponent::new(
                Prefix::Yocto,
                Atom::Parsec,
                2,
            ))]),
        };

        pub(crate) const FOOT_SQUARED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::AtomExponent(AtomExponent::new(
                Atom::FootInternational,
                2,
            ))]),
        };

        pub(crate) const ACRE: Unit = Unit {
            terms: Cow::Borrowed(&[Term::Atom(Atom::AcreUS)]),
        };
    }

    pub(crate) mod l3 {
        use super::*;

        pub(crate) const METER_CUBED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::AtomExponent(AtomExponent::new(Atom::Meter, 3))]),
        };

        pub(crate) const DECIMETER_CUBED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtomExponent(PrefixAtomExponent::new(
                Prefix::Deci,
                Atom::Meter,
                3,
            ))]),
        };
    }

    pub(crate) mod m1 {
        use super::*;

        pub(crate) const GRAM: Unit = Unit {
            terms: Cow::Borrowed(&[Term::Atom(Atom::Gram)]),
        };

        pub(crate) const KILOGRAM: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Gram))]),
        };

        pub(crate) const CENTITONNE: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtom(PrefixAtom::new(
                Prefix::Centi,
                Atom::Tonne,
            ))]),
        };
    }

    pub(crate) mod m2 {
        use super::*;

        pub(crate) const CENTIGRAM_SQUARED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::PrefixAtomExponent(PrefixAtomExponent::new(
                Prefix::Centi,
                Atom::Gram,
                2,
            ))]),
        };
    }

    pub(crate) mod l1m1 {
        use super::*;

        pub(crate) const GRAM_METER: Unit = Unit {
            terms: Cow::Borrowed(&[Term::Atom(Atom::Gram), Term::Atom(Atom::Meter)]),
        };
    }

    pub(crate) mod t1 {
        use super::*;

        pub(crate) const SECOND: Unit = Unit {
            terms: Cow::Borrowed(&[Term::Atom(Atom::Second)]),
        };
    }

    pub(crate) mod t2 {
        use super::*;

        pub(crate) const SECOND_SQUARED: Unit = Unit {
            terms: Cow::Borrowed(&[Term::AtomExponent(AtomExponent::new(Atom::Second, 2))]),
        };
    }

    pub(crate) mod t_1 {
        use super::*;

        pub(crate) const PER_SECOND: Unit = Unit {
            terms: Cow::Borrowed(&[Term::AtomExponent(AtomExponent {
                atom: Atom::Second,
                exponent: -1,
            })]),
        };
    }

    pub(crate) mod l_1m_1 {
        use super::*;

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
    }

    pub(crate) mod l1t_1 {
        use super::*;

        pub(crate) const METER_PER_SECOND: Unit = Unit {
            terms: Cow::Borrowed(&[
                Term::Atom(Atom::Meter),
                Term::AtomExponent(AtomExponent {
                    atom: Atom::Second,
                    exponent: -1,
                }),
            ]),
        };
    }
}
