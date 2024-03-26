#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

pub(super) mod consts;

use std::{borrow::Cow, str::FromStr};

use num_traits::One;

use crate::{reducible::Reducible, term, Term};

use super::function_set::FunctionSet;

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an `"[in_i]"` has a
/// `Definition` of 2.54 cm).
///
pub(crate) enum Definition<V> {
    Base,
    Value(V),
    ValueSpecial {
        value: V,
        function_set: FunctionSet<V>,
    },
    ValueTerms {
        value: V,
        terms: Cow<'static, [Term]>,
    },
    ValueTermsSpecial {
        value: V,
        terms: Cow<'static, [Term]>,
        function_set: FunctionSet<V>,
    },
}

impl<V> Definition<V> {
    pub(crate) fn new_value_terms(value: V, expression: &'static str) -> Self {
        Self::ValueTerms {
            value,
            terms: crate::Unit::from_str(expression).map_or_else(
                |_| unreachable!("expected valid unit definition string: {expression}"),
                crate::Unit::into_terms,
            ),
        }
    }

    pub(crate) fn new_value_terms_special(
        value: V,
        expression: &'static str,
        function_set: FunctionSet<V>,
    ) -> Self {
        Self::ValueTermsSpecial {
            value,
            terms: crate::Unit::from_str(expression).map_or_else(
                |_| unreachable!("expected valid unit definition string: {expression}"),
                crate::Unit::into_terms,
            ),
            function_set,
        }
    }

    pub(crate) fn value(&self) -> V
    where
        V: One + Clone,
    {
        match self {
            Self::Base => <V as One>::one(),
            Self::Value(value) => value.clone(),
            Self::ValueSpecial { value, .. }
            | Self::ValueTerms { value, .. }
            | Self::ValueTermsSpecial { value, .. } => (*value).clone(),
        }
    }

    pub(crate) const fn terms(&self) -> &Cow<'static, [Term]> {
        match self {
            Self::Value(_) | Self::ValueSpecial { .. } | Self::Base => {
                &Cow::Borrowed(term::UNITY_ARRAY_REF)
            }
            Self::ValueTerms { terms, .. } | Self::ValueTermsSpecial { terms, .. } => terms,
        }
    }
}

impl Reducible<f64> for Definition<f64> {
    fn reduce_value(&self, other_value: f64) -> f64 {
        match self {
            Self::Base => One::one(),
            Self::Value(value) => *value,
            Self::ValueTerms { value, terms } => value * terms.reduce_value(other_value),
            Self::ValueSpecial { function_set, .. }
            | Self::ValueTermsSpecial { function_set, .. } => {
                (function_set.convert_to)(other_value)
            }
        }
    }

    fn calculate_magnitude(&self, other_value: f64) -> f64 {
        match self {
            Self::Base => One::one(),
            Self::Value(value) => *value,
            Self::ValueTerms { value, terms } => value * terms.calculate_magnitude(other_value),
            Self::ValueSpecial { function_set, .. }
            | Self::ValueTermsSpecial { function_set, .. } => {
                (function_set.convert_from)(other_value)
            }
        }
    }
}

