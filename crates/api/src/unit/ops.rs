use std::{
    borrow::Cow,
    ops::{Div, Mul},
};

use num_traits::Inv;

use crate::Unit;

use super::term_reducing;

//          ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//          ┃                      impl Div                       ┃
//          ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// ╭───────────────╮
// │ Owned / Owned │
// ╰───────────────╯
impl Div for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        let mut s = self;

        {
            let lhs = s.terms.to_mut();
            lhs.reserve_exact(other.terms.len());
            lhs.extend(other.terms.iter().map(Inv::inv));
        }

        s.terms = term_reducing::reduce_terms(&s.terms);
        s
    }
}

// ╭──────────────────╮
// │ Owned / Borrowed │
// ╰──────────────────╯
impl<'a> Div<&'a Self> for Unit {
    type Output = Self;

    #[inline]
    fn div(self, other: &'a Self) -> Self::Output {
        let mut s = self;

        {
            let lhs = s.terms.to_mut();
            lhs.reserve_exact(other.terms.len());
            lhs.extend(other.terms.iter().map(Inv::inv));
        }

        s.terms = term_reducing::reduce_terms(&s.terms);
        s
    }
}

// ╭─────────────────────╮
// │ Borrowed / Borrowed │
// ╰─────────────────────╯
impl<'a> Div for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: &'a Unit) -> Self::Output {
        let mut lhs = self.terms.to_vec();

        lhs.reserve_exact(other.terms.len());
        lhs.extend(other.terms.iter().map(Inv::inv));

        Unit::new(term_reducing::reduce_terms(&Cow::Owned(lhs)))
    }
}

// ╭──────────────────╮
// │ Borrowed / Owned │
// ╰──────────────────╯
impl<'a> Div<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn div(self, other: Unit) -> Self::Output {
        let mut lhs = self.terms.to_vec();

        lhs.reserve_exact(other.terms.len());
        lhs.extend(other.terms.iter().map(Inv::inv));

        Unit::new(term_reducing::reduce_terms(&Cow::Owned(lhs)))
    }
}

//          ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//          ┃                        impl Mul                         ┃
//          ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
// ╭───────────────╮
// │ Owned * Owned │
// ╰───────────────╯
impl Mul for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        let mut lhs_unit = self;

        {
            let mut rhs_unit = other;
            let lhs = lhs_unit.terms.to_mut();
            lhs.reserve_exact(rhs_unit.terms.len());
            lhs.append(rhs_unit.terms.to_mut());
        }

        lhs_unit.terms = term_reducing::reduce_terms(&lhs_unit.terms);
        lhs_unit
    }
}

// ╭──────────────────╮
// │ Owned * Borrowed │
// ╰──────────────────╯
impl<'a> Mul<&'a Self> for Unit {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        let mut lhs_unit = self;

        {
            let lhs = lhs_unit.terms.to_mut();
            lhs.extend_from_slice(&other.terms);
        }

        lhs_unit.terms = term_reducing::reduce_terms(&lhs_unit.terms);
        lhs_unit
    }
}

// ╭─────────────────────╮
// │ Borrowed * Borrowed │
// ╰─────────────────────╯
impl<'a> Mul for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: &'a Unit) -> Self::Output {
        let mut lhs = self.terms.to_vec();
        lhs.extend_from_slice(&other.terms);

        Unit::new(term_reducing::reduce_terms(&Cow::Owned(lhs)))
    }
}

// ╭──────────────────╮
// │ Borrowed * Owned │
// ╰──────────────────╯
impl<'a> Mul<Unit> for &'a Unit {
    type Output = Unit;

    #[inline]
    fn mul(self, other: Unit) -> Self::Output {
        let mut lhs = self.terms.to_vec();

        {
            let mut rhs_unit = other;
            lhs.reserve_exact(rhs_unit.terms.len());
            lhs.append(rhs_unit.terms.to_mut());
        }

        Unit::new(term_reducing::reduce_terms(&Cow::Owned(lhs)))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        testing::const_units::{ACRE, KILOMETER, METER},
        unit::UNITY,
    };

    use super::*;

    fn seed() -> Unit {
        Unit::from_str("{seed}").unwrap()
    }

    mod div {
        use super::*;

        #[test]
        #[allow(clippy::eq_op)]
        fn validate_owned_div_owned() {
            // assert_eq!(METER / METER, UNITY);
            // let expected = Unit::from_str("m/km").unwrap();
            // assert_eq!(METER / KILOMETER, expected);
            // let unit = Unit::from_str("10m").unwrap();
            // let other = Unit::from_str("20m").unwrap();
            // let expected = Unit::from_str("10m/20m").unwrap();
            // assert_eq!(unit / other, expected);

            assert_eq!(seed() / seed(), UNITY);
            // assert_eq!(UNITY / seed(), Unit::from_str("/{seed}").unwrap());
            // assert_eq!(seed() / ACRE, Unit::from_str("{seed}/[acr_us]").unwrap());
        }

        #[test]
        fn validate_owned_div_borrowed() {
            assert_eq!(METER / &METER, UNITY);
            let expected = Unit::from_str("m/km").unwrap();
            assert_eq!(METER / &KILOMETER, expected);
            let unit = Unit::from_str("10m").unwrap();
            let other = Unit::from_str("20m").unwrap();
            let expected = Unit::from_str("10m/20m").unwrap();
            assert_eq!(unit / &other, expected);

            assert_eq!(seed() / &seed(), UNITY);
            assert_eq!(UNITY / &seed(), Unit::from_str("/{seed}").unwrap());
            assert_eq!(seed() / &ACRE, Unit::from_str("{seed}/[acr_us]").unwrap());
        }

        #[test]
        fn validate_borrowed_div_owned() {
            assert_eq!(&METER / METER, UNITY);
            let expected = Unit::from_str("m/km").unwrap();
            assert_eq!(&METER / KILOMETER, expected);
            let unit = Unit::from_str("10m").unwrap();
            let other = Unit::from_str("20m").unwrap();
            let expected = Unit::from_str("10m/20m").unwrap();
            assert_eq!(&unit / other, expected);

            assert_eq!(&seed() / seed(), UNITY);
            assert_eq!(&UNITY / seed(), Unit::from_str("/{seed}").unwrap());
            assert_eq!(&seed() / ACRE, Unit::from_str("{seed}/[acr_us]").unwrap());
        }

        #[test]
        fn validate_borrowed_div_borrowed() {
            assert_eq!(&METER / &METER, UNITY);
            let expected = Unit::from_str("m/km").unwrap();
            assert_eq!(&METER / &KILOMETER, expected);
            let unit = Unit::from_str("10m").unwrap();
            let other = Unit::from_str("20m").unwrap();
            let expected = Unit::from_str("10m/20m").unwrap();
            assert_eq!(&unit / &other, expected);

            assert_eq!(&seed() / &seed(), UNITY);
            assert_eq!(&UNITY / &seed(), Unit::from_str("/{seed}").unwrap());
            assert_eq!(&seed() / &ACRE, Unit::from_str("{seed}/[acr_us]").unwrap());
        }
    }

    #[test]
    fn validate_mul() {
        let expected = Unit::from_str("m.km").unwrap();
        assert_eq!(METER * KILOMETER, expected);

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        let expected = Unit::from_str("10m.20m").unwrap();
        assert_eq!(unit * other, expected);

        let per_seed = Unit::from_str("/{seed}").unwrap();
        assert_eq!(seed() * &per_seed, UNITY);

        let seed_per_acre = Unit::from_str("{seed}/[acr_us]").unwrap();
        assert_eq!(seed_per_acre * ACRE, seed());
    }
}
