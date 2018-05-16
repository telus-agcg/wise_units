#[macro_use]
extern crate approx;
extern crate simple_logger;
extern crate wise_units;

use wise_units::Measurement;

macro_rules! validate_conversion {
    (
        $convert_from_test_name:ident,
        $convert_to_test_name:ident,
        $measurement_value:expr,
        $measurement_unit:expr,
        $convert_to_unit:expr,
        $expected_value:expr
    ) => {
        #[test]
        fn $convert_from_test_name() {
            simple_logger::init().ok();
            let subject = Measurement::new($measurement_value, $measurement_unit).unwrap();
            let converted = subject.convert_to($convert_to_unit).unwrap();
            assert_relative_eq!(converted.value, $expected_value);
            assert_ulps_eq!(converted.value, $expected_value);
        }

        #[test]
        fn $convert_to_test_name() {
            simple_logger::init().ok();
            let subject = Measurement::new($expected_value, $convert_to_unit).unwrap();
            let converted = subject.convert_to($measurement_unit).unwrap();
            assert_ulps_eq!(converted.value, $measurement_value);
        }
    };
}

validate_conversion!(
    validate_conversion_unity_to_10power_forward,
    validate_conversion_unity_to_10power_back,
    500.0,
    "1",
    "10^",
    50.0
);

validate_conversion!(
    validate_conversion_unity_to_percent_forward,
    validate_conversion_unity_to_percent_back,
    500.0,
    "1",
    "%",
    50_000.0
);

validate_conversion!(
    validate_conversion_m_to_km_forward,
    validate_conversion_m_to_km_back,
    1.0,
    "m",
    "km",
    0.001
);

validate_conversion!(
    validate_conversion_m2_to_km2_forward,
    validate_conversion_m2_to_km2_back,
    1.0,
    "m2",
    "km2",
    0.000_001
);

validate_conversion!(
    validate_conversion_m2_per_s_to_km2_per_s_forward,
    validate_conversion_m2_per_s_to_km2_per_s_back,
    1.0,
    "m2/s",
    "km2/s",
    0.000_001
);

validate_conversion!(
    validate_conversion_s_per_m2_to_s_per_km2_forward,
    validate_conversion_s_per_m2_to_s_per_km2_back,
    1.0,
    "s/m2",
    "s/km2",
    1_000_000.0
);

validate_conversion!(
    validate_conversion_pi_m2_to_m2_forward,
    validate_conversion_pi_m2_to_m2_back,
    5.0,
    "[pi].m2",
    "m2",
    5.0 * std::f64::consts::PI
);

validate_conversion!(
    validate_conversion_percent_to_10power_forward,
    validate_conversion_percent_to_10power_back,
    500.0,
    "%",
    "10^",
    0.5
);

validate_conversion!(
    validate_conversion_pi_to_ppth_forward,
    validate_conversion_pi_to_ppth_back,
    1.0,
    "[pi]",
    "[ppth]",
    1000.0 * std::f64::consts::PI
);

validate_conversion!(
    validate_conversion_l_to_m3_forward,
    validate_conversion_l_to_m3_back,
    2.0,
    "l",
    "m3",
    0.002
);

validate_conversion!(
    validate_special_conversion_cel_to_k_forward,
    validate_special_conversion_cel_to_k_back,
    25.0,
    "Cel",
    "K",
    298.15
);

validate_conversion!(
    validate_special_conversion_degf_to_k_forward,
    validate_special_conversion_degf_to_k_back,
    98.6,
    "[degF]",
    "K",
    310.15
);

validate_conversion!(
    validate_special_conversion_degf_to_cel_forward,
    validate_special_conversion_degf_to_cel_back,
    98.6,
    "[degF]",
    "Cel",
    37.0
);

validate_conversion!(
    validate_special_conversion_degre_to_k_forward,
    validate_special_conversion_degre_to_k_back,
    100.0,
    "[degRe]",
    "K",
    398.15
);

validate_conversion!(
    validate_special_conversion_degre_to_cel_forward,
    validate_special_conversion_degre_to_cel_back,
    100.0,
    "[degRe]",
    "Cel",
    125.0
);

validate_conversion!(
    validate_special_conversion_deg_to_rad_forward,
    validate_special_conversion_deg_to_rad_back,
    180.0,
    "deg",
    "rad",
    std::f64::consts::PI
);

validate_conversion!(
    validate_special_conversion_ph_to_mol_per_l1_forward,
    validate_special_conversion_ph_to_mol_per_l1_back,
    1.0,
    "[pH]",
    "mol/l",
    0.0
);

validate_conversion!(
    validate_special_conversion_ph_to_mol_per_l2_forward,
    validate_special_conversion_ph_to_mol_per_l2_back,
    1.0,
    "[pH]",
    "mol/l",
    0.0
);

validate_conversion!(
    validate_special_conversion_mol_per_l_to_ph1_forward,
    validate_special_conversion_mol_per_l_to_ph1_back,
    0.0,
    "mol/l",
    "[pH]",
    1.0
);

validate_conversion!(
    validate_special_conversion_mol_per_l_to_ph2_forward,
    validate_special_conversion_mol_per_l_to_ph2_back,
    -1.0,
    "mol/l",
    "[pH]",
    10.0
);

// TODO: I don't think UCUM defines the conversion functions properly, based on
// https://en.wikipedia.org/wiki/Decibel_watt.
// validate_conversion!(
//     validate_special_conversion_bel_to_decibel_forward,
//     validate_special_conversion_bel_to_decibel_back,
//     2.0, "B", "dB",
//     20.0
// );

// validate_conversion!(
//     validate_special_conversion_10bel_watt_to_bel_kilowatt_forward,
//     validate_special_conversion_10bel_watt_to_bel_kilowatt_back,
//     2.0, "10B[W]", "B[kW]",
//     3.0
// );

// validate_conversion!(
//     validate_special_conversion_milliwatt_to_decibelwatt_forward,
//     validate_special_conversion_milliwatt_to_decibelwatt_back,
//     1.0, "mW", "dB[W]",
//     -30.0
// );

validate_conversion!(
    validate_special_conversion_bel_to_neper_forward,
    validate_special_conversion_bel_to_neper_back,
    2.0,
    "B",
    "Np",
    4.605_170_185_988_092
);

validate_conversion!(
    validate_special_conversion_bit_s_to_bit_forward,
    validate_special_conversion_bit_s_to_bit_back,
    2.0,
    "bit_s",
    "bit",
    4.0
);

validate_conversion!(
    validate_special_conversion_bel_watt_to_bel_kilowatt_forward,
    validate_special_conversion_bel_watt_to_bel_kilowatt_back,
    2.0,
    "B[W]",
    "B[kW]",
    2.0
);

validate_conversion!(
    validate_special_conversion_bel_watt_to_watt_forward,
    validate_special_conversion_bel_watt_to_watt_back,
    0.0,
    "B[W]",
    "W",
    1.0
);
