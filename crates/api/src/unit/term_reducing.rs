use std::collections::BTreeMap;

use crate::{
    term::{variants::*, Exponent, Factor, UNITY},
    Annotation, Atom, Prefix, Term,
};

/// Internal enum used for reducing `Term`s.
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ComposableTerm<'a> {
    Annotation(&'a str),
    Atom(Atom),
    AtomAnnotation {
        atom: Atom,
        annotation: &'a str,
    },
    PrefixAtom {
        prefix: Prefix,
        atom: Atom,
    },
    PrefixAtomAnnotation {
        prefix: Prefix,
        atom: Atom,
        annotation: &'a str,
    },
    Factor(Factor),
    FactorAnnotation {
        factor: Factor,
        annotation: &'a str,
    },
    FactorAtom {
        factor: Factor,
        atom: Atom,
    },
    FactorAtomAnnotation {
        factor: Factor,
        atom: Atom,
        annotation: &'a str,
    },
    FactorPrefixAtom {
        factor: Factor,
        prefix: Prefix,
        atom: Atom,
    },
    FactorPrefixAtomAnnotation {
        factor: Factor,
        prefix: Prefix,
        atom: Atom,
        annotation: &'a str,
    },
}

impl<'a> From<&'a Term> for ComposableTerm<'a> {
    fn from(term: &'a Term) -> Self {
        match term {
            Term::Annotation(annotation) => Self::Annotation(annotation.as_str()),
            Term::Atom(atom) | Term::AtomExponent(AtomExponent { atom, .. }) => Self::Atom(*atom),
            Term::AtomAnnotation(AtomAnnotation { atom, annotation })
            | Term::AtomExponentAnnotation(AtomExponentAnnotation {
                atom, annotation, ..
            }) => Self::AtomAnnotation {
                atom: *atom,
                annotation: annotation.as_str(),
            },
            Term::PrefixAtom(PrefixAtom { prefix, atom })
            | Term::PrefixAtomExponent(PrefixAtomExponent { prefix, atom, .. }) => {
                Self::PrefixAtom {
                    prefix: *prefix,
                    atom: *atom,
                }
            }
            Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
                prefix,
                atom,
                annotation,
            })
            | Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix,
                atom,
                annotation,
                ..
            }) => Self::PrefixAtomAnnotation {
                prefix: *prefix,
                atom: *atom,
                annotation: annotation.as_str(),
            },
            Term::Factor(factor) | Term::FactorExponent(FactorExponent { factor, .. }) => {
                Self::Factor(*factor)
            }
            Term::FactorAnnotation(FactorAnnotation { factor, annotation })
            | Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor, annotation, ..
            }) => Self::FactorAnnotation {
                factor: *factor,
                annotation: annotation.as_str(),
            },

            Term::FactorAtom(FactorAtom { factor, atom })
            | Term::FactorAtomExponent(FactorAtomExponent { factor, atom, .. }) => {
                Self::FactorAtom {
                    factor: *factor,
                    atom: *atom,
                }
            }
            Term::FactorAtomAnnotation(FactorAtomAnnotation {
                factor,
                atom,
                annotation,
            })
            | Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                factor,
                atom,
                annotation,
                ..
            }) => Self::FactorAtomAnnotation {
                factor: *factor,
                atom: *atom,
                annotation: annotation.as_str(),
            },
            Term::FactorPrefixAtom(FactorPrefixAtom {
                factor,
                prefix,
                atom,
            })
            | Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
                factor,
                prefix,
                atom,
                ..
            }) => Self::FactorPrefixAtom {
                factor: *factor,
                prefix: *prefix,
                atom: *atom,
            },
            Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
                factor,
                prefix,
                atom,
                annotation,
            })
            | Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                factor,
                prefix,
                atom,
                annotation,
                ..
            }) => Self::FactorPrefixAtomAnnotation {
                factor: *factor,
                prefix: *prefix,
                atom: *atom,
                annotation: annotation.as_str(),
            },
        }
    }
}

impl<'a> From<ComposableTerm<'a>> for Term {
    fn from(value: ComposableTerm<'a>) -> Self {
        match value {
            ComposableTerm::Annotation(annotation) => {
                Self::Annotation(Annotation::from(annotation))
            }
            ComposableTerm::Atom(atom) => Self::Atom(atom),
            ComposableTerm::AtomAnnotation { atom, annotation } => {
                Self::AtomAnnotation(AtomAnnotation {
                    atom,
                    annotation: Annotation::from(annotation),
                })
            }
            ComposableTerm::PrefixAtom { prefix, atom } => {
                Self::PrefixAtom(PrefixAtom { prefix, atom })
            }
            ComposableTerm::PrefixAtomAnnotation {
                prefix,
                atom,
                annotation,
            } => Self::PrefixAtomAnnotation(PrefixAtomAnnotation {
                prefix,
                atom,
                annotation: Annotation::from(annotation),
            }),
            ComposableTerm::Factor(factor) => Self::Factor(factor),
            ComposableTerm::FactorAnnotation { factor, annotation } => {
                Self::FactorAnnotation(FactorAnnotation {
                    factor,
                    annotation: Annotation::from(annotation),
                })
            }
            ComposableTerm::FactorAtom { factor, atom } => {
                Self::FactorAtom(FactorAtom { factor, atom })
            }
            ComposableTerm::FactorAtomAnnotation {
                factor,
                atom,
                annotation,
            } => Self::FactorAtomAnnotation(FactorAtomAnnotation {
                factor,
                atom,
                annotation: Annotation::from(annotation),
            }),
            ComposableTerm::FactorPrefixAtom {
                factor,
                prefix,
                atom,
            } => Self::FactorPrefixAtom(FactorPrefixAtom {
                factor,
                prefix,
                atom,
            }),
            ComposableTerm::FactorPrefixAtomAnnotation {
                factor,
                prefix,
                atom,
                annotation,
            } => Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
                factor,
                prefix,
                atom,
                annotation: Annotation::from(annotation),
            }),
        }
    }
}

