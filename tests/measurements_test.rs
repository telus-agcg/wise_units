extern crate wu;
use wu::Measurement;

#[test]
fn validate_conversions() {
    let subject = Measurement::new(1.0, "m");
    let converted = subject.convert_to("km").unwrap();
    assert_eq!(converted.value, 0.001);

    let subject = Measurement::new(1.0, "km");
    let converted = subject.convert_to("m").unwrap();
    assert_eq!(converted.value, 1000.0);

    let subject = Measurement::new(1.0, "m2");
    let converted = subject.convert_to("km2").unwrap();
    assert_eq!(converted.value, 0.001);

    let subject = Measurement::new(1.0, "km2");
    let converted = subject.convert_to("m2").unwrap();
    assert_eq!(converted.value, 1000.0);

    let subject = Measurement::new(1.0, "m2/s");
    let converted = subject.convert_to("km2/s").unwrap();
    assert_eq!(converted.value, 0.001);

    let subject = Measurement::new(1.0, "km2/s");
    let converted = subject.convert_to("m2/s").unwrap();
    assert_eq!(converted.value, 1000.0);

    let subject = Measurement::new(1.0, "s/m2");
    let converted = subject.convert_to("s/km2").unwrap();
    assert_eq!(converted.value, 0.001);

    let subject = Measurement::new(1.0, "s/km2");
    let converted = subject.convert_to("s/m2").unwrap();
    assert_eq!(converted.value, 1000.0);
}
