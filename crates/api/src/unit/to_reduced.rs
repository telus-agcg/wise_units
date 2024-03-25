use crate::{as_fraction::AsFraction, reduce::ToReduced, Composable, Composition, Term};

use super::Unit;

type OptionCombos = Vec<Option<(Term, Composition)>>;

/// The implementation here checks for `Unit`s in `self`'s numerator and denominator that have the
/// same `Composition` (aka "dimension"). For example, if `self` is effectively
/// `"[lb_av].[acr_us]/har"`, `"[acr_us]"` and `"har"` both have the dimension "L2", so they can be
/// canceled out. This, of course, has implications for when a value is involved (in a
/// `Measurement`) since the value using this newly reduced `Unit` can be different (i.e. it would
/// _not_ be different if, for example, the `Unit` was something like `"m4/m2"`, where the scalar
/// representation of the `Unit` is still the same after the reduction).
///
impl ToReduced for Unit {
    type Output = Self;

    /// Reduces `self`'s `Term`s into a new `Unit`, consuming `self`.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use wise_units::reduce::ToReduced;
    /// use wise_units::Unit;
    ///
    /// // "m2" doesn't reduce down...
    /// let m1 = Unit::from_str("m2").unwrap();
    /// let m2 = Unit::from_str("m2").unwrap();
    /// assert_eq!(m1.to_reduced(), m2);
    ///
    /// // ...but "m4/m2" does.
    /// let m1 = Unit::from_str("m4/m2").unwrap();
    /// let m2 = Unit::from_str("m2").unwrap();
    /// assert_eq!(m1.to_reduced(), m2);
    ///
    /// // This also works for Units of the same dimension, but with different scalar
    /// // representations.
    /// let m1 = Unit::from_str("g.m2/har").unwrap();
    /// let m2 = Unit::from_str("g").unwrap();
    /// assert_eq!(m1.to_reduced(), m2);
    ///
    /// // It does not, however, work for reducing when units' dimensions are similar in magnitude.
    /// // This does not reduce down to "g.m2":
    /// let m1 = Unit::from_str("g.m4/har").unwrap();
    /// let m2 = Unit::from_str("g.m4/har").unwrap();
    /// assert_eq!(m1.to_reduced(), m2);
    /// ```
    ///
    #[inline]
    fn to_reduced(&self) -> Self::Output {
        match self.as_fraction() {
            (Some(numerator), Some(denominator)) => {
                let (new_numerators, new_denominators) =
                    build_unit_parts(&numerator.terms, &denominator.terms);

                let mut new_terms: Vec<Term> =
                    Vec::with_capacity(new_numerators.len() + new_denominators.len());
                new_terms.extend_from_slice(&new_numerators);

                let new_denominators =
                    crate::term::num_traits::inv::inv_terms_into(new_denominators);
                new_terms.extend_from_slice(&new_denominators);

                Self::new(super::term_reducing::reduce_terms(&new_terms))
            }
            (_, _) => Self::new(super::term_reducing::reduce_terms(&self.terms)),
        }
    }
}

/// Iterates through `numerator_terms` and `denominator_terms`, comparing each `Term`s
/// `Composition`. If the `Composition` from the numerator matches one in the denominator, the
/// `Term`s are excluded from the resulting sets of `Term`s.
///
fn build_unit_parts(
    numerator_terms: &[Term],
    denominator_terms: &[Term],
) -> (Vec<Term>, Vec<Term>) {
    let mut new_numerator_combos = build_terms_and_compositions(numerator_terms);
    let mut new_denominator_combos = build_terms_and_compositions(denominator_terms);

    'outer: for numerator_combo in new_numerator_combos.iter_mut().rev() {
        'inner: for new_denominator_combo in &mut new_denominator_combos {
            match (&numerator_combo, &new_denominator_combo) {
                (Some(nc), Some(dc)) => {
                    if nc.1 == dc.1 {
                        *numerator_combo = None;
                        *new_denominator_combo = None;
                        continue 'outer;
                    }
                }
                (None, Some(_)) => continue 'outer,
                (_, None) => continue 'inner,
            }
        }
    }

    (
        filter_results(new_numerator_combos),
        filter_results(new_denominator_combos),
    )
}

