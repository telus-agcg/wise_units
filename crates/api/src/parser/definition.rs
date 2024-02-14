#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

use std::borrow::Cow;

use num_traits::One;

use crate::{
    parser::{function_set::FunctionSet, Error, Term},
    reducible::Reducible,
};

use super::term;

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an `"[in_i]"` has a
/// `Definition` of 2.54 cm).
///
pub(crate) enum Definition<V> {
    Base,
    NonDimensional(V),
    NonDimensionalSpecial {
        value: V,
        function_set: FunctionSet<V>,
    },
    Dimensional {
        value: V,
        terms: Cow<'static, [Term]>,
    },
    DimensionalSpecial {
        value: V,
        terms: Cow<'static, [Term]>,
        function_set: FunctionSet<V>,
    },
}

impl<V> Definition<V> {
    pub(crate) fn try_new_dimensional(value: V, expression: &'static str) -> Result<Self, Error> {
        Ok(Self::Dimensional {
            value,
            terms: Cow::Owned(super::parse(expression)?),
        })
    }

    pub(crate) fn try_new_dimensional_special(
        value: V,
        expression: &'static str,
        function_set: FunctionSet<V>,
    ) -> Result<Self, Error> {
        Ok(Self::DimensionalSpecial {
            value,
            terms: Cow::Owned(super::parse(expression)?),
            function_set,
        })
    }

    pub(crate) fn value(&self) -> V
    where
        V: One + Clone,
    {
        match self {
            Self::Base => <V as One>::one(),
            Self::NonDimensional(value) => value.clone(),
            Self::NonDimensionalSpecial { value, .. }
            | Self::Dimensional { value, .. }
            | Self::DimensionalSpecial { value, .. } => (*value).clone(),
        }
    }

    pub(crate) const fn terms(&self) -> &Cow<'static, [Term]> {
        match self {
            Self::Base => &Cow::Borrowed(term::UNITY_ARRAY_REF),
            Self::NonDimensional(_) | Self::NonDimensionalSpecial { .. } => {
                &Cow::Borrowed(term::UNITY_ARRAY_REF)
            }
            Self::Dimensional { terms, .. } | Self::DimensionalSpecial { terms, .. } => terms,
        }
    }
}

impl Reducible<f64> for Definition<f64> {
    fn reduce_value(&self, other_value: f64) -> f64 {
        match self {
            Self::Base => One::one(),
            Self::NonDimensional(value) => *value,
            Self::Dimensional { value, terms } => value * terms.reduce_value(other_value),
            Self::NonDimensionalSpecial { function_set, .. }
            | Self::DimensionalSpecial { function_set, .. } => {
                (function_set.convert_to)(other_value)
            }
        }
    }

    fn calculate_magnitude(&self, other_value: f64) -> f64 {
        match self {
            Self::Base => One::one(),
            Self::NonDimensional(value) => *value,
            Self::Dimensional { value, terms } => value * terms.calculate_magnitude(other_value),
            Self::NonDimensionalSpecial { function_set, .. }
            | Self::DimensionalSpecial { function_set, .. } => {
                (function_set.convert_from)(other_value)
            }
        }
    }
}
