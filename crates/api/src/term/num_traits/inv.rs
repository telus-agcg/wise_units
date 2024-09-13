use num_traits::Inv;

use crate::{
    term::variants::{AtomExponent, FactorExponent, InvOutput},
    Term,
};

impl Inv for Term {
    type Output = Self;

    fn inv(self) -> Self::Output {
        match self {
            Self::Annotation(annotation) => annotation.inv().into(),
            Self::Atom(atom) => AtomExponent { atom, exponent: -1 }.into(),
            Self::AtomAnnotation(inner) => inner.inv().into(),
            Self::AtomExponent(inner) => inner.inv().into(),
            Self::AtomExponentAnnotation(inner) => inner.inv().into(),
            Self::PrefixAtom(inner) => inner.inv().into(),
            Self::PrefixAtomAnnotation(inner) => inner.inv().into(),
            Self::PrefixAtomExponent(inner) => inner.inv().into(),
            Self::PrefixAtomExponentAnnotation(inner) => inner.inv().into(),
            Self::Factor(factor) => FactorExponent {
                factor,
                exponent: -1,
            }
            .into(),
            Self::FactorAnnotation(inner) => inner.inv().into(),
            Self::FactorExponent(inner) => inner.inv().into(),
            Self::FactorExponentAnnotation(inner) => inner.inv().into(),
            Self::FactorAtom(inner) => inner.inv().into(),
            Self::FactorAtomAnnotation(inner) => inner.inv().into(),
            Self::FactorAtomExponent(inner) => inner.inv().into(),
            Self::FactorAtomExponentAnnotation(inner) => inner.inv().into(),
            Self::FactorPrefixAtom(inner) => inner.inv().into(),
            Self::FactorPrefixAtomAnnotation(inner) => inner.inv().into(),
            Self::FactorPrefixAtomExponent(inner) => inner.inv().into(),
            Self::FactorPrefixAtomExponentAnnotation(inner) => inner.inv().into(),
        }
    }
}

/// Note that in cases where an exponent is added to the `Term` _and_ the `Term` has an
/// `Annotation`, that `Annotation` has to be cloned into the new `Term`; in other words, there is
/// more than likely allocations happening here.
///
impl<'a> Inv for &'a Term {
    type Output = Term;

    fn inv(self) -> Self::Output {
        match self {
            Term::Annotation(annotation) => annotation.inv().into(),
            Term::Atom(atom) => AtomExponent {
                atom: *atom,
                exponent: -1,
            }
            .into(),
            Term::AtomAnnotation(inner) => inner.inv().into(),
            Term::AtomExponent(inner) => inner.inv().into(),
            Term::AtomExponentAnnotation(inner) => inner.inv().into(),
            Term::PrefixAtom(inner) => inner.inv().into(),
            Term::PrefixAtomAnnotation(inner) => inner.inv().into(),
            Term::PrefixAtomExponent(inner) => inner.inv().into(),
            Term::PrefixAtomExponentAnnotation(inner) => inner.inv().into(),
            Term::Factor(factor) => FactorExponent {
                factor: *factor,
                exponent: -1,
            }
            .into(),
            Term::FactorExponentAnnotation(inner) => inner.inv().into(),
            Term::FactorAnnotation(inner) => inner.inv().into(),
            Term::FactorAtom(inner) => inner.inv().into(),
            Term::FactorExponent(inner) => inner.inv().into(),
            Term::FactorAtomAnnotation(inner) => inner.inv().into(),
            Term::FactorAtomExponent(inner) => inner.inv().into(),
            Term::FactorAtomExponentAnnotation(inner) => inner.inv().into(),
            Term::FactorPrefixAtom(inner) => inner.inv().into(),
            Term::FactorPrefixAtomAnnotation(inner) => inner.inv().into(),
            Term::FactorPrefixAtomExponent(inner) => inner.inv().into(),
            Term::FactorPrefixAtomExponentAnnotation(inner) => inner.inv().into(),
        }
    }
}

impl<'a> Inv for &'a mut Term {
    type Output = Self;

