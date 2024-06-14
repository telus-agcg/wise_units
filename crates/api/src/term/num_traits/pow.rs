use num_traits::Pow;

use crate::{
    term::{
        variants::{AtomExponent, FactorExponent},
        Exponent, PowOutput, UNITY,
    },
    Term,
};

impl Pow<Exponent> for Term {
    type Output = Self;

    /// NOTE: Implementation here assumes that when you don't have an atom-based Term, the
    /// atom/base unit is the unity (1).
    ///
    fn pow(self, rhs: Exponent) -> Self::Output {
        match self {
            Self::Annotation(inner) => inner.pow(rhs).into(),
            Self::Atom(atom) => match rhs {
                0 => UNITY,
                1 => self,
                exponent => Self::AtomExponent(AtomExponent { atom, exponent }),
            },
            Self::AtomAnnotation(inner) => inner.pow(rhs).into(),
            Self::AtomExponent(inner) => inner.pow(rhs).into(),
            Self::AtomExponentAnnotation(inner) => inner.pow(rhs).into(),
            Self::PrefixAtom(inner) => inner.pow(rhs).into(),
            Self::PrefixAtomAnnotation(inner) => inner.pow(rhs).into(),
            Self::PrefixAtomExponent(inner) => inner.pow(rhs).into(),
            Self::PrefixAtomExponentAnnotation(inner) => inner.pow(rhs).into(),
            Self::Factor(factor) => match rhs {
                0 => UNITY,
                1 => self,
                exponent => Self::FactorExponent(FactorExponent { factor, exponent }),
            },
            Self::FactorAnnotation(inner) => inner.pow(rhs).into(),
            Self::FactorExponent(inner) => inner.pow(rhs).into(),
            Self::FactorExponentAnnotation(inner) => inner.pow(rhs).into(),
            Self::FactorAtom(inner) => inner.pow(rhs).into(),
            Self::FactorAtomAnnotation(inner) => inner.pow(rhs).into(),
            Self::FactorAtomExponent(inner) => inner.pow(rhs).into(),
            Self::FactorAtomExponentAnnotation(inner) => inner.pow(rhs).into(),
            Self::FactorPrefixAtom(inner) => inner.pow(rhs).into(),
            Self::FactorPrefixAtomAnnotation(inner) => inner.pow(rhs).into(),
            Self::FactorPrefixAtomExponent(inner) => inner.pow(rhs).into(),
            Self::FactorPrefixAtomExponentAnnotation(inner) => inner.pow(rhs).into(),
        }
    }
}

impl<'a> Pow<Exponent> for &'a Term {
    type Output = Term;

