use crate::{Measurement, UcumUnit};

impl<'a> From<&'a Measurement> for f64 {
    fn from(measurement: &'a Measurement) -> Self {
        measurement.scalar()
    }
}