    fn inv(self) -> Self::Output {
        match self {
            Term::Annotation(annotation) => {
                *self = Term::FactorExponentAnnotation(annotation.inv());
            }
            Term::Atom(atom) => {
                *self = Term::AtomExponent(AtomExponent {
                    atom: *atom,
                    exponent: -1,
                });
            }
            Term::AtomAnnotation(inner) => *self = Term::AtomExponentAnnotation(inner.inv()),
            Term::AtomExponent(inner) => match inner.inv() {
                InvOutput::One(atom) => *self = Term::Atom(atom),
                InvOutput::Rest(()) => (),
            },
            Term::AtomExponentAnnotation(inner) => match inner.inv() {
                InvOutput::One(atom_annotation) => *self = Term::AtomAnnotation(atom_annotation),
                InvOutput::Rest(()) => (),
            },
            Term::PrefixAtom(inner) => *self = Term::PrefixAtomExponent(inner.inv()),
            Term::PrefixAtomAnnotation(inner) => {
                *self = Term::PrefixAtomExponentAnnotation(inner.inv());
            }
            Term::PrefixAtomExponent(inner) => match inner.inv() {
                InvOutput::One(prefix_atom) => *self = Term::PrefixAtom(prefix_atom),
                InvOutput::Rest(()) => (),
            },
            Term::PrefixAtomExponentAnnotation(inner) => match inner.inv() {
                InvOutput::One(prefix_atom_annotation) => {
                    *self = Term::PrefixAtomAnnotation(prefix_atom_annotation);
                }
                InvOutput::Rest(()) => (),
            },
            Term::Factor(factor) => {
                *self = Term::FactorExponent(FactorExponent {
                    factor: *factor,
                    exponent: -1,
                });
            }
            Term::FactorAnnotation(inner) => *self = Term::FactorExponentAnnotation(inner.inv()),
            Term::FactorExponent(inner) => match inner.inv() {
                InvOutput::One(factor) => *self = Term::Factor(factor),
                InvOutput::Rest(()) => (),
            },
            Term::FactorExponentAnnotation(inner) => match inner.inv() {
                InvOutput::One(fa) => *self = Term::FactorAnnotation(fa),
                InvOutput::Rest(()) => (),
            },
            Term::FactorAtom(inner) => *self = Term::FactorAtomExponent(inner.inv()),
            Term::FactorAtomAnnotation(inner) => {
                *self = Term::FactorAtomExponentAnnotation(inner.inv());
            }
            Term::FactorAtomExponent(inner) => match inner.inv() {
                InvOutput::One(fa) => *self = Term::FactorAtom(fa),
                InvOutput::Rest(()) => (),
            },
            Term::FactorAtomExponentAnnotation(inner) => match inner.inv() {
                InvOutput::One(faa) => *self = Term::FactorAtomAnnotation(faa),
                InvOutput::Rest(()) => (),
            },
            Term::FactorPrefixAtom(inner) => *self = Term::FactorPrefixAtomExponent(inner.inv()),
            Term::FactorPrefixAtomAnnotation(inner) => {
                *self = Term::FactorPrefixAtomExponentAnnotation(inner.inv());
            }
            Term::FactorPrefixAtomExponent(inner) => match inner.inv() {
                InvOutput::One(fpa) => *self = Term::FactorPrefixAtom(fpa),
                InvOutput::Rest(()) => (),
            },
            Term::FactorPrefixAtomExponentAnnotation(inner) => match inner.inv() {
                InvOutput::One(fpaa) => *self = Term::FactorPrefixAtomAnnotation(fpaa),
                InvOutput::Rest(()) => (),
            },
        };

        self
    }
}

// This solves not being able to `impl Inv for &mut Vec<Term>`.
//
pub(crate) fn inv_terms(terms: &mut Vec<Term>) {
    for term in terms {
        let _ = term.inv();
    }
}

