extern crate wise_units;

use wise_units::Measurement;

#[test]
fn validate_unity_conversions() {
    let subject = Measurement::new(500.0, "1");
    let converted = subject.convert_to("10^").unwrap();
    assert_floats_eq(converted.value, 50.0);

    let subject = Measurement::new(500.0, "1");
    let converted = subject.convert_to("%").unwrap();
    assert_floats_eq(converted.value, 50_000.0);
}

#[test]
fn validate_meter_conversions() {
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
}

#[test]
fn validate_pi_conversions() {
    let subject = Measurement::new(5.0, "[pi].m2");
    let converted = subject.convert_to("m2").unwrap();
    assert_floats_eq(converted.value, 15.707_963_267);
}

#[test]
fn validate_number_conversions() {
    let subject = Measurement::new(500.0, "%");
    let converted = subject.convert_to("10^").unwrap();
    assert_floats_eq(converted.value, 0.5);

    let subject = Measurement::new(1.0, "[pi]");
    let converted = subject.convert_to("[ppth]").unwrap();
    assert_floats_eq(converted.value, 3141.592_653_589);

    // TODO: Special units
    // let subject = Measurement::new(7.0, "[pH]");
    // let converted = subject.convert_to("mol/l").unwrap();
    // assert_floats_eq(converted.value, 0.000_000_1);

    // let subject = Measurement::new(7.0, "mol/l");
    // let converted = subject.convert_to("[pH]").unwrap();
    // assert_floats_eq(converted.value, -0.845098040014257);
}

#[test]
fn validate_liter_conversions() {
    let subject = Measurement::new(2.0, "l");
    let converted = subject.convert_to("m3").unwrap();
    assert_floats_eq(converted.value, 0.002);
}

// TODO: Special units
#[test]
#[ignore(reason = "Special Units")]
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

    let subject = Measurement::new(100.0, "[degRe]");
    let converted = subject.convert_to("K").unwrap();
    assert_floats_eq(converted.value, 398.15);

    let subject = Measurement::new(398.15, "K");
    let converted = subject.convert_to("[degRe]").unwrap();
    assert_floats_eq(converted.value, 100.0);

    let subject = Measurement::new(100.0, "[degRe]");
    let converted = subject.convert_to("Cel").unwrap();
    assert_floats_eq(converted.value, 125.0);

    let subject = Measurement::new(180.0, "deg");
    let converted = subject.convert_to("rad").unwrap();
    assert_floats_eq(converted.value, std::f64::consts::PI);

    let subject = Measurement::new(std::f64::consts::PI, "rad");
    let converted = subject.convert_to("deg").unwrap();
    assert_floats_eq(converted.value, 180.0);

    // TODO: I don't understand why this fails.
    let subject = Measurement::new(1.0, "[p'diop]");
    let converted = subject.convert_to("deg").unwrap();
    assert_floats_eq(converted.value, 0.57);
}

// Because the precision of floats can vary, using assert_eq! with float values
// is not recommended; clippy's recommendation is to calculate the absolute
// value of the difference and make sure that it's under some acceptable
// threshold.
fn assert_floats_eq(actual: f64, expected: f64) {
    let error_threshold = f64::from(std::f32::EPSILON);
    let difference = actual - expected;

    assert!(difference.abs() < error_threshold, "Actual: {}, Expected: {}, Diff: {}", actual, expected, difference);
}
