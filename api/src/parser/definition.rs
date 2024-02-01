#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

mod reducible;

#[cfg(feature = "v2")]
mod v2;

use std::borrow::Cow;

use num_traits::One;

use crate::parser::{function_set::FunctionSet, Error, Term};

use super::term;

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an `"[in_i]"` has a
/// `Definition` of 2.54 cm).
///
#[derive(Debug)]
pub(crate) enum Definition<V>
where
    V: One + PartialEq,
{
    Base,
    NonDimensional(V),
    NonDimensionalSpecial {
        value: V,
        function_set: FunctionSet,
    },
    Dimensional {
        value: V,
        terms: Cow<'static, [Term]>,
    },
    DimensionalSpecial {
        value: V,
        terms: Cow<'static, [Term]>,
        function_set: FunctionSet,
    },
}

impl<V> Definition<V>
where
    V: One + PartialEq + Clone,
{
    pub(crate) fn try_new_dimensional(value: V, expression: &'static str) -> Result<Self, Error> {
        Ok(Self::Dimensional {
            value,
            terms: Cow::Owned(super::parse(expression)?),
        })
    }

    pub(crate) fn try_new_dimensional_special(
        value: V,
        expression: &'static str,
        function_set: FunctionSet,
    ) -> Result<Self, Error> {
        Ok(Self::DimensionalSpecial {
            value,
            terms: Cow::Owned(super::parse(expression)?),
            function_set,
        })
    }

    pub(crate) fn value(&self) -> V {
        match self {
            Definition::Base => <V as One>::one(),
            Definition::NonDimensional(value) => value.clone(),
            Definition::NonDimensionalSpecial { value, .. }
            | Definition::Dimensional { value, .. }
            | Definition::DimensionalSpecial { value, .. } => (*value).clone(),
        }
    }

    pub(crate) const fn terms(&self) -> &Cow<'static, [Term]> {
        match self {
            Definition::Base => &Cow::Borrowed(term::UNITY_SLICE),
            Definition::NonDimensional(_) | Definition::NonDimensionalSpecial { .. } => {
                &Cow::Borrowed(term::UNITY_SLICE)
            }
            Definition::Dimensional { terms, .. }
            | Definition::DimensionalSpecial { terms, .. } => terms,
        }
    }

    pub(crate) fn function_set(&self) -> Option<FunctionSet> {
        match self {
            Definition::Base | Definition::NonDimensional(_) | Definition::Dimensional { .. } => {
                None
            }
            Definition::NonDimensionalSpecial { function_set, .. }
            | Definition::DimensionalSpecial { function_set, .. } => Some(*function_set),
        }
    }

    // TODO: this was incorrectly _only_ checking terms; to be the unity, based
    // on our heuristics, the old implementation required `terms` to be a `[Term::new_unity()]`
    // _and_ the `value` == 1. This wasn't checking the value.
    pub(crate) fn is_unity(&self) -> bool {
        match self {
            Definition::Base => true,
            Definition::NonDimensional(value) => value.is_one(),
            Definition::NonDimensionalSpecial { value, .. } => value.is_one(),
            Definition::DimensionalSpecial { value, terms, .. } => {
                value.is_one() && terms.len() == 1 && terms[0].is_unity()
            }
            Definition::Dimensional { value, terms } => {
                value.is_one() && terms.len() == 1 && terms[0].is_unity()
            }
        }
    }
}

impl<V> Default for Definition<V>
where
    V: One + PartialEq,
{
    fn default() -> Self {
        Self::NonDimensional(num_traits::One::one())
    }
}