/// Takes each `Term` and builds an `Option<(Term, Composition)>`, where each will be used to
/// compare against each numerator/denominator `Term`'s `Composition`. They're wrapped in an
/// `Option` so that when one needs to be removed from the result set, it can just be set to a
/// `None`.
///
fn build_terms_and_compositions(terms: &[Term]) -> OptionCombos {
    terms
        .iter()
        .map(|term| Some((term.clone(), term.composition())))
        .collect()
}

/// Builds a set of `Terms` from the reduced `option_combos`.
///
fn filter_results(option_combos: OptionCombos) -> Vec<Term> {
    option_combos
        .into_iter()
        .filter_map(|combo| combo.map(|c| c.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_reduced {
        use super::*;
        use std::str::FromStr;

        macro_rules! validate_reduction {
            ($test_name:ident, $input:expr, $expected:expr) => {
                #[test]
                fn $test_name() {
                    let unit = Unit::from_str($input).unwrap();
                    let actual = unit.to_reduced();
                    let expected = Unit::from_str($expected).unwrap();

                    assert_eq!(&actual, &expected);
                    assert_eq!(actual.expression(), expected.expression());
                }
            };
        }

        validate_reduction!(validate_m, "m", "m");
        validate_reduction!(validate_m2_per_m, "m2/m", "m");
        validate_reduction!(validate_100m2_per_m, "100m2/m", "100m2/m");
        validate_reduction!(validate_m2_dot_m2, "m2.m2", "m4");
        validate_reduction!(validate_m2_dot_m2_per_s_dot_s, "m2.m2/s.s", "m4/s2");
        validate_reduction!(validate_m2_dot_s_per_s_dot_m2, "m2.s/s.m2", "1");

        validate_reduction!(validate_lb_av_dot_har_per_m2, "[lb_av].har/m2", "[lb_av]");
        validate_reduction!(validate_lb_av_dot_har_per_m2_dot_g, "[lb_av].har/m2.g", "1");

        validate_reduction!(
            validate_acr_us_per_m2_per_har,
            "[acr_us]/m2/har",
            "[acr_us]"
        );
        validate_reduction!(
            validate_acr_us_per_m2_per_har_per_sft_i,
            "[acr_us]/m2/har/[sft_i]",
            "1"
        );
    }

    mod into_reduced {
        use super::*;
        use crate::reduce::IntoReduced;
        use std::str::FromStr;

        macro_rules! validate_reduction {
            ($test_name:ident, $input:expr, $expected:expr) => {
                #[test]
                fn $test_name() {
                    let unit = Unit::from_str($input).unwrap();
                    let actual = unit.into_reduced();
                    let expected = Unit::from_str($expected).unwrap();

                    assert_eq!(&actual, &expected);
                    assert_eq!(actual.expression(), expected.expression());
                }
            };
        }

        validate_reduction!(validate_m, "m", "m");
        validate_reduction!(validate_m2_per_m, "m2/m", "m");
        validate_reduction!(validate_100m2_per_m, "100m2/m", "100m2/m");
        validate_reduction!(validate_m2_dot_m2, "m2.m2", "m4");
        validate_reduction!(validate_m2_dot_m2_per_s_dot_s, "m2.m2/s.s", "m4/s2");
        validate_reduction!(validate_m2_dot_s_per_s_dot_m2, "m2.s/s.m2", "1");

        validate_reduction!(validate_lb_av_dot_har_per_m2, "[lb_av].har/m2", "[lb_av]");
        validate_reduction!(validate_lb_av_dot_har_per_m2_dot_g, "[lb_av].har/m2.g", "1");

        validate_reduction!(
            validate_acr_us_per_m2_per_har,
            "[acr_us]/m2/har",
            "[acr_us]"
        );
        validate_reduction!(
            validate_acr_us_per_m2_per_har_per_sft_i,
            "[acr_us]/m2/har/[sft_i]",
            "1"
        );
    }
}