    fn pow(self, rhs: Exponent) -> Self::Output {
        match self {
            Term::Annotation(inner) => inner.pow(rhs).into(),
            Term::Atom(atom) => match rhs {
                0 => UNITY,
                1 => Term::Atom(*atom),
                exponent => Term::AtomExponent(AtomExponent {
                    atom: *atom,
                    exponent,
                }),
            },
            Term::AtomAnnotation(inner) => inner.pow(rhs).into(),
            Term::AtomExponent(inner) => inner.pow(rhs).into(),
            Term::AtomExponentAnnotation(inner) => inner.pow(rhs).into(),
            Term::PrefixAtom(inner) => inner.pow(rhs).into(),
            Term::PrefixAtomAnnotation(inner) => inner.pow(rhs).into(),
            Term::PrefixAtomExponent(inner) => inner.pow(rhs).into(),
            Term::PrefixAtomExponentAnnotation(inner) => inner.pow(rhs).into(),
            Term::Factor(factor) => match rhs {
                0 => UNITY,
                1 => self.clone(),
                exponent => Term::FactorExponent(FactorExponent {
                    factor: *factor,
                    exponent,
                }),
            },
            Term::FactorAnnotation(inner) => inner.pow(rhs).into(),
            Term::FactorExponent(inner) => inner.pow(rhs).into(),
            Term::FactorExponentAnnotation(inner) => inner.pow(rhs).into(),
            Term::FactorAtom(inner) => inner.pow(rhs).into(),
            Term::FactorAtomAnnotation(inner) => inner.pow(rhs).into(),
            Term::FactorAtomExponent(inner) => inner.pow(rhs).into(),
            Term::FactorAtomExponentAnnotation(inner) => inner.pow(rhs).into(),
            Term::FactorPrefixAtom(inner) => inner.pow(rhs).into(),
            Term::FactorPrefixAtomAnnotation(inner) => inner.pow(rhs).into(),
            Term::FactorPrefixAtomExponent(inner) => inner.pow(rhs).into(),
            Term::FactorPrefixAtomExponentAnnotation(inner) => inner.pow(rhs).into(),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut Term {
    type Output = Self;

    #[allow(clippy::too_many_lines)]
    fn pow(self, rhs: Exponent) -> Self::Output {
        match self {
            Term::Annotation(annotation) => match annotation.pow(rhs) {
                PowOutput::Zero(()) | PowOutput::One(()) => (),
                PowOutput::Rest(factor_exponent_annotation) => {
                    *self = Term::FactorExponentAnnotation(factor_exponent_annotation);
                }
            },
            Term::Atom(atom) => match rhs {
                0 => *self = UNITY,
                1 => (),
                exponent => {
                    *self = Term::AtomExponent(AtomExponent {
                        atom: *atom,
                        exponent,
                    });
                }
            },
            Term::AtomAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(atom_exponent_annotation) => {
                    *self = Term::AtomExponentAnnotation(atom_exponent_annotation);
                }
            },
            Term::AtomExponent(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) => *self = Term::Factor(factor),
                PowOutput::One(atom) => *self = Term::Atom(atom),
                PowOutput::Rest(()) => (),
            },
            Term::AtomExponentAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(atom_annotation) => *self = Term::AtomAnnotation(atom_annotation),
                PowOutput::Rest(()) => (),
            },
            Term::PrefixAtom(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) => *self = Term::Factor(factor),
                PowOutput::One(()) => (),
                PowOutput::Rest(prefix_atom_exponent) => {
                    *self = Term::PrefixAtomExponent(prefix_atom_exponent);
                }
            },
            Term::PrefixAtomAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(prefix_atom_exponent_annotation) => {
                    *self = Term::PrefixAtomExponentAnnotation(prefix_atom_exponent_annotation);
                }
            },
            Term::PrefixAtomExponent(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) => *self = Term::Factor(factor),
                PowOutput::One(prefix_atom) => *self = Term::PrefixAtom(prefix_atom),
                PowOutput::Rest(()) => (),
            },
            Term::PrefixAtomExponentAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(prefix_atom_annotation) => {
                    *self = Term::PrefixAtomAnnotation(prefix_atom_annotation);
                }
                PowOutput::Rest(()) => (),
            },
            Term::Factor(factor) => match rhs {
                0 => *self = UNITY,
                1 => (),
                exponent => {
                    *self = Term::FactorExponent(FactorExponent {
                        factor: *factor,
                        exponent,
                    });
                }
            },
            Term::FactorAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(factor_exponent_annotation) => {
                    *self = Term::FactorExponentAnnotation(factor_exponent_annotation);
                }
            },
            Term::FactorExponent(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) | PowOutput::One(factor) => *self = Term::Factor(factor),
                PowOutput::Rest(()) => (),
            },
            Term::FactorExponentAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(factor_annotation) => {
                    *self = Term::FactorAnnotation(factor_annotation);
                }
                PowOutput::Rest(()) => (),
            },
            Term::FactorAtom(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) => *self = Term::Factor(factor),
                PowOutput::One(()) => (),
                PowOutput::Rest(factor_atom_exponent) => {
                    *self = Term::FactorAtomExponent(factor_atom_exponent);
                }
            },
            Term::FactorAtomAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(factor_atom_exponent_annotation) => {
                    *self = Term::FactorAtomExponentAnnotation(factor_atom_exponent_annotation);
                }
            },
            Term::FactorAtomExponent(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) => *self = Term::Factor(factor),
                PowOutput::One(factor_atom) => *self = Term::FactorAtom(factor_atom),
                PowOutput::Rest(()) => (),
            },
            Term::FactorAtomExponentAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(factor_atom_annotation) => {
                    *self = Term::FactorAtomAnnotation(factor_atom_annotation);
                }
                PowOutput::Rest(()) => (),
            },
            Term::FactorPrefixAtom(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) => *self = Term::Factor(factor),
                PowOutput::One(()) => (),
                PowOutput::Rest(factor_prefix_atom_exponent) => {
                    *self = Term::FactorPrefixAtomExponent(factor_prefix_atom_exponent);
                }
            },
            Term::FactorPrefixAtomAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(()) => (),
                PowOutput::Rest(factor_prefix_atom_exponent_annotation) => {
                    *self = Term::FactorPrefixAtomExponentAnnotation(
                        factor_prefix_atom_exponent_annotation,
                    );
                }
            },
            Term::FactorPrefixAtomExponent(inner) => match inner.pow(rhs) {
                PowOutput::Zero(factor) => *self = Term::Factor(factor),
                PowOutput::One(factor_prefix_atom) => {
                    *self = Term::FactorPrefixAtom(factor_prefix_atom);
                }
                PowOutput::Rest(()) => (),
            },
            Term::FactorPrefixAtomExponentAnnotation(inner) => match inner.pow(rhs) {
                PowOutput::Zero(annotation) => *self = Term::Annotation(annotation),
                PowOutput::One(factor_prefix_atom_annotation) => {
                    *self = Term::FactorPrefixAtomAnnotation(factor_prefix_atom_annotation);
                }
                PowOutput::Rest(()) => (),
            },
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{Annotation, Atom, Prefix};

    use super::*;

    // NOTE: This macro tests all 3 of the `Pow` implementations.
    //
    macro_rules! test_pow {
        ($test_name:ident, $subject:expr => $pow:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                // Test &mut Term impl
                {
                    let mut mut_borrowed = $subject.clone();
                    let _ = Pow::pow(&mut mut_borrowed, $pow);
                    assert_eq!(mut_borrowed, $expected);
                }

                // Test &Term impl
                {
                    let owned = $subject;
                    let powered = Pow::pow(&owned, $pow);
                    assert_eq!(powered, $expected);
                }

                // Test Term impl
                {
                    let owned = $subject;
                    let powered = Pow::pow(owned, $pow);
                    assert_eq!(powered, $expected);
                }
            }
        };
    }

    // ╭────────────╮
    // │ Annotation │
    // ╰────────────╯
    mod annotation {
        use super::*;

        fn subject() -> Term {
            Term::Annotation(Annotation::from("meow"))
        }

        test_pow!(validate_to_0, subject() => 0, term!(annotation: "meow"));
        test_pow!(validate_to_1, subject() => 1, term!(annotation: "meow"));
        test_pow!(validate_to_2, subject() => 2, term!(annotation: "meow", exponent: 2, factor: 1));
        test_pow!(validate_to_minus_1, subject() => -1, term!(annotation: "meow", exponent: -1, factor: 1));
        test_pow!(validate_to_minus_2, subject() => -2, term!(annotation: "meow", exponent: -2, factor: 1));
    }

    // ╭──────╮
    // │ Atom │
    // ╰──────╯
    mod atom {
        use super::*;

        const SUBJECT: Term = Term::Atom(Atom::Meter);

        test_pow!(validate_atom_to_0, SUBJECT => 0, Term::Factor(1));
        test_pow!(validate_atom_to_1, SUBJECT => 1, term!(Meter));
        test_pow!(validate_atom_to_2, SUBJECT => 2, term!(Meter, exponent: 2));
        test_pow!(validate_atom_to_minus_1, SUBJECT => -1, term!(Meter, exponent: -1));
        test_pow!(validate_atom_to_minus_2, SUBJECT => -2, term!(Meter, exponent: -2));
    }

    // ╭────────────────╮
    // │ AtomAnnotation │
    // ╰────────────────╯
    mod atom_annotation {
        use crate::term::AtomAnnotation;

        use super::*;

        fn subject() -> Term {
            Term::AtomAnnotation(AtomAnnotation {
                atom: Atom::Meter,
                annotation: Annotation::from("meow"),
            })
        }

        test_pow!(validate_to_0, subject() => 0, term!(annotation: "meow"));
        test_pow!(validate_to_1, subject() => 1, term!(Meter, annotation: "meow"));
        test_pow!(validate_to_2, subject() => 2, term!(Meter, annotation: "meow", exponent: 2));
        test_pow!(validate_to_minus_1, subject() => -1, term!(Meter, annotation: "meow", exponent: -1));
        test_pow!(validate_to_minus_2, subject() => -2, term!(Meter, annotation: "meow", exponent: -2));
    }

    // ╭──────────────╮
    // │ AtomExponent │
    // ╰──────────────╯
    mod atom_exponent {
        use super::*;

        const SUBJECT_1: Term = Term::AtomExponent(AtomExponent {
            atom: Atom::Meter,
            exponent: 1,
        });
        const SUBJECT_2: Term = Term::AtomExponent(AtomExponent {
            atom: Atom::Meter,
            exponent: 2,
        });
        const SUBJECT_MINUS_1: Term = Term::AtomExponent(AtomExponent {
            atom: Atom::Meter,
            exponent: -1,
        });
        const SUBJECT_MINUS_2: Term = Term::AtomExponent(AtomExponent {
            atom: Atom::Meter,
            exponent: -2,
        });

        // .pow(0)
        test_pow!(validate_1_to_0, SUBJECT_1 => 0, Term::Factor(1));
        test_pow!(validate_2_to_0, SUBJECT_2 => 0, Term::Factor(1));
        test_pow!(validate_minus_1_to_0, SUBJECT_MINUS_1 => 0, Term::Factor(1));
        test_pow!(validate_minus_2_to_0, SUBJECT_MINUS_2 => 0, Term::Factor(1));

        // .pow(1)
        test_pow!(validate_1_to_1, SUBJECT_1 => 1, SUBJECT_1);
        test_pow!(validate_2_to_1, SUBJECT_2 => 1, SUBJECT_2);
        test_pow!(validate_minus_1_to_1, SUBJECT_MINUS_1 => 1, SUBJECT_MINUS_1);
        test_pow!(validate_minus_2_to_1, SUBJECT_MINUS_2 => 1, SUBJECT_MINUS_2);

        // .pow(2)
        test_pow!(validate_1_to_2, SUBJECT_1 => 2, term!(Meter, exponent: 2));
        test_pow!(validate_2_to_2, SUBJECT_2 => 2, term!(Meter, exponent: 4));
        test_pow!(validate_minus_1_to_2, SUBJECT_MINUS_1 => 2, term!(Meter, exponent: -2));
        test_pow!(validate_minus_2_to_2, SUBJECT_MINUS_2 => 2, term!(Meter, exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, SUBJECT_1 => -1, term!(Meter, exponent: -1));
        test_pow!(validate_2_to_minus_1, SUBJECT_2 => -1, term!(Meter, exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, SUBJECT_MINUS_1 => -1, term!(Meter, exponent: 1));
        test_pow!(validate_minus_2_to_minus_1, SUBJECT_MINUS_2 => -1, term!(Meter, exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, SUBJECT_1 => -2, term!(Meter, exponent: -2));
        test_pow!(validate_2_to_minus_2, SUBJECT_2 => -2, term!(Meter, exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, SUBJECT_MINUS_1 => -2, term!(Meter, exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, SUBJECT_MINUS_2 => -2, term!(Meter, exponent: 4));
    }

    // ╭────────────────────────╮
    // │ AtomExponentAnnotation │
    // ╰────────────────────────╯
    mod atom_exponent_annotation {
        use crate::term::AtomExponentAnnotation;

        use super::*;

        fn subject_1() -> Term {
            Term::AtomExponentAnnotation(AtomExponentAnnotation {
                atom: Atom::Meter,
                exponent: 1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_2() -> Term {
            Term::AtomExponentAnnotation(AtomExponentAnnotation {
                atom: Atom::Meter,
                exponent: 2,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_1() -> Term {
            Term::AtomExponentAnnotation(AtomExponentAnnotation {
                atom: Atom::Meter,
                exponent: -1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_2() -> Term {
            Term::AtomExponentAnnotation(AtomExponentAnnotation {
                atom: Atom::Meter,
                exponent: -2,
                annotation: Annotation::from("meow"),
            })
        }

        // .pow(0)
        test_pow!(validate_1_to_0, subject_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_2_to_0, subject_2() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_1_to_0, subject_minus_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_2_to_0, subject_minus_2() => 0, term!(annotation: "meow"));

        // .pow(1)
        test_pow!(validate_1_to_1, subject_1() => 1, subject_1());
        test_pow!(validate_2_to_1, subject_2() => 1, subject_2());
        test_pow!(validate_minus_1_to_1, subject_minus_1() => 1, subject_minus_1());
        test_pow!(validate_minus_2_to_1, subject_minus_2() => 1, subject_minus_2());

        // .pow(2)
        test_pow!(validate_1_to_2, subject_1() => 2, term!(Meter, annotation: "meow", exponent: 2));
        test_pow!(validate_2_to_2, subject_2() => 2, term!(Meter, annotation: "meow", exponent: 4));
        test_pow!(validate_minus_1_to_2, subject_minus_1() => 2, term!(Meter, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_2_to_2, subject_minus_2() => 2, term!(Meter, annotation: "meow", exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, subject_1() => -1, term!(Meter, annotation: "meow", exponent: -1));
        test_pow!(validate_2_to_minus_1, subject_2() => -1, term!(Meter, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, subject_minus_1() => -1, term!(Meter, annotation: "meow", exponent: 1));
        test_pow!(validate_minus_2_to_minus_1, subject_minus_2() => -1, term!(Meter, annotation: "meow", exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, subject_1() => -2, term!(Meter, annotation: "meow", exponent: -2));
        test_pow!(validate_2_to_minus_2, subject_2() => -2, term!(Meter, annotation: "meow", exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, subject_minus_1() => -2, term!(Meter, annotation: "meow", exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, subject_minus_2() => -2, term!(Meter, annotation: "meow", exponent: 4));
    }

    // ╭────────────╮
    // │ PrefixAtom │
    // ╰────────────╯
    mod prefix_atom {
        use super::*;

        use crate::term::PrefixAtom;

        const SUBJECT: Term = Term::PrefixAtom(PrefixAtom {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
        });

        test_pow!(validate_to_0, SUBJECT => 0, Term::Factor(1));
        test_pow!(validate_to_1, SUBJECT => 1, SUBJECT);
        test_pow!(validate_to_2, SUBJECT => 2, term!(Kilo, Meter, exponent: 2));
        test_pow!(validate_to_minus_1, SUBJECT => -1, term!(Kilo, Meter, exponent: -1));
        test_pow!(validate_to_minus_2, SUBJECT => -2, term!(Kilo, Meter, exponent: -2));
    }

    // ╭──────────────────────╮
    // │ PrefixAtomAnnotation │
    // ╰──────────────────────╯
    mod prefix_atom_annotation {
        use super::*;

        use crate::term::PrefixAtomAnnotation;

        fn subject() -> Term {
            Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                annotation: Annotation::from("meow"),
            })
        }

        test_pow!(validate_to_0, subject() => 0, Term::Annotation(Annotation::from("meow")));
        test_pow!(validate_to_1, subject() => 1, subject());
        test_pow!(validate_to_2, subject() => 2, term!(Kilo, Meter, annotation: "meow", exponent: 2));
        test_pow!(validate_to_minus_1, subject() => -1, term!(Kilo, Meter, annotation: "meow", exponent: -1));
        test_pow!(validate_to_minus_2, subject() => -2, term!(Kilo, Meter, annotation: "meow", exponent: -2));
    }
    // ╭────────────────────╮
    // │ PrefixAtomExponent │
    // ╰────────────────────╯
    mod prefix_atom_exponent {
        use crate::term::PrefixAtomExponent;

        use super::*;

        const SUBJECT_1: Term = Term::PrefixAtomExponent(PrefixAtomExponent {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: 1,
        });

        const SUBJECT_2: Term = Term::PrefixAtomExponent(PrefixAtomExponent {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: 2,
        });

        const SUBJECT_MINUS_1: Term = Term::PrefixAtomExponent(PrefixAtomExponent {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -1,
        });

        const SUBJECT_MINUS_2: Term = Term::PrefixAtomExponent(PrefixAtomExponent {
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -2,
        });

        // .pow(0)
        test_pow!(validate_1_to_0, SUBJECT_1 => 0, Term::Factor(1));
        test_pow!(validate_2_to_0, SUBJECT_2 => 0, Term::Factor(1));
        test_pow!(validate_minus_1_to_0, SUBJECT_MINUS_1 => 0, Term::Factor(1));
        test_pow!(validate_minus_2_to_0, SUBJECT_MINUS_2 => 0, Term::Factor(1));

        // .pow(1)
        test_pow!(validate_1_to_1, SUBJECT_1 => 1, SUBJECT_1);
        test_pow!(validate_2_to_1, SUBJECT_2 => 1, SUBJECT_2);
        test_pow!(validate_minus_1_to_1, SUBJECT_MINUS_1 => 1, SUBJECT_MINUS_1);
        test_pow!(validate_minus_2_to_1, SUBJECT_MINUS_2 => 1, SUBJECT_MINUS_2);

        // .pow(2)
        test_pow!(validate_1_to_2, SUBJECT_1 => 2, term!(Kilo, Meter, exponent: 2));
        test_pow!(validate_2_to_2, SUBJECT_2 => 2, term!(Kilo, Meter, exponent: 4));
        test_pow!(validate_minus_1_to_2, SUBJECT_MINUS_1 => 2, term!(Kilo, Meter, exponent: -2));
        test_pow!(validate_minus_2_to_2, SUBJECT_MINUS_2 => 2, term!(Kilo, Meter, exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, SUBJECT_1 => -1, term!(Kilo, Meter, exponent: -1));
        test_pow!(validate_2_to_minus_1, SUBJECT_2 => -1, term!(Kilo, Meter, exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, SUBJECT_MINUS_1 => -1, term!(Kilo, Meter, exponent: 1));
        test_pow!(validate_minus_2_to_minus_1, SUBJECT_MINUS_2 => -1, term!(Kilo, Meter, exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, SUBJECT_1 => -2, term!(Kilo, Meter, exponent: -2));
        test_pow!(validate_2_to_minus_2, SUBJECT_2 => -2, term!(Kilo, Meter, exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, SUBJECT_MINUS_1 => -2, term!(Kilo, Meter, exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, SUBJECT_MINUS_2 => -2, term!(Kilo, Meter, exponent: 4));
    }

    // ╭──────────────────────────────╮
    // │ PrefixAtomExponentAnnotation │
    // ╰──────────────────────────────╯
    mod prefix_atom_exponent_annotation {
        use crate::term::PrefixAtomExponentAnnotation;

        use super::*;

        fn subject_1() -> Term {
            Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: 1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_2() -> Term {
            Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: 2,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_1() -> Term {
            Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: -1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_2() -> Term {
            Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: -2,
                annotation: Annotation::from("meow"),
            })
        }

        // .pow(0)
        test_pow!(validate_1_to_0, subject_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_2_to_0, subject_2() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_1_to_0, subject_minus_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_2_to_0, subject_minus_2() => 0, term!(annotation: "meow"));

        // .pow(1)
        test_pow!(validate_1_to_1, subject_1() => 1, subject_1());
        test_pow!(validate_2_to_1, subject_2() => 1, subject_2());
        test_pow!(validate_minus_1_to_1, subject_minus_1() => 1, subject_minus_1());
        test_pow!(validate_minus_2_to_1, subject_minus_2() => 1, subject_minus_2());

        // .pow(2)
        test_pow!(validate_1_to_2, subject_1() => 2, term!(Kilo, Meter, annotation: "meow", exponent: 2));
        test_pow!(validate_2_to_2, subject_2() => 2, term!(Kilo, Meter, annotation: "meow", exponent: 4));
        test_pow!(validate_minus_1_to_2, subject_minus_1() => 2, term!(Kilo, Meter, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_2_to_2, subject_minus_2() => 2, term!(Kilo, Meter, annotation: "meow", exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, subject_1() => -1, term!(Kilo, Meter, annotation: "meow", exponent: -1));
        test_pow!(validate_2_to_minus_1, subject_2() => -1, term!(Kilo, Meter, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, subject_minus_1() => -1, term!(Kilo, Meter, annotation: "meow", exponent: 1));
        test_pow!(validate_minus_2_to_minus_1, subject_minus_2() => -1, term!(Kilo, Meter, annotation: "meow", exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, subject_1() => -2, term!(Kilo, Meter, annotation: "meow", exponent: -2));
        test_pow!(validate_2_to_minus_2, subject_2() => -2, term!(Kilo, Meter, annotation: "meow", exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, subject_minus_1() => -2, term!(Kilo, Meter, annotation: "meow", exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, subject_minus_2() => -2, term!(Kilo, Meter, annotation: "meow", exponent: 4));
    }

    // ╭────────╮
    // │ Factor │
    // ╰────────╯
    mod factor {
        use super::*;

        const SUBJECT: Term = Term::Factor(42);

        test_pow!(validate_to_0, SUBJECT => 0, Term::Factor(1));
        test_pow!(validate_to_1, SUBJECT => 1, SUBJECT);
        test_pow!(validate_to_2, SUBJECT => 2, term!(factor: 42, exponent: 2));
        test_pow!(validate_to_minus_1, SUBJECT => -1, term!(factor: 42, exponent: -1));
        test_pow!(validate_to_minus_2, SUBJECT => -2, term!(factor: 42, exponent: -2));
    }

    // ╭──────────────────╮
    // │ FactorAnnotation │
    // ╰──────────────────╯
    mod factor_annotation {
        use crate::term::FactorAnnotation;

        use super::*;

        fn subject() -> Term {
            Term::FactorAnnotation(FactorAnnotation {
                factor: 42,
                annotation: Annotation::from("meow"),
            })
        }

        test_pow!(validate_to_0, subject() => 0, term!(annotation: "meow"));
        test_pow!(validate_to_1, subject() => 1, subject());
        test_pow!(validate_to_2, subject() => 2, term!(factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_to_minus_1, subject() => -1, term!(factor: 42, annotation: "meow", exponent: -1));
        test_pow!(validate_to_minus_2, subject() => -2, term!(factor: 42, annotation: "meow", exponent: -2));
    }

    // ╭────────────────╮
    // │ FactorExponent │
    // ╰────────────────╯
    mod factor_exponent {
        use super::*;

        const SUBJECT_1: Term = Term::FactorExponent(FactorExponent {
            factor: 42,
            exponent: 1,
        });
        const SUBJECT_2: Term = Term::FactorExponent(FactorExponent {
            factor: 42,
            exponent: 2,
        });
        const SUBJECT_MINUS_1: Term = Term::FactorExponent(FactorExponent {
            factor: 42,
            exponent: -1,
        });
        const SUBJECT_MINUS_2: Term = Term::FactorExponent(FactorExponent {
            factor: 42,
            exponent: -2,
        });

        // .pow(0)
        test_pow!(validate_1_to_0, SUBJECT_1 => 0, Term::Factor(1));
        test_pow!(validate_2_to_0, SUBJECT_2 => 0, Term::Factor(1));
        test_pow!(validate_minus_1_to_0, SUBJECT_MINUS_1 => 0, Term::Factor(1));
        test_pow!(validate_minus_2_to_0, SUBJECT_MINUS_2 => 0, Term::Factor(1));

        // .pow(1)
        test_pow!(validate_1_to_1, SUBJECT_1 => 1, term!(factor: 42));
        test_pow!(validate_2_to_1, SUBJECT_2 => 1, SUBJECT_2);
        test_pow!(validate_minus_1_to_1, SUBJECT_MINUS_1 => 1, SUBJECT_MINUS_1);
        test_pow!(validate_minus_2_to_1, SUBJECT_MINUS_2 => 1, SUBJECT_MINUS_2);

        // .pow(2)
        test_pow!(validate_1_to_2, SUBJECT_1 => 2, term!(factor: 42, exponent: 2));
        test_pow!(validate_2_to_2, SUBJECT_2 => 2, term!(factor: 42, exponent: 4));
        test_pow!(validate_minus_1_to_2, SUBJECT_MINUS_1 => 2, term!(factor: 42, exponent: -2));
        test_pow!(validate_minus_2_to_2, SUBJECT_MINUS_2 => 2, term!(factor: 42, exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, SUBJECT_1 => -1, term!(factor: 42, exponent: -1));
        test_pow!(validate_2_to_minus_1, SUBJECT_2 => -1, term!(factor: 42, exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, SUBJECT_MINUS_1 => -1, term!(factor: 42, exponent: 1));
        test_pow!(validate_minus_2_to_minus_1, SUBJECT_MINUS_2 => -1, term!(factor: 42, exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, SUBJECT_1 => -2, term!(factor: 42, exponent: -2));
        test_pow!(validate_2_to_minus_2, SUBJECT_2 => -2, term!(factor: 42, exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, SUBJECT_MINUS_1 => -2, term!(factor: 42, exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, SUBJECT_MINUS_2 => -2, term!(factor: 42, exponent: 4));
    }

    // ╭──────────────────────────╮
    // │ FactorExponentAnnotation │
    // ╰──────────────────────────╯
    mod factor_exponent_annotation {
        use crate::term::FactorExponentAnnotation;

        use super::*;

        fn subject_1() -> Term {
            Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor: 42,
                exponent: 1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_2() -> Term {
            Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor: 42,
                exponent: 2,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_1() -> Term {
            Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor: 42,
                exponent: -1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_2() -> Term {
            Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor: 42,
                exponent: -2,
                annotation: Annotation::from("meow"),
            })
        }

        // .pow(0)
        test_pow!(validate_1_to_0, subject_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_2_to_0, subject_2() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_1_to_0, subject_minus_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_2_to_0, subject_minus_2() => 0, term!(annotation: "meow"));

        // .pow(1)
        test_pow!(validate_1_to_1, subject_1() => 1, term!(factor: 42, annotation: "meow"));
        test_pow!(validate_2_to_1, subject_2() => 1, subject_2());
        test_pow!(validate_minus_1_to_1, subject_minus_1() => 1, subject_minus_1());
        test_pow!(validate_minus_2_to_1, subject_minus_2() => 1, subject_minus_2());

        // .pow(2)
        test_pow!(validate_1_to_2, subject_1() => 2, term!(factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_2_to_2, subject_2() => 2, term!(factor: 42, annotation: "meow", exponent: 4));
        test_pow!(validate_minus_1_to_2, subject_minus_1() => 2, term!(factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_2_to_2, subject_minus_2() => 2, term!(factor: 42, annotation: "meow", exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, subject_1() => -1, term!(factor: 42, annotation: "meow", exponent: -1));
        test_pow!(validate_2_to_minus_1, subject_2() => -1, term!(factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, subject_minus_1() => -1, term!(factor: 42, annotation: "meow"));
        test_pow!(validate_minus_2_to_minus_1, subject_minus_2() => -1, term!(factor: 42, annotation: "meow", exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, subject_1() => -2, term!(factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_2_to_minus_2, subject_2() => -2, term!(factor: 42, annotation: "meow", exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, subject_minus_1() => -2, term!(factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, subject_minus_2() => -2, term!(factor: 42, annotation: "meow", exponent: 4));
    }

    // ╭────────────╮
    // │ FactorAtom │
    // ╰────────────╯
    mod factor_atom {
        use crate::term::FactorAtom;

        use super::*;

        const SUBJECT: Term = Term::FactorAtom(FactorAtom {
            factor: 42,
            atom: Atom::Meter,
        });

        test_pow!(validate_to_0, SUBJECT => 0, Term::Factor(1));
        test_pow!(validate_to_1, SUBJECT => 1, SUBJECT);
        test_pow!(validate_to_2, SUBJECT => 2, term!(Meter, factor: 42, exponent: 2));
        test_pow!(validate_to_minus_1, SUBJECT => -1, term!(Meter, factor: 42, exponent: -1));
        test_pow!(validate_to_minus_2, SUBJECT => -2, term!(Meter, factor: 42, exponent: -2));
    }

    // ╭──────────────────────╮
    // │ FactorAtomAnnotation │
    // ╰──────────────────────╯
    mod factor_atom_annotation {
        use crate::term::FactorAtomAnnotation;

        use super::*;

        fn subject() -> Term {
            Term::FactorAtomAnnotation(FactorAtomAnnotation {
                factor: 42,
                atom: Atom::Meter,
                annotation: Annotation::from("meow"),
            })
        }

        test_pow!(validate_to_0, subject() => 0, term!(annotation: "meow"));
        test_pow!(validate_to_1, subject() => 1, subject());
        test_pow!(validate_to_2, subject() => 2, term!(Meter, factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_to_minus_1, subject() => -1, term!(Meter, factor: 42, annotation: "meow", exponent: -1));
        test_pow!(validate_to_minus_2, subject() => -2, term!(Meter, factor: 42, annotation: "meow", exponent: -2));
    }

    // ╭────────────────────╮
    // │ FactorAtomExponent │
    // ╰────────────────────╯
    mod factor_atom_exponent {
        use crate::term::FactorAtomExponent;

        use super::*;

        const SUBJECT_1: Term = Term::FactorAtomExponent(FactorAtomExponent {
            factor: 42,
            atom: Atom::Meter,
            exponent: 1,
        });
        const SUBJECT_2: Term = Term::FactorAtomExponent(FactorAtomExponent {
            factor: 42,
            atom: Atom::Meter,
            exponent: 2,
        });
        const SUBJECT_MINUS_1: Term = Term::FactorAtomExponent(FactorAtomExponent {
            factor: 42,
            atom: Atom::Meter,
            exponent: -1,
        });
        const SUBJECT_MINUS_2: Term = Term::FactorAtomExponent(FactorAtomExponent {
            factor: 42,
            atom: Atom::Meter,
            exponent: -2,
        });

        // .pow(0)
        test_pow!(validate_1_to_0, SUBJECT_1 => 0, Term::Factor(1));
        test_pow!(validate_2_to_0, SUBJECT_2 => 0, Term::Factor(1));
        test_pow!(validate_minus_1_to_0, SUBJECT_MINUS_1 => 0, Term::Factor(1));
        test_pow!(validate_minus_2_to_0, SUBJECT_MINUS_2 => 0, Term::Factor(1));

        // .pow(1)
        test_pow!(validate_1_to_1, SUBJECT_1 => 1, term!(Meter, factor: 42));
        test_pow!(validate_2_to_1, SUBJECT_2 => 1, SUBJECT_2);
        test_pow!(validate_minus_1_to_1, SUBJECT_MINUS_1 => 1, SUBJECT_MINUS_1);
        test_pow!(validate_minus_2_to_1, SUBJECT_MINUS_2 => 1, SUBJECT_MINUS_2);

        // .pow(2)
        test_pow!(validate_1_to_2, SUBJECT_1 => 2, term!(Meter, factor: 42, exponent: 2));
        test_pow!(validate_2_to_2, SUBJECT_2 => 2, term!(Meter, factor: 42, exponent: 4));
        test_pow!(validate_minus_1_to_2, SUBJECT_MINUS_1 => 2, term!(Meter, factor: 42, exponent: -2));
        test_pow!(validate_minus_2_to_2, SUBJECT_MINUS_2 => 2, term!(Meter, factor: 42, exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, SUBJECT_1 => -1, term!(Meter, factor: 42, exponent: -1));
        test_pow!(validate_2_to_minus_1, SUBJECT_2 => -1, term!(Meter, factor: 42, exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, SUBJECT_MINUS_1 => -1, term!(Meter, factor: 42, exponent: 1));
        test_pow!(validate_minus_2_to_minus_1, SUBJECT_MINUS_2 => -1, term!(Meter, factor: 42, exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, SUBJECT_1 => -2, term!(Meter, factor: 42, exponent: -2));
        test_pow!(validate_2_to_minus_2, SUBJECT_2 => -2, term!(Meter, factor: 42, exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, SUBJECT_MINUS_1 => -2, term!(Meter, factor: 42, exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, SUBJECT_MINUS_2 => -2, term!(Meter, factor: 42, exponent: 4));
    }

    // ╭──────────────────────────────╮
    // │ FactorAtomExponentAnnotation │
    // ╰──────────────────────────────╯
    mod factor_atom_exponent_annotation {
        use crate::term::FactorAtomExponentAnnotation;

        use super::*;

        fn subject_1() -> Term {
            Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                factor: 42,
                atom: Atom::Meter,
                exponent: 1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_2() -> Term {
            Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                factor: 42,
                atom: Atom::Meter,
                exponent: 2,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_1() -> Term {
            Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                factor: 42,
                atom: Atom::Meter,
                exponent: -1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_2() -> Term {
            Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                factor: 42,
                atom: Atom::Meter,
                exponent: -2,
                annotation: Annotation::from("meow"),
            })
        }

        // .pow(0)
        test_pow!(validate_1_to_0, subject_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_2_to_0, subject_2() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_1_to_0, subject_minus_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_2_to_0, subject_minus_2() => 0, term!(annotation: "meow"));

        // .pow(1)
        test_pow!(validate_1_to_1, subject_1() => 1, term!(Meter, factor: 42, annotation: "meow"));
        test_pow!(validate_2_to_1, subject_2() => 1, subject_2());
        test_pow!(validate_minus_1_to_1, subject_minus_1() => 1, subject_minus_1());
        test_pow!(validate_minus_2_to_1, subject_minus_2() => 1, subject_minus_2());

        // .pow(2)
        test_pow!(validate_1_to_2, subject_1() => 2, term!(Meter, factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_2_to_2, subject_2() => 2, term!(Meter, factor: 42, annotation: "meow", exponent: 4));
        test_pow!(validate_minus_1_to_2, subject_minus_1() => 2, term!(Meter, factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_2_to_2, subject_minus_2() => 2, term!(Meter, factor: 42, annotation: "meow", exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, subject_1() => -1, term!(Meter, factor: 42, annotation: "meow", exponent: -1));
        test_pow!(validate_2_to_minus_1, subject_2() => -1, term!(Meter, factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, subject_minus_1() => -1, term!(Meter, factor: 42, annotation: "meow"));
        test_pow!(validate_minus_2_to_minus_1, subject_minus_2() => -1, term!(Meter, factor: 42, annotation: "meow", exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, subject_1() => -2, term!(Meter, factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_2_to_minus_2, subject_2() => -2, term!(Meter, factor: 42, annotation: "meow", exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, subject_minus_1() => -2, term!(Meter, factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, subject_minus_2() => -2, term!(Meter, factor: 42, annotation: "meow", exponent: 4));
    }

    // ╭──────────────────╮
    // │ FactorPrefixAtom │
    // ╰──────────────────╯
    mod factor_prefix_atom {
        use crate::term::FactorPrefixAtom;

        use super::*;

        const SUBJECT: Term = Term::FactorPrefixAtom(FactorPrefixAtom {
            factor: 42,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
        });

        test_pow!(validate_to_0, SUBJECT => 0, Term::Factor(1));
        test_pow!(validate_to_1, SUBJECT => 1, SUBJECT);
        test_pow!(validate_to_2, SUBJECT => 2, term!(Kilo, Meter, factor: 42, exponent: 2));
        test_pow!(validate_to_minus_1, SUBJECT => -1, term!(Kilo, Meter, factor: 42, exponent: -1));
        test_pow!(validate_to_minus_2, SUBJECT => -2, term!(Kilo, Meter, factor: 42, exponent: -2));
    }

    // ╭────────────────────────────╮
    // │ FactorPrefixAtomAnnotation │
    // ╰────────────────────────────╯
    mod factor_prefix_atom_annotation {
        use crate::term::FactorPrefixAtomAnnotation;

        use super::*;

        fn subject() -> Term {
            Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
                factor: 42,
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                annotation: Annotation::from("meow"),
            })
        }

        test_pow!(validate_to_0, subject() => 0, term!(annotation: "meow"));
        test_pow!(validate_to_1, subject() => 1, subject());
        test_pow!(validate_to_2, subject() => 2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_to_minus_1, subject() => -1, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -1));
        test_pow!(validate_to_minus_2, subject() => -2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -2));
    }

    // ╭──────────────────────────╮
    // │ FactorPrefixAtomExponent │
    // ╰──────────────────────────╯
    mod factor_prefix_atom_exponent {
        use crate::term::FactorPrefixAtomExponent;

        use super::*;

        const SUBJECT_1: Term = Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
            factor: 42,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: 1,
        });
        const SUBJECT_2: Term = Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
            factor: 42,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: 2,
        });
        const SUBJECT_MINUS_1: Term = Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
            factor: 42,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -1,
        });
        const SUBJECT_MINUS_2: Term = Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
            factor: 42,
            prefix: Prefix::Kilo,
            atom: Atom::Meter,
            exponent: -2,
        });

        // .pow(0)
        test_pow!(validate_1_to_0, SUBJECT_1 => 0, Term::Factor(1));
        test_pow!(validate_2_to_0, SUBJECT_2 => 0, Term::Factor(1));
        test_pow!(validate_minus_1_to_0, SUBJECT_MINUS_1 => 0, Term::Factor(1));
        test_pow!(validate_minus_2_to_0, SUBJECT_MINUS_2 => 0, Term::Factor(1));

        // .pow(1)
        test_pow!(validate_1_to_1, SUBJECT_1 => 1, term!(Kilo, Meter, factor: 42));
        test_pow!(validate_2_to_1, SUBJECT_2 => 1, SUBJECT_2);
        test_pow!(validate_minus_1_to_1, SUBJECT_MINUS_1 => 1, SUBJECT_MINUS_1);
        test_pow!(validate_minus_2_to_1, SUBJECT_MINUS_2 => 1, SUBJECT_MINUS_2);

        // .pow(2)
        test_pow!(validate_1_to_2, SUBJECT_1 => 2, term!(Kilo, Meter, factor: 42, exponent: 2));
        test_pow!(validate_2_to_2, SUBJECT_2 => 2, term!(Kilo, Meter, factor: 42, exponent: 4));
        test_pow!(validate_minus_1_to_2, SUBJECT_MINUS_1 => 2, term!(Kilo, Meter, factor: 42, exponent: -2));
        test_pow!(validate_minus_2_to_2, SUBJECT_MINUS_2 => 2, term!(Kilo, Meter, factor: 42, exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, SUBJECT_1 => -1, term!(Kilo, Meter, factor: 42, exponent: -1));
        test_pow!(validate_2_to_minus_1, SUBJECT_2 => -1, term!(Kilo, Meter, factor: 42, exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, SUBJECT_MINUS_1 => -1, term!(Kilo, Meter, factor: 42, exponent: 1));
        test_pow!(validate_minus_2_to_minus_1, SUBJECT_MINUS_2 => -1, term!(Kilo, Meter, factor: 42, exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, SUBJECT_1 => -2, term!(Kilo, Meter, factor: 42, exponent: -2));
        test_pow!(validate_2_to_minus_2, SUBJECT_2 => -2, term!(Kilo, Meter, factor: 42, exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, SUBJECT_MINUS_1 => -2, term!(Kilo, Meter, factor: 42, exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, SUBJECT_MINUS_2 => -2, term!(Kilo, Meter, factor: 42, exponent: 4));
    }

    // ╭────────────────────────────────────╮
    // │ FactorPrefixAtomExponentAnnotation │
    // ╰────────────────────────────────────╯
    mod factor_prefix_atom_exponent_annotation {
        use crate::term::FactorPrefixAtomExponentAnnotation;

        use super::*;

        fn subject_1() -> Term {
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor: 42,
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: 1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_2() -> Term {
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor: 42,
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: 2,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_1() -> Term {
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor: 42,
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: -1,
                annotation: Annotation::from("meow"),
            })
        }

        fn subject_minus_2() -> Term {
            Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor: 42,
                prefix: Prefix::Kilo,
                atom: Atom::Meter,
                exponent: -2,
                annotation: Annotation::from("meow"),
            })
        }

        // .pow(0)
        test_pow!(validate_1_to_0, subject_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_2_to_0, subject_2() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_1_to_0, subject_minus_1() => 0, term!(annotation: "meow"));
        test_pow!(validate_minus_2_to_0, subject_minus_2() => 0, term!(annotation: "meow"));

        // .pow(1)
        test_pow!(validate_1_to_1, subject_1() => 1, term!(Kilo, Meter, factor: 42, annotation: "meow"));
        test_pow!(validate_2_to_1, subject_2() => 1, subject_2());
        test_pow!(validate_minus_1_to_1, subject_minus_1() => 1, subject_minus_1());
        test_pow!(validate_minus_2_to_1, subject_minus_2() => 1, subject_minus_2());

        // .pow(2)
        test_pow!(validate_1_to_2, subject_1() => 2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_2_to_2, subject_2() => 2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: 4));
        test_pow!(validate_minus_1_to_2, subject_minus_1() => 2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_2_to_2, subject_minus_2() => 2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -4));

        // .pow(-1)
        test_pow!(validate_1_to_minus_1, subject_1() => -1, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -1));
        test_pow!(validate_2_to_minus_1, subject_2() => -1, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_minus_1_to_minus_1, subject_minus_1() => -1, term!(Kilo, Meter, factor: 42, annotation: "meow"));
        test_pow!(validate_minus_2_to_minus_1, subject_minus_2() => -1, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: 2));

        // .pow(-2)
        test_pow!(validate_1_to_minus_2, subject_1() => -2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -2));
        test_pow!(validate_2_to_minus_2, subject_2() => -2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: -4));
        test_pow!(validate_minus_1_to_minus_2, subject_minus_1() => -2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: 2));
        test_pow!(validate_minus_2_to_minus_2, subject_minus_2() => -2, term!(Kilo, Meter, factor: 42, annotation: "meow", exponent: 4));
    }
}