// This solves not being able to `impl Inv for Vec<Term>`.
//
pub(crate) fn inv_terms_into(terms: Vec<Term>) -> Vec<Term> {
    terms.into_iter().map(Inv::inv).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: This macro tests all 3 of the `Inv` implementations.
    //
    macro_rules! test_inv {
        ($test_name:ident, $subject:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                // Test &mut Term impl
                let mut mut_borrowed = $subject.clone();
                let _ = Inv::inv(&mut mut_borrowed);
                assert_eq!(mut_borrowed, $expected);

                // Test &Term impl
                let owned = $subject;
                let inverted = Inv::inv(&owned);
                assert_eq!(inverted, $expected);

                // Test Term impl
                let owned = $subject;
                let inverted = Inv::inv(owned);
                assert_eq!(inverted, $expected);
            }
        };
    }

    // ╭────────────╮
    // │ Annotation │
    // ╰────────────╯
    test_inv!(
        validate_annotation,
        term!(annotation: "meow"),
        term!(factor: 1, exponent: -1, annotation: "meow")
    );

    // ╭──────╮
    // │ Atom │
    // ╰──────╯
    test_inv!(validate_atom, term!(Meter), term!(Meter, exponent: -1));

    // ╭────────────────╮
    // │ AtomAnnotation │
    // ╰────────────────╯
    test_inv!(
        validate_atom_annotation,
        term!(Meter, annotation: "meow"),
        term!(Meter, annotation: "meow", exponent: -1)
    );

    // ╭──────────────╮
    // │ AtomExponent │
    // ╰──────────────╯
    test_inv!(
        validate_atom_exponent_positive_exp,
        term!(Meter, exponent: 1),
        term!(Meter, exponent: -1)
    );
    test_inv!(
        validate_atom_exponent_negative_one_exp,
        term!(Meter, exponent: -1),
        term!(Meter)
    );
    test_inv!(
        validate_atom_exponent_negative_other_exp,
        term!(Meter, exponent: -2),
        term!(Meter, exponent: 2)
    );

    // ╭────────────────────────╮
    // │ AtomExponentAnnotation │
    // ╰────────────────────────╯
    test_inv!(
        validate_atom_exponent_annotation_positive_exp,
        term!(Meter, exponent: 1, annotation: "meow"),
        term!(Meter, exponent: -1, annotation: "meow")
    );
    test_inv!(
        validate_atom_exponent_annotation_negative_one_exp,
        term!(Meter, exponent: -1, annotation: "meow"),
        term!(Meter, annotation: "meow")
    );
    test_inv!(
        validate_atom_exponent_annotation_negative_other_exp,
        term!(Meter, exponent: -2, annotation: "meow"),
        term!(Meter, exponent: 2, annotation: "meow")
    );

    // ╭────────────╮
    // │ PrefixAtom │
    // ╰────────────╯
    test_inv!(
        validate_prefix_atom,
        term!(Kilo, Meter),
        term!(Kilo, Meter, exponent: -1)
    );

    // ╭──────────────────────╮
    // │ PrefixAtomAnnotation │
    // ╰──────────────────────╯
    test_inv!(
        validate_prefix_atom_annotation,
        term!(Kilo, Meter, annotation: "meow"),
        term!(Kilo, Meter, annotation: "meow", exponent: -1)
    );

    // ╭────────────────────╮
    // │ PrefixAtomExponent │
    // ╰────────────────────╯
    test_inv!(
        validate_prefix_atom_exponent_positive_exp,
        term!(Kilo, Meter, exponent: 1),
        term!(Kilo, Meter, exponent: -1)
    );
    test_inv!(
        validate_prefix_atom_exponent_negative_one_exp,
        term!(Kilo, Meter, exponent: -1),
        term!(Kilo, Meter)
    );
    test_inv!(
        validate_prefix_atom_exponent_negative_other_exp,
        term!(Kilo, Meter, exponent: -2),
        term!(Kilo, Meter, exponent: 2)
    );

    // ╭──────────────────────────────╮
    // │ PrefixAtomExponentAnnotation │
    // ╰──────────────────────────────╯
    test_inv!(
        validate_prefix_atom_exponent_annotation_positive_exp,
        term!(Kilo, Meter, annotation: "meow", exponent: 1),
        term!(Kilo, Meter, annotation: "meow", exponent: -1)
    );
    test_inv!(
        validate_prefix_atom_exponent_annotation_negative_one_exp,
        term!(Kilo, Meter, annotation: "meow", exponent: -1),
        term!(Kilo, Meter, annotation: "meow")
    );
    test_inv!(
        validate_prefix_atom_exponent_annotation_negative_other_exp,
        term!(Kilo, Meter, annotation: "meow", exponent: -2),
        term!(Kilo, Meter, annotation: "meow", exponent: 2)
    );

    // ╭────────╮
    // │ Factor │
    // ╰────────╯
    test_inv!(
        validate_factor,
        term!(factor: 42),
        term!(factor: 42, exponent: -1)
    );

    // ╭──────────────────╮
    // │ FactorAnnotation │
    // ╰──────────────────╯
    test_inv!(
        validate_factor_annotation,
        term!(factor: 42, annotation: "meow"),
        term!(factor: 42, exponent: -1, annotation: "meow")
    );

    // ╭────────────────╮
    // │ FactorExponent │
    // ╰────────────────╯
    test_inv!(
        validate_factor_exponent_positive_exp,
        term!(factor: 42, exponent: 1),
        term!(factor: 42, exponent: -1)
    );
    test_inv!(
        validate_factor_exponent_negative_one,
        term!(factor: 42, exponent: -1),
        term!(factor: 42)
    );
    test_inv!(
        validate_factor_exponent_negative_other,
        term!(factor: 42, exponent: -2),
        term!(factor: 42, exponent: 2)
    );

    // ╭──────────────────────────╮
    // │ FactorExponentAnnotation │
    // ╰──────────────────────────╯
    test_inv!(
        validate_factor_exponent_annotation_positive_exp,
        term!(factor: 42, exponent: 1, annotation: "meow"),
        term!(factor: 42, exponent: -1, annotation: "meow")
    );
    test_inv!(
        validate_factor_exponent_annotation_negative_one,
        term!(factor: 42, exponent: -1, annotation: "meow"),
        term!(factor: 42, annotation: "meow")
    );
    test_inv!(
        validate_factor_exponent_annotation_negative_other,
        term!(factor: 42, exponent: -2, annotation: "meow"),
        term!(factor: 42, exponent: 2, annotation: "meow")
    );

    // ╭────────────╮
    // │ FactorAtom │
    // ╰────────────╯
    test_inv!(
        validate_factor_atom,
        term!(Meter, factor: 42),
        term!(Meter, factor: 42, exponent: -1)
    );

    // ╭──────────────────────╮
    // │ FactorAtomAnnotation │
    // ╰──────────────────────╯
    test_inv!(
        validate_factor_atom_annotation,
        term!(Meter, factor: 42, annotation: "meow"),
        term!(Meter, factor: 42, annotation: "meow", exponent: -1)
    );

    // ╭────────────────────╮
    // │ FactorAtomExponent │
    // ╰────────────────────╯
    test_inv!(
        validate_factor_atom_exponent_positive_exp,
        term!(Meter, factor: 42, exponent: 1),
        term!(Meter, factor: 42, exponent: -1)
    );
    test_inv!(
        validate_factor_atom_exponent_negative_one,
        term!(Meter, factor: 42, exponent: -1),
        term!(Meter, factor: 42)
    );
    test_inv!(
        validate_factor_atom_exponent_negative_other,
        term!(Meter, factor: 42, exponent: -2),
        term!(Meter, factor: 42, exponent: 2)
    );

    // ╭──────────────────────────────╮
    // │ FactorAtomExponentAnnotation │
    // ╰──────────────────────────────╯
    test_inv!(
        validate_factor_atom_exponent_annotation_positive_exp,
        term!(Meter, factor: 42, exponent: 1, annotation: "meow"),
        term!(Meter, factor: 42, exponent: -1, annotation: "meow")
    );
    test_inv!(
        validate_factor_atom_exponent_annotation_negative_one,
        term!(Meter, factor: 42, exponent: -1, annotation: "meow"),
        term!(Meter, factor: 42, annotation: "meow")
    );
    test_inv!(
        validate_factor_atom_exponent_annotation_negative_other,
        term!(Meter, factor: 42, exponent: -2, annotation: "meow"),
        term!(Meter, factor: 42, exponent: 2, annotation: "meow")
    );

    // ╭──────────────────╮
    // │ FactorPrefixAtom │
    // ╰──────────────────╯
    test_inv!(
        validate_factor_prefix_atom,
        term!(Kilo, Meter, factor: 42),
        term!(Kilo, Meter, factor: 42, exponent: -1)
    );

    // ╭────────────────────────────╮
    // │ FactorPrefixAtomAnnotation │
    // ╰────────────────────────────╯
    test_inv!(
        validate_factor_prefix_atom_annotation,
        term!(Kilo, Meter, factor: 42, annotation: "meow"),
        term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -1)
    );

    // ╭──────────────────────────╮
    // │ FactorPrefixAtomExponent │
    // ╰──────────────────────────╯
    test_inv!(
        validate_factor_prefix_atom_exponent_positive_exp,
        term!(Kilo, Meter, factor: 42, exponent: 1),
        term!(Kilo, Meter, factor: 42, exponent: -1)
    );
    test_inv!(
        validate_factor_prefix_atom_exponent_negative_one,
        term!(Kilo, Meter, factor: 42, exponent: -1),
        term!(Kilo, Meter, factor: 42)
    );
    test_inv!(
        validate_factor_prefix_atom_exponent_negative_other,
        term!(Kilo, Meter, factor: 42, exponent: -2),
        term!(Kilo, Meter, factor: 42, exponent: 2)
    );

    // ╭────────────────────────────────────╮
    // │ FactorPrefixAtomExponentAnnotation │
    // ╰────────────────────────────────────╯
    test_inv!(
        validate_factor_prefix_atom_exponent_annotation_positive_exp,
        term!(Kilo, Meter, factor: 42, exponent: 1, annotation: "meow"),
        term!(Kilo, Meter, factor: 42, exponent: -1, annotation: "meow")
    );
    test_inv!(
        validate_factor_prefix_atom_exponent_annotation_negative_one,
        term!(Kilo, Meter, factor: 42, exponent: -1, annotation: "meow"),
        term!(Kilo, Meter, factor: 42, annotation: "meow")
    );
    test_inv!(
        validate_factor_prefix_atom_exponent_annotation_negative_other,
        term!(Kilo, Meter, factor: 42, exponent: -2, annotation: "meow"),
        term!(Kilo, Meter, factor: 42, exponent: 2, annotation: "meow")
    );
}
