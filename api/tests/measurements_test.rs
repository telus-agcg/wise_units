#[macro_use]
extern crate approx;
extern crate wise_units;

use wise_units::{Convertible, Measurement};

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
            let subject = Measurement::try_new($measurement_value, $measurement_unit).unwrap();
            let converted = subject.convert_to($convert_to_unit).unwrap();
            assert_relative_eq!(converted.value(), $expected_value);
            assert_ulps_eq!(converted.value(), $expected_value);
        }

        #[test]
        fn $convert_to_test_name() {
            let subject = Measurement::try_new($expected_value, $convert_to_unit).unwrap();
            let converted = subject.convert_to($measurement_unit).unwrap();
            assert_ulps_eq!(converted.value(), $measurement_value);
        }
    };
}

validate_conversion!(
    validate_conversion_meters_per_second_to_miles_per_hour_forward,
    validate_conversion_meters_per_second_to_miles_per_hour_back,
    1.0,
    "m/s",
    "[mi_i]/h",
    2.236_936_292_054_402_5
);

validate_conversion!(
    validate_conversion_lb_per_acre_to_lb_per_m2_forward,
    validate_conversion_lb_per_acre_to_lb_per_m2_back,
    1.0,
    "[lb_av]/[acr_us]",
    "[lb_av]/m2",
    0.000_247_104_393_046_627_9
);

validate_conversion!(
    validate_conversion_lb_per_acre_to_lb_per_ft2_forward,
    validate_conversion_lb_per_acre_to_lb_per_ft2_back,
    1.0,
    "[lb_av]/[acr_us]",
    "[lb_av]/[ft_i]2",
    2.295_674_931_138_659e-5
);

validate_conversion!(
    validate_conversion_lb_to_ston_forward,
    validate_conversion_lb_to_ston_back,
    1.0,
    "[lb_av]",
    "[ston_av]",
    0.000_5
);

validate_conversion!(
    validate_conversion_m_per_s_to_ft_per_hour_forward,
    validate_conversion_m_per_s_to_ft_per_hour_back,
    1.0,
    "m/s",
    "[ft_i]/h",
    11_811.023_622_047_243
);

validate_conversion!(
    validate_conversion_lb_to_oz_forward,
    validate_conversion_lb_to_oz_back,
    1.0,
    "[lb_av]",
    "[oz_av]",
    16.0
);

validate_conversion!(
    validate_conversion_g_per_cm3_to_mg_per_l_forward,
    validate_conversion_g_per_cm3_to_mg_per_l_back,
    1.0,
    "g/cm3",
    "mg/L",
    1_000_000.0
);

validate_conversion!(
    validate_conversion_meq_per_100cm3_to_meq_per_l_forward,
    validate_conversion_meq_per_100cm3_to_meq_per_l_back,
    1.0,
    "meq/100cm3",
    "meq/L",
    0.001
);

validate_conversion!(
    validate_conversion_percent_to_g_per_kg_forward,
    validate_conversion_percent_to_g_per_kg_back,
    1.0,
    "%",
    "g/kg",
    10.0
);

validate_conversion!(
    validate_conversion_percent_to_mg_per_kg_forward,
    validate_conversion_percent_to_mg_per_kg_back,
    1.0,
    "%",
    "mg/kg",
    10_000.0
);

validate_conversion!(
    validate_conversion_percent_to_ug_per_kg_forward,
    validate_conversion_percent_to_ug_per_kg_back,
    1.0,
    "%",
    "ug/kg",
    10_000_000.0
);

validate_conversion!(
    validate_conversion_ppm_to_in_per_ft_forward,
    validate_conversion_ppm_to_in_per_ft_back,
    1.0,
    "[ppm]",
    "[in_i]/[ft_i]",
    1.2e-05
);

validate_conversion!(
    validate_conversion_ppm_to_mg_per_kg_forward,
    validate_conversion_ppm_to_mg_per_kg_back,
    1.0,
    "[ppm]",
    "mg/kg",
    1.0
);

validate_conversion!(
    validate_conversion_ppm_to_ug_per_kg_forward,
    validate_conversion_ppm_to_ug_per_kg_back,
    1.0,
    "[ppm]",
    "ug/kg",
    1000.0
);

validate_conversion!(
    validate_conversion_ug_per_kg_to_in_per_ft_forward,
    validate_conversion_ug_per_kg_to_in_per_ft_back,
    1.0,
    "ug/kg",
    "[in_i]/[ft_i]",
    1.2e-08
);

validate_conversion!(
    validate_conversion_lb_acre_per_har_to_kg_forward,
    validate_conversion_lb_acre_per_har_to_kg_back,
    1.0,
    "[lb_av].[acr_us]/har",
    "kg",
    0.183_563_053_820_094_75
);

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

#[test]
fn mebi() {
    let over_one_mebibyte = Measurement::try_new(1.125, "MiBy").unwrap();
    let nine_mebi_bit = over_one_mebibyte.convert_to("bit").unwrap();
    assert_eq!(
        nine_mebi_bit,
        Measurement::try_new(9 * 1024 * 1024, "bit").unwrap()
    );
    eprintln!("M: {} {}", over_one_mebibyte, nine_mebi_bit);
}
