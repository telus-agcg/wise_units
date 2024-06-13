use num_traits::{FromPrimitive, ToPrimitive};

use crate::Measurement;

impl ToPrimitive for Measurement {
    fn to_i64(&self) -> Option<i64> {
        ToPrimitive::to_f64(self)
            .as_ref()
            .and_then(ToPrimitive::to_i64)
    }

    fn to_u64(&self) -> Option<u64> {
        ToPrimitive::to_f64(self)
            .as_ref()
            .and_then(ToPrimitive::to_u64)
    }

    fn to_f64(&self) -> Option<f64> {
        Some(f64::from(self))
    }
}

impl FromPrimitive for Measurement {
    fn from_i64(n: i64) -> Option<Self> {
        n.to_f64().and_then(FromPrimitive::from_f64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        n.to_f64().and_then(FromPrimitive::from_f64)
    }

    fn from_f64(n: f64) -> Option<Self> {
        <Self as num_traits::NumCast>::from(n)
    }
}

impl num_traits::NumCast for Measurement {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        Some(Self {
            value: n.to_f64()?,
            unit: crate::unit::UNITY,
        })
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    use super::*;

    #[test]
    fn to_primitive_test() {
        let m = measurement!(10.5, "kL");
        assert_ulps_eq!(10.5, f64::from(&m));
        assert_eq!(10, m.to_i64().unwrap());
        assert_eq!(10, m.to_u64().unwrap());

        let m = measurement!(-10.5, "kL");
        assert_ulps_eq!(-10.5, f64::from(&m));
        assert_eq!(-10, m.to_i64().unwrap());
        assert!(m.to_u64().is_none());

        let m = measurement!(10.0, "L");
        assert_ulps_eq!(0.01, f64::from(&m));
        assert_eq!(0, m.to_i64().unwrap());
        assert_eq!(0, m.to_u64().unwrap());

        let m = measurement!(-10.0, "L");
        assert_ulps_eq!(-0.01, f64::from(&m));
        assert_eq!(0, m.to_i64().unwrap());
        assert_eq!(0, m.to_u64().unwrap());
    }

    #[test]
    fn from_primitive_test() {
        let output = Measurement::from_u32(u32::MAX).unwrap();
        assert_eq!(
            Measurement::new(u32::MAX.into(), crate::unit::UNITY),
            output
        );

        let output = Measurement::from_i32(i32::MAX).unwrap();
        assert_eq!(
            Measurement::new(i32::MAX.into(), crate::unit::UNITY),
            output
        );
    }

    #[test]
    fn numcast_test() {
        use num_traits::NumCast;

        let output = <Measurement as NumCast>::from(u32::MAX).unwrap();
        assert_eq!(
            Measurement::new(u32::MAX.into(), crate::unit::UNITY),
            output
        );

        let output = <Measurement as NumCast>::from(i32::MIN).unwrap();
        assert_eq!(
            Measurement::new(i32::MIN.into(), crate::unit::UNITY),
            output
        );
    }
}
