//! These tests are for the dynamically generated `symbol_parser` module.
//!
#[cfg(test)]
mod atom_test {
    use crate::parser::{
        Atom, Classification, Composable, Composition, Dimension, Prefix, Term, UcumSymbol,
    };
    use crate::ucum_unit::UcumUnit;

    macro_rules! validate_definition {
        (
            $test_name:ident,
            $atom_name:ident,
            $expected_value:expr,
            $($expected_term:expr),+
        ) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$atom_name;
                let expected = vec![$($expected_term),+];

                assert_eq!(atom.definition().value(), $expected_value);
                assert_eq!(atom.definition().terms(), expected.as_slice());
            }
        };
    }

    macro_rules! validate_scalar {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$variant;
                assert_relative_eq!(atom.scalar(), $value);
                assert_ulps_eq!(atom.scalar(), $value);
            }
        };
    }

    macro_rules! validate_scalars {
        ($($test_name: ident, $variant: ident, $value: expr);+ $(;)*) => {
            $(
                validate_scalar!($test_name, $variant, $value);
             )+
        };
    }

    macro_rules! validate_magnitude {
        ($test_name:ident, $variant:ident, $value:expr) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$variant;
                assert_relative_eq!(atom.magnitude(), $value);
                assert_ulps_eq!(atom.magnitude(), $value);
            }
        };
    }

    macro_rules! validate_magnitudes {
        ($($test_name: ident, $variant: ident, $value: expr);+ $(;)*) => {
            $(
                validate_magnitude!($test_name, $variant, $value);
             )+
        };
    }

    macro_rules! validate_composition {
        (
            @insert
            $composition:expr,
            $dimension_variant:ident: $value:expr
        ) => {
            $composition.insert(Dimension::$dimension_variant, $value);
        };

        (
            $test_name:ident,
            $atom_variant:ident,
            $($dimension_variant:ident: $value:expr),+
        ) => {
            #[test]
            fn $test_name() {
                let atom = Atom::$atom_variant;
                let mut composition = Composition::default();
                $(
                    validate_composition!(@insert composition, $dimension_variant: $value);
                 )+
                    assert_eq!(atom.composition(), composition);
            }
        };
    }

    #[test]
    fn validate_classification_si() {
        let atoms = vec![
            Atom::Candela,
            Atom::Coulomb,
            Atom::DegreeCelsius,
            Atom::Gram,
            Atom::Kelvin,
            Atom::Meter,
            Atom::Radian,
            Atom::Second,
            Atom::Mole,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Si);
        }
    }

    #[test]
    fn validate_classification_us_lengths() {
        let atoms = vec![Atom::AcreUS, Atom::FootUS, Atom::RodUS];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::UsLengths);
        }
    }

    #[test]
    fn validate_classification_iso1000() {
        let atoms = vec![Atom::Are, Atom::Degree, Atom::Liter];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Iso1000);
        }
    }

    #[test]
    fn validate_classification_heat() {
        let atoms = vec![Atom::DegreeFahrenheit, Atom::DegreeReaumur];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Heat);
        }
    }

    #[test]
    fn validate_classification_us_volumes() {
        let atoms = vec![
            Atom::FluidOunceUS,
            Atom::GillUS,
            Atom::QuartUS,
            Atom::QueenAnnesWineGallonUS,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::UsVolumes);
        }
    }

    #[test]
    fn validate_classification_intcust() {
        let atoms = vec![Atom::InchInternational, Atom::FootInternational];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Intcust);
        }
    }

    #[test]
    fn validate_classification_dimless() {
        let atoms = vec![
            Atom::PartsPerBillion,
            Atom::PartsPerMillion,
            Atom::PartsPerThousand,
            Atom::PartsPerTrillion,
            Atom::Percent,
            Atom::TheNumberPi,
            Atom::TheNumberTenForArbitraryPowersCaret,
            Atom::TheNumberTenForArbitraryPowersStar,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Dimless);
        }
    }

    #[test]
    fn validate_classification_chemical() {
        let atoms = vec![Atom::PH];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Chemical);
        }
    }

    #[test]
    fn validate_classification_clinical() {
        let atoms = vec![Atom::PrismDiopter];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Clinical);
        }
    }

    #[test]
    fn validate_definitions_base_atoms() {
        let base_atoms = vec![
            Atom::Candela,
            Atom::Coulomb,
            Atom::Gram,
            Atom::Kelvin,
            Atom::Meter,
            Atom::Radian,
            Atom::Second,
        ];
        let terms = vec![Term::new_unity()];

        for base_atom in base_atoms {
            assert_eq!(base_atom.definition().value(), 1.0);
            assert_eq!(base_atom.definition().terms(), terms.as_slice());
        }
    }

    validate_definition!(
        validate_definition_acre_us,
        AcreUS,
        160.0,
        term!(RodUS, exponent: 2)
    );

    validate_definition!(
        validate_definition_are,
        Are,
        100.0,
        term!(Meter, exponent: 2)
    );

    validate_definition!(
        validate_definition_degree,
        Degree,
        2.0,
        term!(TheNumberPi),
        term!(Radian),
        term!(factor: 360, exponent: -1)
    );
    validate_definition!(
        validate_definition_degree_celsius,
        DegreeCelsius,
        1.0,
        term!(Kelvin)
    );
    validate_definition!(
        validate_definition_degree_fahrenheit,
        DegreeFahrenheit,
        5.0,
        term!(Kelvin),
        term!(factor: 9, exponent: -1)
    );
    validate_definition!(
        validate_definition_degree_reaumur,
        DegreeReaumur,
        5.0,
        term!(Kelvin),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_fluid_ounce_us,
        FluidOunceUS,
        1.0,
        term!(GillUS),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_foot_international,
        FootInternational,
        12.0,
        term!(InchInternational)
    );
    validate_definition!(
        validate_definition_inch_international,
        InchInternational,
        254e-2,
        term!(Centi, Meter)
    );
    validate_definition!(
        validate_definition_foot_us,
        FootUS,
        1200.0,
        term!(Meter),
        term!(factor: 3937, exponent: -1)
    );
    validate_definition!(
        validate_definition_gill_us,
        GillUS,
        1.0,
        term!(PintUS),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_liter,
        Liter,
        1.0,
        term!(Deci, Meter, exponent: 3)
    );
    validate_definition!(
        validate_definition_mole,
        Mole,
        6.022_136_7,
        term!(TheNumberTenForArbitraryPowersStar, exponent: 23)
    );
    validate_definition!(
        validate_definition_parts_per_billion,
        PartsPerBillion,
        1.0,
        term!(TheNumberTenForArbitraryPowersStar, exponent: -9)
    );
    validate_definition!(
        validate_definition_parts_per_million,
        PartsPerMillion,
        1.0,
        term!(TheNumberTenForArbitraryPowersStar, exponent: -6)
    );
    validate_definition!(
        validate_definition_parts_per_thousand,
        PartsPerThousand,
        1.0,
        term!(TheNumberTenForArbitraryPowersStar, exponent: -3)
    );
    validate_definition!(
        validate_definition_ph,
        PH,
        1.0,
        term!(Mole),
        term!(Liter, exponent: -1)
    );
    validate_definition!(
        validate_definition_pint_us,
        PintUS,
        1.0,
        term!(QuartUS),
        term!(factor: 2, exponent: -1)
    );
    validate_definition!(
        validate_definition_prism_diopter,
        PrismDiopter,
        1.0,
        term!(Radian)
    );
    validate_definition!(validate_definition_bel_watt, BelWatt, 1.0, term!(Watt));
    validate_definition!(
        validate_definition_quart_us,
        QuartUS,
        1.0,
        term!(QueenAnnesWineGallonUS),
        term!(factor: 4, exponent: -1)
    );
    validate_definition!(
        validate_definition_metric_cup,
        MetricCup,
        240.0,
        term!(Milli, LiterSecondary)
    );
    validate_definition!(
        validate_definition_queen_annes_wine_gallon_us,
        QueenAnnesWineGallonUS,
        231.0,
        term!(InchInternational, exponent: 3)
    );
    validate_definition!(validate_definition_rod_us, RodUS, 16.5, term!(FootUS));
    validate_definition!(
        validate_definition_the_number_pi,
        TheNumberPi,
        3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3,
        term!(factor: 1)
    );
    validate_definition!(
        validate_definition_the_number_ten_for_arbitrary_powers_caret,
        TheNumberTenForArbitraryPowersCaret,
        10.0,
        term!(factor: 1)
    );
    validate_definition!(
        validate_definition_the_number_ten_for_arbitrary_powers_star,
        TheNumberTenForArbitraryPowersStar,
        10.0,
        term!(factor: 1)
    );

    // Composition tests
    validate_composition!(validate_composition_candela, Candela, LuminousIntensity: 1);
    validate_composition!(validate_composition_coulomb, Coulomb, ElectricCharge: 1);
    validate_composition!(validate_composition_gram, Gram, Mass: 1);
    validate_composition!(validate_composition_kelvin, Kelvin, Temperature: 1);
    validate_composition!(validate_composition_meter, Meter, Length: 1);
    validate_composition!(validate_composition_radian, Radian, PlaneAngle: 1);
    validate_composition!(validate_composition_second, Second, Time: 1);

    validate_composition!(validate_composition_acre_british, AcreBritish, Length: 2);
    validate_composition!(validate_composition_acre_us, AcreUS, Length: 2);
    validate_composition!(validate_composition_are, Are, Length: 2);
    validate_composition!(validate_composition_astronomic_unit, AstronomicUnit, Length: 1);
    validate_composition!(validate_composition_bar, Bar, Length: -1, Mass: 1, Time: -2);
    validate_composition!(validate_composition_barrel_us, BarrelUS, Length: 3);
    validate_composition!(validate_composition_becquerel, Becquerel, Time: -1);
    validate_composition!(validate_composition_biot, Biot, ElectricCharge: 1, Time: -1);
    validate_composition!(validate_composition_board_foot_international, BoardFootInternational, Length: 3);
    validate_composition!(
        validate_composition_boltzmann_constant,
        BoltzmannConstant,
        Length: 2, Mass: 1, Temperature: -1, Time: -2
    );
    validate_composition!(
        validate_composition_british_thermal_unit_at_39f,
        BritishThermalUnitAt39F,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_bushel_british,
        BushelBritish,
        Length: 3
    );
    validate_composition!(
        validate_composition_bushel_us,
        BushelUS,
        Length: 3
    );
    validate_composition!(
        validate_composition_calorie_at_15c,
        CalorieAt15C,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_calorie_at_20c,
        CalorieAt20C,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_international_table_calorie,
        InternationalTableCalorie,
        Length: 2, Mass: 1, Time: -2
    );
    validate_composition!(
        validate_composition_pound_av,
        PoundAvoirdupois,
        Mass: 1
    );

    // Scalar tests
    validate_scalars!(
        validate_scalar_candela, Candela, 1.0;
        validate_scalar_coulomb, Coulomb, 1.0;
        validate_scalar_gram, Gram, 1.0;
        validate_scalar_kelvin, Kelvin, 1.0;
        validate_scalar_meter, Meter, 1.0;
        validate_scalar_radian, Radian, 1.0;
        validate_scalar_second, Second, 1.0;

        validate_scalar_acre_br, AcreBritish, 4046.850_049_400_268_7;
        validate_scalar_acre_us, AcreUS, 4046.872_609_874_252;
        validate_scalar_are, Are, 100.0;
        validate_scalar_astronomic_unit, AstronomicUnit, 149_597_870_691.0;
        validate_scalar_bar, Bar, 100_000_000.0;
        validate_scalar_barrel_us, BarrelUS, 0.158_987_294_928;
        validate_scalar_becquerel, Becquerel, 1.0;
        validate_scalar_biot, Biot, 10.0;
        validate_scalar_board_foot_international, BoardFootInternational, 0.002_359_737_216;
        validate_scalar_boltzmann_constant, BoltzmannConstant, 1.380_658e-20;
        validate_scalar_btu_at_39f, BritishThermalUnitAt39F, 1_059_670.0;
        validate_scalar_bushel_br, BushelBritish, 0.036_368_72;
    validate_scalar_bushel_us, BushelUS, 0.035_239_070_166_88;

    validate_scalar_calorie_at_20c, CalorieAt20C, 4181.9;
    validate_scalar_calorie_at_15c, CalorieAt15C, 4185.8;
    validate_scalar_calorie_it, InternationalTableCalorie, 4186.8;
    validate_scalar_calorie_th, ThermochemicalCalorie, 4184.0;
    validate_scalar_calorie_m, MeanCalorie, 4190.02;
    validate_scalar_calorie, Calorie, 4184.0;
    validate_scalar_calorie_nutrition_label, NutritionLabelCalories, 4_184_000.0;
    validate_scalar_cicero, Cicero, 0.004_511_111_111_111_111;
    validate_scalar_circular_mil_international, CircularMilInternational, 1_217_369_588.005_220_4;
    validate_scalar_cord_international, CordInternational, 3.624_556_363_776;
    validate_scalar_cord_us, CordUS, 3.624_556_363_776;
    validate_scalar_cubic_foot_international, CubicFootInternational, 0.028_316_846_592;
    validate_scalar_cubic_inch_international, CubicInchInternational, 1.638_706_4e-05;
    validate_scalar_cubic_yard_international, CubicYardInternational, 0.764_554_857_984;
    validate_scalar_cup_us, CupUS, 0.000_236_588_236_5;
    validate_scalar_curie, Curie, 37_000_000_000.0;

    validate_scalar_day, Day, 86_400.0;
    validate_scalar_degree, Degree, 0.0174_532_925_199_432_95;
    validate_scalar_degree_minute, MinuteAngle, 0.000_290_888_208_665_721_6;
    validate_scalar_degree_rankine, DegreeRankine, 0.555_555_555_555_555_6;
    validate_scalar_degree_second, SecondAngle, 4.848_136_811_095_36e-06;
    validate_scalar_didot, Didot, 0.000_375_925_925_925_925_93;
    validate_scalar_dram_ap, DramApothecaries, 3.887_934_6;
    validate_scalar_dram_av, DramAvoirdupois, 1.771_845_195_312_5;
    validate_scalar_dry_pint_us, DryPintUS, 0.000_550_610_471_357_5;
    validate_scalar_dry_quart_us, DryQuartUS, 0.001_101_220_942_715;
    validate_scalar_dyne, Dyne, 0.01;

    validate_scalar_electron_mass, ElectronMass, 9.109_389_7e-28;
    validate_scalar_electron_vold, Electronvolt, 1.602_177_33e-16;
    validate_scalar_elementary_charge, ElementaryCharge, 1.60217733e-19;
    validate_scalar_equivalents, Equivalents, 6.0221367e+23;
    validate_scalar_erg, Erg, 0.0001;

    validate_scalar_farad, Farad, 0.001;
    validate_scalar_fathom_br, FathomBritish, 1.828_798_56;
    validate_scalar_fathom_international, FathomInternational, 1.828_8;
    validate_scalar_fathom_us, FathomUS, 1.828_803_657_607_315_2;
    validate_scalar_fluid_dram_br, FluidDramBritish, 3.551_632_812_5e-06;
    validate_scalar_fluid_dram_us, FluidDramUS, 3.696_691_195_312_5e-06;
    validate_scalar_fluid_ounce_br, FluidOunceBritish, 2.841_306_25e-05;
    validate_scalar_fluid_ounce_us, FluidOunceUS, 2.95735295625e-05;
    validate_scalar_foot_br, FootBritish, 0.304_799_76;
    validate_scalar_foot_international, FootInternational, 0.3048;
    validate_scalar_foot_us, FootUS, 0.304_800_609_601_219_2;
    validate_scalar_furlong_us, FurlongUS, 201.16840233680466;

    validate_scalar_gal, Gal, 0.01;
    validate_scalar_gallon_br, GallonBritish, 0.004_546_09;
    validate_scalar_gauss, Gauss, 0.1;
    validate_scalar_gilbert, Gilbert, 0.795_774_715_459_476_8;
    validate_scalar_gill_br, GillBritish, 0.000_142_065_312_5;
    validate_scalar_gill_us, GillUS, 0.000_118_294_118_25;
    validate_scalar_gon, Gon, 0.015_707_963_267_948_967;
    validate_scalar_gram_force, GramForce, 9.806_65;
    validate_scalar_gram_percent, GramPercent, 9999.999_999_999_996;
    validate_scalar_grain, Grain, 0.064_798_91;
    validate_scalar_gray, Gray, 1.0;
    validate_scalar_gunters_chain_br, GuntersChainBritish, 20.116_784_16;
    validate_scalar_gunters_chain_us, GuntersChainUS, 20.116_840_233_680_467;

    validate_scalar_hand_international, HandInternational, 0.1016;
    validate_scalar_hertz, Hertz, 1.0;
    validate_scalar_henry, Henry, 1_000.0;
    validate_scalar_historical_winchester_gallon, HistoricalWinchesterGallon, 0.004_404_883_770_86;
    validate_scalar_horsepower, Horsepower, 745_699.871_582_270_3;
    validate_scalar_hour, Hour, 3600.0;

    validate_scalar_inch_br, InchBritish, 0.025_399_98;
    validate_scalar_inch_international, InchInternational, 0.025_4;
    validate_scalar_inch_us, InchUS, 0.025_400_050_800_101_6;

    validate_scalar_joule, Joule, 1000.0;

    validate_scalar_kayser, Kayser, 100.0;
    validate_scalar_knot_br, KnotBritish, 0.514_772_928;
    validate_scalar_knot_international, KnotInternational, 0.514_444_444_444_444_5;

    validate_scalar_lambert, Lambert, 31_415.926_535_897_932;
    validate_scalar_long_hundredweight_av, LongHunderdweightAvoirdupois, 50_802.345_44;
    validate_scalar_long_ton_av, LongTonAvoirdupois, 1_016_046.908_8;
    validate_scalar_light_year, LightYear, 9.460_730_472_580_8e+15;
    validate_scalar_ligne, Ligne, 0.002_255_555_555_555_555_4;
    validate_scalar_line, Line, 0.002_116_666_666_666_667;
    validate_scalar_link_for_gunters_chain_br, LinkForGuntersChainBritish, 0.201_167_841_6;
    validate_scalar_link_for_gunters_chain_us, LinkForGuntersChainUS, 0.201_168_402_336_804_66;
    validate_scalar_liter, Liter, 0.001;
    validate_scalar_lumen, Lumen, 1.0;
    validate_scalar_lux, Lux, 1.0;

    validate_scalar_maxwell, Maxwell, 1.0e-05;
    validate_scalar_mean_gregorian_month, MeanGregorianMonth, 2_629_746.0;
    validate_scalar_mean_gregorian_year, MeanGregorianYear, 31_556_952.0;
    validate_scalar_mean_julian_month, MeanJulianMonth, 2_629_800.0;
    validate_scalar_mean_julian_year, MeanJulianYear, 31_557_600.0;
    validate_scalar_metric_cup, MetricCup, 0.000_24;
    validate_scalar_metric_fluid_ounce, MetricFluidOunce, 3.0e-05;
    validate_scalar_metric_tablespoon, MetricTablespoon, 1.5e-05;
    validate_scalar_metric_teaspoon, MetricTeaspoon, 5.0e-06;
    validate_scalar_mil_international, MilInternational, 2.54e-05;
    validate_scalar_mil_us, MilUS, 2.540_005_080_010_16e-05;
    validate_scalar_mile_br, MileBritish, 1_609.342_732_8;
    validate_scalar_mile_international, MileInternational, 1_609.344;
    validate_scalar_mile_us, MileUS, 1_609.347_218_694_437_3;
    validate_scalar_minim_br, MinimBritish, 5.919_388_020_833_333_4e-08;
    validate_scalar_minim_us, MinimUS, 6.161_151_992_187_5e-08;
    validate_scalar_mole, Mole, 6.0221367e+23;
    validate_scalar_month, Month, 2_629_800.0;

    validate_scalar_nautical_mile_br, NauticalMileBritish, 1_853.182_540_8;
    validate_scalar_nautical_mile_internationa, NauticalMileInternational, 1852.0;
    validate_scalar_newton, Newton, 1000.0;
    validate_scalar_ohm, Ohm, 1000.0;
    validate_scalar_oersted, Oersted, 79.577_471_545_947_67;
    validate_scalar_ounce_ap, OunceApothecaries, 31.103_476_8;
    validate_scalar_ounce_av, OunceAvoirdupois, 28.349_523_125;
    validate_scalar_ounce_m, MetricOunce, 28.0;
    validate_scalar_ounce_tr, OunceTroy, 31.103_476_8;

    validate_scalar_pace_br, PaceBritish, 0.761_999_4;
    validate_scalar_parsec, Parsec, 3.085_678e+16;
    validate_scalar_parts_per_billion, PartsPerBillion, 1.0e-09;
    validate_scalar_parts_per_million, PartsPerMillion, 1.0e-06;
    validate_scalar_parts_per_thousand, PartsPerThousand, 1.0e-03;
    validate_scalar_parts_per_trillion, PartsPerTrillion, 1.0e-012;
    validate_scalar_pascal, Pascal, 1_000.0;
    validate_scalar_peck_br, PeckBritish, 0.009_092_18;
    validate_scalar_peck_us, PeckUS, 0.008_809_767_541_72;
    validate_scalar_percent, Percent, 0.01;
    validate_scalar_permeability_of_vacuum, PermeabilityOfVacuum, 0.001_256_637_061_435_917_5;
    validate_scalar_permittivity_of_vacuum, PermittivityOfVacuum, 8.854_187_817e-15;
    validate_scalar_phot, Phot, 0.000_1;
    validate_scalar_pica, Pica, 0.004_233_333_333_333_334;
    validate_scalar_pied, Pied, 0.324_8;
    validate_scalar_pint_br, PintBritish, 0.000_568_261_25;
    validate_scalar_pint_us, PintUS, 0.000_473_176_473;
    validate_scalar_planck_constant, PlanckConstant, 6.626_075_5e-31;
    validate_scalar_point, Point, 0.000_352_777_777_777_777_76;
    validate_scalar_pouce, Pouce, 0.027_066_666_666_666_666;
    validate_scalar_pound_ap, PoundApothecaries, 373.241_721_6;
    validate_scalar_pound_av, PoundAvoirdupois, 453.592_37;
    validate_scalar_pound_tr, PoundTroy, 373.241_721_6;
    validate_scalar_pound_force, PoundForceAvoirdupois, 4448.221_615_260_5;
    validate_scalar_poise, Poise, 100.0;
    validate_scalar_printers_pica, PrintersPica, 0.004_217_517_6;
    validate_scalar_printers_point, PrintersPoint, 0.000_351_459_8;
    validate_scalar_protein_nitrogen_unit, ProteinNitrogenUnit, 1.0;
    validate_scalar_proton_mass, ProtonMass, 1.672_623_100_000_000_2e-24;

    validate_scalar_quart_br, QuartBritish, 0.001_136_522_5;
    validate_scalar_quart_us, QuartUS, 0.000_946_352_946;
    validate_scalar_queen_annes_wine_gallon, QueenAnnesWineGallonUS, 0.003_785_411_784;

    validate_scalar_ramdens_chain_us, RamdensChainUS, 30.480_060_960_121_92;
    validate_scalar_radiation_absorbed_dose, RadiationAbsorbedDose, 0.01;
    validate_scalar_radiation_equivalent_man, RadiationEquivalentMan, 0.01;
    validate_scalar_rod_br, RodBritish, 5.029_196_04;
    validate_scalar_rod_us, RodUS, 5.029_210_058_420_117;
    validate_scalar_roentgen, Roentgen, 2.58e-07;

    validate_scalar_scruple_ap, ScrupleApothecaries, 1.295_978_2;
    validate_scalar_section, Section, 2_589_998.470_319_521;
    validate_scalar_short_hundredweight_av, ShortHundredweightAvoirdupois, 45_359.237;
    validate_scalar_short_ton_av, ShortTonAvoirdupois, 907_184.74;
    validate_scalar_siemens, Siemens, 0.001;
    validate_scalar_sievert, Sievert, 1.0;
    validate_scalar_square_foot_international, SquareFootInternational, 0.092_903_04;
    validate_scalar_square_mile_us, SquareMileUS, 2_589_998.470_319_521;
    validate_scalar_square_rod_us, SquareRodUS, 25.292_953_811_714_074;
    validate_scalar_square_yard_international, SquareYardInternational, 0.836_127_36;
    validate_scalar_standard_acceleration_of_free_fall, StandardAccelerationOfFreeFall, 9.80665;
    validate_scalar_standard_atomsphere, StandardAtmosphere, 101_325_000.0;
    validate_scalar_steradian, Steradian, 1.0;
    validate_scalar_stilb, Stilb, 10_000.0;
    validate_scalar_stokes, Stokes, 0.000_1;
    validate_scalar_stone_av, StoneAvoirdupois, 6_350.293_18;
    validate_scalar_synodal_month, SynodalMonth, 2_551_442.976;

    validate_scalar_tablespoon_us, TablespoonUS, 1.478_676_478_125e-05;
    validate_scalar_teaspoon_us, TeaspoonUS, 4.928_921_593_75e-06;
    validate_scalar_tesla, Tesla, 1000.0;
    validate_scalar_the_number_pi, TheNumberPi, 3.141_592_653_589_793;
    validate_scalar_the_number_ten_for_arbitrary_powers_caret, TheNumberTenForArbitraryPowersCaret, 10.0;
    validate_scalar_the_number_ten_for_arbitrary_powers_star, TheNumberTenForArbitraryPowersStar, 10.0;
    validate_scalar_tonne, Tonne, 1_000_000.0;
    validate_scalar_township, Township, 93_239_944.931_502_76;
    validate_scalar_tropical_year, TropicalYear, 31_556_925.216;

    validate_scalar_unified_atomic_mass_unit, UnifiedAtomicMassUnit, 1.660_540_2e-24;
    validate_scalar_velocity_of_light, VelocityOfLight, 299_792_458.0;
    validate_scalar_volt, Volt, 1000.0;
    validate_scalar_watt, Watt, 1000.0;
    validate_scalar_weber, Weber, 1000.0;
    validate_scalar_week, Week, 604_800.0;
    validate_scalar_yard_br, YardBritish, 0.914_399_28;
    validate_scalar_yard_international, YardInternational, 0.914_4;
    validate_scalar_yard_us, YardUS, 0.914_401_828_803_657_6;
    validate_scalar_year, Year, 31_557_600.0;
    validate_special_scalar_degree_celsius, DegreeCelsius, 274.15;
    validate_special_scalar_degree_fahrenheit, DegreeFahrenheit, 255.927_777_777_777_8;
    validate_special_scalar_degree_reaumur, DegreeReaumur, 274.400_000_000_000_03;
    validate_special_scalar_ph, PH, 0.0;
    );

    validate_magnitudes!(
        validate_magnitude_candela, Candela, 1.0;
        validate_magnitude_coulomb, Coulomb, 1.0;
        validate_magnitude_gram, Gram, 1.0;
        validate_magnitude_kelvin, Kelvin, 1.0;
        validate_magnitude_radian, Radian, 1.0;
        validate_magnitude_second, Second, 1.0;
        validate_magnitude_acre_us, AcreUS, 1.0;
        validate_magnitude_are, Are, 1.0;
        validate_magnitude_degree, Degree, 1.0;
        validate_magnitude_degree_celsius, DegreeCelsius, 1.0;
        validate_magnitude_degree_fahrenheit, DegreeFahrenheit, 1.000_000_000_000_056_8;
        validate_magnitude_degree_reaumur, DegreeReaumur, 1.0;
        validate_magnitude_fluid_ounce_us, FluidOunceUS, 1.0;
        validate_magnitude_foot_us, FootUS, 1.0;
        validate_magnitude_foot_international, FootInternational, 1.0;
        validate_magnitude_gill_us, GillUS, 1.0;
        validate_magnitude_inch_international, InchInternational, 1.0;
        validate_magnitude_liter, Liter, 1.0;
        validate_magnitude_mole, Mole, 1.0;
        validate_magnitude_parts_per_billion, PartsPerBillion, 1.0;
        validate_magnitude_parts_per_million, PartsPerMillion, 1.0;
        validate_magnitude_parts_per_thousand, PartsPerThousand, 1.0;
        validate_magnitude_percent, Percent, 1.0;
        validate_magnitude_ph, PH, 1.0;
        validate_magnitude_pint_us, PintUS, 1.0;
        validate_magnitude_prism_diopter, PrismDiopter, 1.0;
        validate_magnitude_quart_us, QuartUS, 1.0;
        validate_magnitude_queen_annes_wine_gallon, QueenAnnesWineGallonUS, 1.0;
        validate_magnitude_rod_us, RodUS, 1.0;
        validate_magnitude_the_number_pi, TheNumberPi, 1.0;
    );

    #[test]
    fn validate_display() {
        let atom = Atom::TheNumberPi;
        assert_eq!(&atom.to_string(), "[pi]")
    }
}
