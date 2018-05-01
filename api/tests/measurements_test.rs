#[macro_use]
extern crate approx;

extern crate wise_units;

use wise_units::Measurement;

macro_rules! validate_conversion {
    (
        $test_name: ident,
        $measurement_value: expr,
        $measurement_unit: expr,
        $convert_to_unit: expr,
        $expected_value: expr
    ) => {
        #[test]
        fn $test_name() {
            let subject = Measurement::new($measurement_value, $measurement_unit).unwrap();
            let converted = subject.convert_to($convert_to_unit).unwrap();
            assert_relative_eq!(converted.value, $expected_value);
            assert_ulps_eq!(converted.value, $expected_value);

            // Now test converting back
            let subject = Measurement::new($expected_value, $convert_to_unit).unwrap();
            let converted = subject.convert_to($measurement_unit).unwrap();
            assert_ulps_eq!(converted.value, $measurement_value);
        }
    };
}

validate_conversion!(
    validate_conversion_unity_to_10power,
    500.0, "1", "10^",
    50.0
);

validate_conversion!(
    validate_conversion_unity_to_percent,
    500.0, "1", "%",
    50_000.0
);

validate_conversion!(
    validate_conversion_m_to_km,
    1.0, "m", "km",
    0.001
);

validate_conversion!(
    validate_conversion_m2_to_km2,
    1.0, "m2", "km2",
    0.000_001
);

validate_conversion!(
    validate_conversion_m2_per_s_to_km2_per_s,
    1.0, "m2/s", "km2/s",
    0.000_001
);

validate_conversion!(
    validate_conversion_s_per_m2_to_s_per_km2,
    1.0, "s/m2", "s/km2",
    1_000_000.0
);

validate_conversion!(
    validate_conversion_pi_m2_to_m2,
    5.0, "[pi].m2", "m2",
    5.0 * std::f64::consts::PI
);

validate_conversion!(
    validate_conversion_percent_to_10power,
    500.0, "%", "10^",
    0.5
);

validate_conversion!(
    validate_conversion_pi_to_ppth,
    1.0, "[pi]", "[ppth]",
    1000.0 * std::f64::consts::PI
);

validate_conversion!(
    validate_conversion_l_to_m3,
    2.0, "l", "m3",
    0.002
);

validate_conversion!(
    validate_special_conversion_cel_to_k,
    25.0, "Cel", "K",
    298.15
);

validate_conversion!(
    validate_special_conversion_degf_to_k,
    98.6, "[degF]", "K",
    310.15
);

validate_conversion!(
    validate_special_conversion_degf_to_cel,
    98.6, "[degF]", "Cel",
    37.0
);

validate_conversion!(
    validate_special_conversion_degre_to_k,
    100.0, "[degRe]", "K",
    398.15
);

validate_conversion!(
    validate_special_conversion_degre_to_cel,
    100.0, "[degRe]", "Cel",
    125.0
);

validate_conversion!(
    validate_special_conversion_deg_to_rad,
    180.0, "deg", "rad",
    std::f64::consts::PI
);

validate_conversion!(
    validate_special_conversion_ph_to_mol_per_l1,
    1.0, "[pH]", "mol/l",
    0.0
);

validate_conversion!(
    validate_special_conversion_ph_to_mol_per_l2,
    10.0, "[pH]", "mol/l",
    -1.0
);

validate_conversion!(
    validate_special_conversion_mol_per_l_to_ph1,
    0.0, "mol/l", "[pH]",
    1.0
);

validate_conversion!(
    validate_special_conversion_mol_per_l_to_ph2,
    -1.0, "mol/l", "[pH]",
    10.0
);