type Parts<'a> = (ComposableTerm<'a>, Exponent);

impl<'a> From<Parts<'a>> for Term {
    fn from(parts: Parts<'a>) -> Self {
        match parts.1 {
            // For anything to the 0th power:
            // - any variant that has an annotation, keep only the annotation
            // - all other variants, return the Unity (1).
            //
            0 => match parts.0 {
                ComposableTerm::Atom(_)
                | ComposableTerm::PrefixAtom { .. }
                | ComposableTerm::Factor(_)
                | ComposableTerm::FactorAtom { .. }
                | ComposableTerm::FactorPrefixAtom { .. } => UNITY,
                ComposableTerm::Annotation(annotation)
                | ComposableTerm::AtomAnnotation { annotation, .. }
                | ComposableTerm::PrefixAtomAnnotation { annotation, .. }
                | ComposableTerm::FactorAnnotation { annotation, .. }
                | ComposableTerm::FactorAtomAnnotation { annotation, .. }
                | ComposableTerm::FactorPrefixAtomAnnotation { annotation, .. } => {
                    Self::Annotation(Annotation::from(annotation))
                }
            },

            // For anything to the 1st power, just return the `Term` analog.
            //
            1 => Self::from(parts.0),

            // Anything with a non-0 or non-1 exponent, return the related `Exponent` variant of
            // `Term`, except:
            // - `Annotation`: can't be combined with an exponent
            // - `Factor`: can't be combined with an exponent
            // - `FactorAnnotation`: can't be combined with an exponent
            //
            exponent => match parts.0 {
                ComposableTerm::Annotation(annotation) => {
                    Self::Annotation(Annotation::from(annotation))
                }
                ComposableTerm::Atom(atom) => Self::AtomExponent(AtomExponent { atom, exponent }),
                ComposableTerm::AtomAnnotation { atom, annotation } => {
                    Self::AtomExponentAnnotation(AtomExponentAnnotation {
                        atom,
                        exponent,
                        annotation: Annotation::from(annotation),
                    })
                }
                ComposableTerm::PrefixAtom { prefix, atom } => {
                    Self::PrefixAtomExponent(PrefixAtomExponent {
                        prefix,
                        atom,
                        exponent,
                    })
                }
                ComposableTerm::PrefixAtomAnnotation {
                    prefix,
                    atom,
                    annotation,
                } => Self::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                    prefix,
                    atom,
                    exponent,
                    annotation: Annotation::from(annotation),
                }),
                ComposableTerm::Factor(factor) => Self::Factor(factor),
                ComposableTerm::FactorAnnotation { factor, annotation } => {
                    Self::FactorAnnotation(FactorAnnotation {
                        factor,
                        annotation: Annotation::from(annotation),
                    })
                }
                ComposableTerm::FactorAtom { factor, atom } => {
                    Self::FactorAtomExponent(FactorAtomExponent {
                        factor,
                        atom,
                        exponent,
                    })
                }
                ComposableTerm::FactorAtomAnnotation {
                    factor,
                    atom,
                    annotation,
                } => Self::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                    factor,
                    atom,
                    exponent,
                    annotation: Annotation::from(annotation),
                }),
                ComposableTerm::FactorPrefixAtom {
                    factor,
                    prefix,
                    atom,
                } => Self::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
                    factor,
                    prefix,
                    atom,
                    exponent,
                }),
                ComposableTerm::FactorPrefixAtomAnnotation {
                    factor,
                    prefix,
                    atom,
                    annotation,
                } => Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                    factor,
                    prefix,
                    atom,
                    exponent,
                    annotation: Annotation::from(annotation),
                }),
            },
        }
    }
}

/// Function used in `Unit` for reducing its `Term`s.
///
pub(super) fn reduce_terms(terms: &[Term]) -> Vec<Term> {
    let map = reduce_to_map(terms);

    // If everything is reduced away, the effective Unit should be "1".
    if map.is_empty() {
        vec![UNITY]
    } else {
        // Reconstructs the map into the Vec<Term>.
        map.into_iter().map(Term::from).collect()
    }
}

/// Iterates through `terms`, finds `Term`s that have the same attributes that determine
/// uniqueness (`atom`, `prefix`, `factor`), and sums those exponents. This is the destructuring
/// part of `reduce_terms()`.
///
fn reduce_to_map(terms: &[Term]) -> BTreeMap<ComposableTerm<'_>, Exponent> {
    let mut map = BTreeMap::new();

    for term in terms {
        let composable_term = ComposableTerm::from(term);

        let _ = map
            .entry(composable_term)
            .and_modify(|exponent| *exponent += term.effective_exponent())
            .or_insert_with(|| term.effective_exponent());
    }

    map
}
