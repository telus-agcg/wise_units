use std::borrow::Cow;

use num_traits::Inv;

use crate::{reduce::ToReduced, Composable, Term};

use super::Unit;

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
        let mut terms: Vec<_> = self.terms.iter().map(|t| Some(Cow::Borrowed(t))).collect();

        'outer: for (i, outer_term) in self.terms.iter().enumerate() {
            if terms[i].is_none() {
                continue 'outer;
            }

            let next_outer = i + 1;
            let remainder = &self.terms[next_outer..];

            'inner: for (j, inner_term) in remainder.iter().enumerate() {
                let terms_offset = next_outer + j;

                if terms[terms_offset].is_none() {
                    continue 'inner;
                }

                if outer_term.composition() == inner_term.composition().inv() {
                    terms[i] = None;
                    terms[terms_offset] = None;
                    continue 'outer;
                }

                if outer_term.atom() == inner_term.atom()
                    && outer_term.factor() == inner_term.factor()
                    && outer_term.annotation() == inner_term.annotation()
                {
                    let mut new_term = outer_term.clone();

                    match (outer_term.exponent(), inner_term.exponent()) {
                        (None, None) => (),
                        (None, Some(rhs)) => {
                            new_term = new_term.assign_exponent(rhs);
                        }
                        (Some(lhs), None) => {
                            new_term = new_term.assign_exponent(lhs);
                        }
                        (Some(lhs), Some(rhs)) => {
                            let x = lhs + rhs;

                            if x == 0 || x == 1 {
                            } else {
                                new_term = new_term.assign_exponent(x);
                            }
                        }
                    };
                    terms[i] = Some(Cow::Owned(new_term));
                    terms[next_outer] = None;
                    continue 'outer;
                }
            }
        }

        let terms: Vec<Term> = terms.into_iter().flatten().map(Cow::into_owned).collect();
        Self::new(terms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_reduced {
        use super::*;
        use std::str::FromStr;

        macro_rules! validate_reduction {
            ($test_name:ident, $input:expr, expected: $expected:expr) => {
                #[test]
                fn $test_name() {
                    let unit = Unit::from_str($input).unwrap();
                    let actual = unit.to_reduced();
                    let expected = Unit::from_str($expected).unwrap();

                    assert_eq!(actual.expression(), expected.expression());
                    assert_eq!(&actual, &expected);
                }
            };
        }

        validate_reduction!(validate_m, "m", expected: "m");
        validate_reduction!(validate_m2_per_m, "m2/m", expected: "m");
        validate_reduction!(validate_100m2_per_m, "100m2/m", expected: "100m2/m");
        validate_reduction!(validate_m2_dot_m2, "m2.m2", expected: "m4");
        validate_reduction!(validate_m2_dot_m2_per_s_dot_s, "m2.m2/s.s", expected: "m4/s2");
        validate_reduction!(validate_m2_dot_s_per_s_dot_m2, "m2.s/s.m2", expected: "1");

        validate_reduction!(validate_lb_av_dot_har_per_m2, "[lb_av].har/m2", expected: "[lb_av]");
        validate_reduction!(validate_lb_av_dot_har_per_m2_dot_g, "[lb_av].har/m2.g", expected: "1");

        validate_reduction!(
            validate_acr_us_per_m2_per_har,
            "[acr_us]/m2/har",
            expected: "har"
        );
        validate_reduction!(
            validate_acr_us_per_m2_per_har_per_sft_i,
            "[acr_us]/m2/har/[sft_i]",
            expected: "1"
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

        validate_reduction!(validate_acr_us_per_m2_per_har, "[acr_us]/m2/har", "har");
        validate_reduction!(
            validate_acr_us_per_m2_per_har_per_sft_i,
            "[acr_us]/m2/har/[sft_i]",
            "1"
        );
    }
}
