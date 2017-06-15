extern crate wu;
use wu::Measurement;

#[test]
fn validate_conversions() {
    let subject = Measurement::new(500.0, "1");
    let converted = subject.convert_to("10^").unwrap();
    assert_floats_eq(converted.value, 50.0);

    let subject = Measurement::new(500.0, "1");
    let converted = subject.convert_to("%").unwrap();
    assert_floats_eq(converted.value, 50_000.0);

    let subject = Measurement::new(1.0, "m");
    let converted = subject.convert_to("km").unwrap();
    assert_floats_eq(converted.value, 0.001);

    let subject = Measurement::new(1.0, "km");
    let converted = subject.convert_to("m").unwrap();
    assert_floats_eq(converted.value, 1_000.0);

    let subject = Measurement::new(1.0, "m2");
    let converted = subject.convert_to("km2").unwrap();
    assert_floats_eq(converted.value, 0.000_001);

    let subject = Measurement::new(1.0, "km2");
    let converted = subject.convert_to("m2").unwrap();
    assert_floats_eq(converted.value, 1_000_000.0);

    let subject = Measurement::new(1.0, "m2/s");
    let converted = subject.convert_to("km2/s").unwrap();
    assert_floats_eq(converted.value, 0.000_001);

    let subject = Measurement::new(1.0, "km2/s");
    let converted = subject.convert_to("m2/s").unwrap();
    assert_floats_eq(converted.value, 1_000_000.0);

    let subject = Measurement::new(1.0, "s/m2");
    let converted = subject.convert_to("s/km2").unwrap();
    assert_floats_eq(converted.value, 1_000_000.0);

    let subject = Measurement::new(1.0, "s/km2");
    let converted = subject.convert_to("s/m2").unwrap();
    assert_floats_eq(converted.value, 0.000_001);

    let subject = Measurement::new(5.0, "[pi].m2");
    let converted = subject.convert_to("m2").unwrap();
    assert_floats_eq(converted.value, 15.708);

    let subject = Measurement::new(500.0, "%");
    let converted = subject.convert_to("10^").unwrap();
    assert_floats_eq(converted.value, 0.5);

    let subject = Measurement::new(1.0, "[pi]");
    let converted = subject.convert_to("[ppth]").unwrap();
    assert_floats_eq(converted.value, 3141.5927);

    let subject = Measurement::new(2.0, "l");
    let converted = subject.convert_to("m3").unwrap();
    assert_floats_eq(converted.value, 0.002);

    // let subject = Measurement::new(7.0, "[pH]");
    // let converted = subject.convert_to("mole").unwrap();
    // assert_eq!(round_value(converted.value), 3141.5927);
}

#[test]
fn validate_special_conversions() {
    let subject = Measurement::new(25.0, "Cel");
    let converted = subject.convert_to("K").unwrap();
    assert_floats_eq(converted.value, 298.15);

    let subject = Measurement::new(298.15, "K");
    let converted = subject.convert_to("Cel").unwrap();
    assert_floats_eq(converted.value, 25.0);

    let subject = Measurement::new(98.6, "[degF]");
    let converted = subject.convert_to("K").unwrap();
    assert_floats_eq(converted.value, 310.15);

    let subject = Measurement::new(310.15, "K");
    let converted = subject.convert_to("[degF]").unwrap();
    assert_floats_eq(converted.value, 98.6);

    let subject = Measurement::new(98.6, "[degF]");
    let converted = subject.convert_to("Cel").unwrap();
    assert_floats_eq(converted.value, 37.0);

    let subject = Measurement::new(37.0, "Cel");
    let converted = subject.convert_to("[degF]").unwrap();
    assert_floats_eq(converted.value, 98.6);
}

fn assert_floats_eq(actual: f64, expected: f64) {
    let error_threshold = 0.000_000_1;
    let difference = (actual - expected).abs();

    assert!(difference < error_threshold, "Difference in floats was {}", difference);
}
