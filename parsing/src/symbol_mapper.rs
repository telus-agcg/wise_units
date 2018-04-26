use atom::Atom;
use error::Error;
use pest::iterators::Pair;
use prefix::Prefix;
use symbol_parser::Rule;

// Technically there should never be a symbol without an atom.
#[derive(Debug)]
pub struct Symbol {
    pub prefix: Option<Prefix>,
    pub atom: Option<Atom>,
}

impl Symbol {
    pub fn new() -> Self {
        Symbol {
            prefix: None,
            atom: None,
        }
    }
}

pub fn map(pair: Pair<Rule>) -> Result<Symbol, Error> {
    fn visit_pairs(pair: Pair<Rule>) -> Result<Symbol, Error> {
        let symbol = match pair.as_rule() {
            Rule::symbol => visit_symbol(pair)?,
            _ => {
                println!("visit_with_pairs: unreachable rule: {:#?}", pair);
                unreachable!()
            }
        };

        Ok(symbol)
    }

    let symbol = visit_pairs(pair)?;

    Ok(symbol)
}

fn visit_symbol(pair: Pair<Rule>) -> Result<Symbol, Error> {
    let mut symbol = Symbol::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::pri_prefix => {
                let next = inner_pair.into_inner().next().expect("Unable to get next prefix");
                let prefix = visit_prefix(next)?;

                symbol.prefix = Some(prefix);
            }
            Rule::pri_atom => {
                let next = inner_pair.into_inner().next().expect("Unable to get next atom");
                let atom = visit_atom(next)?;

                symbol.atom = Some(atom);
            }
            _ => {
                println!("visit_symbol: unreachable rule: {:#?}", inner_pair);
                unreachable!()
            },
        }
    }

    Ok(symbol)
}

fn visit_atom(pair: Pair<Rule>) -> Result<Atom, Error> {
    let atom = match pair.as_rule() {
        // Base units first.
        Rule::coulomb                         => Atom::Coulomb,
        Rule::kelvin                          => Atom::Kelvin,
        Rule::pri_meter   | Rule::sec_meter   => Atom::Meter,
        Rule::pri_candela | Rule::sec_candela => Atom::Candela,
        Rule::pri_gram    | Rule::sec_gram    => Atom::Gram,
        Rule::pri_radian  | Rule::sec_radian  => Atom::Radian,
        Rule::pri_second  | Rule::sec_second  => Atom::Second,

        // Derived units last.
        Rule::pri_acre_br                | Rule::sec_acre_br                => Atom::AcreBR,
        Rule::pri_acre_us                | Rule::sec_acre_us                => Atom::AcreUS,
        Rule::ampere                                                        => Atom::Ampere,
        Rule::pri_are                    | Rule::sec_are                    => Atom::Are,
        Rule::pri_astronomic_unit        | Rule::sec_astronomic_unit        => Atom::AstronomicUnit,
        Rule::pri_unified_atom_mass_unit | Rule::sec_unified_atom_mass_unit => Atom::AtomicMassUnit,

        Rule::pri_bar                | Rule::sec_bar                => Atom::Bar,
        Rule::pri_barrel_us          | Rule::sec_barrel_us          => Atom::BarrelUS,
        Rule::pri_becquerel          | Rule::sec_becquerel          => Atom::Becquerel,
        Rule::pri_biot               | Rule::sec_biot               => Atom::Biot,
        Rule::pri_board_foot_i       | Rule::sec_board_foot_i       => Atom::BoardFootInternational,
        Rule::pri_boltzmann_constant | Rule::sec_boltzmann_constant => Atom::BoltzmannConstant,
        Rule::pri_btu_at_39f         | Rule::sec_btu_at_39f         => Atom::BtuAt39F,
        Rule::pri_bushel_br          | Rule::sec_bushel_br          => Atom::BushelBR,
        Rule::pri_bushel_us          | Rule::sec_bushel_us          => Atom::BushelUS,

        Rule::pri_calorie_at_15c | Rule::sec_calorie_at_15c => Atom::CalorieAt15C,
        Rule::pri_calorie_at_20c | Rule::sec_calorie_at_20c => Atom::CalorieAt20C,
        Rule::pri_calorie_it     | Rule::sec_calorie_it     => Atom::CalorieIT,
        Rule::pri_calorie_th     | Rule::sec_calorie_th     => Atom::CalorieTH,
        Rule::pri_calorie_m      | Rule::sec_calorie_m      => Atom::CalorieM,
        Rule::pri_calorie        | Rule::sec_calorie        => Atom::Calorie,
        Rule::pri_calorie_nutrition_label | Rule::sec_calorie_nutrition_label => Atom::CalorieNutritionLabel,
        Rule::pri_cicero         | Rule::sec_cicero         => Atom::Cicero,
        Rule::pri_circular_mil_i | Rule::sec_circular_mil_i => Atom::CircularMilInternational,
        Rule::pri_cord_i         | Rule::sec_cord_i         => Atom::CordInternational,
        Rule::pri_cord_us        | Rule::sec_cord_us        => Atom::CordUS,
        Rule::pri_cubic_foot_i   | Rule::sec_cubic_foot_i   => Atom::CubicFootInternational,
        Rule::pri_cubic_inch_i   | Rule::sec_cubic_inch_i   => Atom::CubicInchInternational,
        Rule::pri_cubic_yard_i   | Rule::sec_cubic_yard_i   => Atom::CubicYardInternational,
        Rule::pri_cup_us         | Rule::sec_cup_us         => Atom::CupUS,
        Rule::pri_curie          | Rule::sec_curie          => Atom::Curie,

        Rule::pri_day               | Rule::sec_day               => Atom::Day,
        Rule::pri_degree_celsius    | Rule::sec_degree_celsius    => Atom::DegreeCelsius,
        Rule::pri_degree_fahrenheit | Rule::sec_degree_fahrenheit => Atom::DegreeFahrenheit,
        Rule::degree_rankine                                      => Atom::DegreeRankine,
        Rule::degree_reaumur                                      => Atom::DegreeReaumur,
        Rule::minute_angle                                        => Atom::DegreeMinute,
        Rule::second_angle                                        => Atom::DegreeSecond,
        Rule::pri_degree       | Rule::sec_degree                 => Atom::Degree,
        Rule::pri_didot        | Rule::sec_didot                  => Atom::Didot,
        Rule::pri_dram_ap      | Rule::sec_dram_ap                => Atom::DramAP,
        Rule::pri_dram_av      | Rule::sec_dram_av                => Atom::DramAV,
        Rule::pri_dry_pint_us  | Rule::sec_dry_pint_us            => Atom::DryPintUS,
        Rule::pri_dry_quart_us | Rule::sec_dry_quart_us           => Atom::DryQuartUS,
        Rule::pri_dyne         | Rule::sec_dyne                   => Atom::Dyne,

        Rule::pri_electronvolt      | Rule::sec_electronvolt      => Atom::Electronvolt,
        Rule::pri_electron_mass     | Rule::sec_electron_mass     => Atom::ElectronMass,
        Rule::pri_elementary_charge | Rule::sec_elementary_charge => Atom::ElementaryCharge,
        // "eq"                     | "EQ"                        => Atom::Equivalents,
        Rule::pri_erg               | Rule::sec_erg               => Atom::Erg,

        Rule::farad                                         => Atom::Farad,
        Rule::pri_fathom_br      | Rule::sec_fathom_br      => Atom::FathomBR,
        Rule::pri_fathom_i       | Rule::sec_fathom_i       => Atom::FathomInternational,
        Rule::pri_fathom_us      | Rule::sec_fathom_us      => Atom::FathomUS,
        Rule::pri_fluid_dram_br  | Rule::sec_fluid_dram_br  => Atom::FluidDramBR,
        Rule::pri_fluid_dram_us  | Rule::sec_fluid_dram_us  => Atom::FluidDramUS,
        Rule::pri_fluid_ounce_br | Rule::sec_fluid_ounce_br => Atom::FluidOunceBR,
        Rule::pri_fluid_ounce_us | Rule::sec_fluid_ounce_us => Atom::FluidOunceUS,
        Rule::pri_foot_br        | Rule::sec_foot_br        => Atom::FootBR,
        Rule::pri_foot_i         | Rule::sec_foot_i         => Atom::FootInternational,
        Rule::pri_foot_us        | Rule::sec_foot_us        => Atom::FootUS,
        Rule::pri_furlong_us     | Rule::sec_furlong_us     => Atom::FurlongUS,

        Rule::pri_gal              | Rule::sec_gal              => Atom::Gal,
        Rule::pri_gallon_br        | Rule::sec_gallon_br        => Atom::GallonBR,
        Rule::pri_gauss            | Rule::sec_gauss            => Atom::Gauss,
        Rule::pri_gilbert          | Rule::sec_gilbert          => Atom::Gilbert,
        Rule::pri_gill_br          | Rule::sec_gill_br          => Atom::GillBR,
        Rule::pri_gill_us          | Rule::sec_gill_us          => Atom::GillUS,
        Rule::pri_gon              | Rule::sec_gon              => Atom::Gon,
        Rule::pri_gram_force       | Rule::sec_gram_force       => Atom::GramForce,
        // "g%"                    | "G%"                       => Atom::GramPercent,
        Rule::pri_grain            | Rule::sec_grain            => Atom::Grain,
        Rule::pri_gray             | Rule::sec_gray             => Atom::Gray,
        Rule::pri_gunters_chain_br | Rule::sec_gunters_chain_br => Atom::GuntersChainBR,
        Rule::pri_gunters_chain_us | Rule::sec_gunters_chain_us => Atom::GuntersChainUS,

        Rule::pri_hand_i | Rule::sec_hand_i => Atom::HandInternational,
        Rule::pri_hertz | Rule::sec_hertz => Atom::Hertz,
        Rule::henry => Atom::Henry,
        Rule::pri_historical_winchester_gallon
            | Rule::sec_historical_winchester_gallon => Atom::HistoricalWinchesterGallon,
        // "[HP]"                   => Atom::Horsepower,
        Rule::pri_hour | Rule::sec_hour => Atom::Hour,

        Rule::pri_inch_br | Rule::sec_inch_br => Atom::InchBR,
        Rule::pri_inch_i  | Rule::sec_inch_i  => Atom::InchInternational,
        Rule::pri_inch_us | Rule::sec_inch_us => Atom::InchUS,

        Rule::joule => Atom::Joule,

        Rule::pri_kayser  | Rule::sec_kayser  => Atom::Kayser,
        Rule::pri_knot_br | Rule::sec_knot_br => Atom::KnotBR,
        Rule::pri_knot_i  | Rule::sec_knot_i  => Atom::KnotInternational,

        Rule::pri_lambert                   | Rule::sec_lambert                   => Atom::Lambert,
        Rule::pri_long_hundredweight_av     | Rule::sec_long_hundredweight_av     => Atom::LongHundredweightAV,
        Rule::pri_long_ton_av               | Rule::sec_long_ton_av               => Atom::LongTonAV,
        Rule::pri_light_year                | Rule::sec_light_year                => Atom::LightYear,
        Rule::pri_ligne                     | Rule::sec_ligne                     => Atom::Ligne,
        Rule::pri_line                      | Rule::sec_line                      => Atom::Line,
        Rule::pri_link_for_gunters_chain_br | Rule::sec_link_for_gunters_chain_br => Atom::LinkForGuntersChainBR,
        Rule::pri_link_for_gunters_chain_us | Rule::sec_link_for_gunters_chain_us => Atom::LinkForGuntersChainUS,
        Rule::pri_link_for_ramdens_chain_us | Rule::sec_link_for_ramdens_chain_us => Atom::LinkForRamdensChainUS,
        Rule::pri_liter                     | Rule::sec_liter                     => Atom::Liter,
        Rule::pri_lumen                     | Rule::sec_lumen                     => Atom::Lumen,
        Rule::pri_lux                       | Rule::sec_lux                       => Atom::Lux,

        Rule::pri_maxwell              | Rule::sec_maxwell              => Atom::Maxwell,
        Rule::pri_mean_gregorian_month | Rule::sec_mean_gregorian_month => Atom::MeanGregorianMonth,
        Rule::pri_mean_gregorian_year  | Rule::sec_mean_gregorian_year  => Atom::MeanGregorianYear,
        Rule::pri_mean_julian_month    | Rule::sec_mean_julian_month    => Atom::MeanJulianMonth,
        Rule::pri_mean_julian_year     | Rule::sec_mean_julian_year     => Atom::MeanJulianYear,
        Rule::pri_cup_m                | Rule::sec_cup_m                => Atom::CupM,
        // "[foz_m]"                   | "[FOZ_M]"                      => Atom::FluidOunceM,
        Rule::pri_fluid_ounce_m        | Rule::sec_fluid_ounce_m        => Atom::FluidOunceM,
        Rule::pri_tablespoon_m         | Rule::sec_tablespoon_m         => Atom::TablespoonM,
        Rule::pri_teaspoon_m           | Rule::sec_teaspoon_m           => Atom::TeaspoonM,
        Rule::pri_mil_i                | Rule::sec_mil_i                => Atom::MilInternational,
        Rule::pri_mil_us               | Rule::sec_mil_us               => Atom::MilUS,
        Rule::pri_mile_br              | Rule::sec_mile_br              => Atom::MileBR,
        Rule::pri_mile_i               | Rule::sec_mile_i               => Atom::MileInternational,
        Rule::pri_mile_us              | Rule::sec_mile_us              => Atom::MileUS,
        Rule::pri_minute_time          | Rule::sec_minute_time          => Atom::Minute,
        Rule::pri_minim_br             | Rule::sec_minim_br             => Atom::MinimBR,
        Rule::pri_minim_us             | Rule::sec_minim_us             => Atom::MinimUS,
        Rule::pri_mole                 | Rule::sec_mole                 => Atom::Mole,
        Rule::pri_month                | Rule::sec_month                => Atom::Month,

        Rule::pri_nautical_mile_br | Rule::sec_nautical_mile_br => Atom::NauticalMileBR,
        Rule::pri_nautical_mile_i | Rule::sec_nautical_mile_i => Atom::NauticalMileInternational,
        Rule::newton => Atom::Newton,
        Rule::pri_newtonian_constant_of_gravitation | Rule::sec_newtonian_constant_of_gravitation => Atom::NewtonianConstantOfGravitation,
        Rule::pri_ohm                               | Rule::sec_ohm => Atom::Ohm,
        Rule::pri_oersted                           | Rule::sec_oersted => Atom::Oersted,
        Rule::pri_ounce_ap                          | Rule::sec_ounce_ap => Atom::OunceAP,
        Rule::pri_ounce_av                          | Rule::sec_ounce_av => Atom::OunceAV,
        Rule::pri_ounce_m                           | Rule::sec_ounce_m => Atom::OunceM,
        Rule::pri_ounce_tr                          | Rule::sec_ounce_tr => Atom::OunceTR,

        Rule::pri_pace_br            | Rule::sec_pace_br                    => Atom::PaceBR,
        Rule::pri_parsec             | Rule::sec_parsec                     => Atom::Parsec,
        Rule::pri_parts_per_billion  | Rule::sec_parts_per_billion          => Atom::PartsPerBillion,
        Rule::pri_parts_per_million  | Rule::sec_parts_per_million          => Atom::PartsPerMillion,
        Rule::pri_parts_per_thousand | Rule::sec_parts_per_thousand         => Atom::PartsPerThousand,
        Rule::pri_parts_per_trillion | Rule::sec_parts_per_trillion         => Atom::PartsPerTrillion,
        Rule::pri_pascal             | Rule::sec_pascal                     => Atom::Pascal,
        Rule::pri_peck_br            | Rule::sec_peck_br                    => Atom::PeckBR,
        Rule::pri_peck_us            | Rule::sec_peck_us                    => Atom::PeckUS,
        Rule::percent                                                       => Atom::Percent,
        Rule::pri_permeability_of_vacuum | Rule::sec_permeability_of_vacuum => Atom::PermeabilityOfVacuum,
        Rule::pri_permittivity_of_vacuum | Rule::sec_permittivity_of_vacuum => Atom::PermittivityOfVacuum,
        Rule::pri_pennyweight_tr | Rule::sec_pennyweight_tr                 => Atom::PennyweightTR,
        // "[pH]"      | "[PH]"                                             => Atom::PH,
        Rule::pri_phot | Rule::sec_phot                                     => Atom::Phot,
        Rule::pri_pica | Rule::sec_pica                                     => Atom::Pica,
        Rule::pri_pied | Rule::sec_pied                                     => Atom::Pied,
        Rule::pri_pint_br | Rule::sec_pint_br                               => Atom::PintBR,
        Rule::pri_pint_us | Rule::sec_pint_us                               => Atom::PintUS,
        Rule::pri_planck_constant | Rule::sec_planck_constant               => Atom::PlanckConstant,
        Rule::pri_point | Rule::sec_point                                   => Atom::Point,
        Rule::pri_pouce | Rule::sec_pouce                                   => Atom::Pouce,
        Rule::pri_pound_ap | Rule::sec_pound_ap                             => Atom::PoundAP,
        Rule::pri_pound_av | Rule::sec_pound_av                             => Atom::PoundAV,
        Rule::pri_pound_tr | Rule::sec_pound_tr                             => Atom::PoundTR,
        Rule::pri_pound_force | Rule::sec_pound_force                       => Atom::PoundForce,
        Rule::poise                                                         => Atom::Poise,
        // "[p'diop]"  | "[P'DIOP]"                                         => Atom::PrismDiopter,
        // "[PNU]"                                                          => Atom::ProteinNitrogenUnit,
        Rule::pri_proton_mass | Rule::sec_proton_mass                       => Atom::ProtonMass,
        Rule::pri_printers_pica | Rule::sec_printers_pica                   => Atom::PrintersPica,
        Rule::pri_printers_point | Rule::sec_printers_point                 => Atom::PrintersPoint,

        Rule::pri_quart_br | Rule::sec_quart_br                                 => Atom::QuartBR,
        Rule::pri_quart_us | Rule::sec_quart_us                                 => Atom::QuartUS,
        Rule::pri_queen_annes_wine_gallon | Rule::sec_queen_annes_wine_gallon   => Atom::QueenAnnesWineGallon,
        Rule::pri_ramdens_chain_us | Rule::sec_ramdens_chain_us                 => Atom::RamdensChainUS,
        Rule::pri_radiation_absorbed_dose | Rule::sec_radiation_absorbed_dose   => Atom::RadiationAbsorbedDose,
        Rule::pri_radiation_equivalent_man | Rule::sec_radiation_equivalent_man => Atom::RadiationEquivalentMan,
        Rule::pri_rod_br | Rule::sec_rod_br                                     => Atom::RodBR,
        Rule::pri_rod_us | Rule::sec_rod_us                                     => Atom::RodUS,
        Rule::pri_roentgen | Rule::sec_roentgen                                 => Atom::Roentgen,

        Rule::pri_scruple | Rule::sec_scruple => Atom::ScrupleAP,
        Rule::pri_section | Rule::sec_section => Atom::Section,
        Rule::pri_short_hundredweight_av | Rule::sec_short_hundredweight_av => Atom::ShortHundredweightAV,
        Rule::pri_short_ton_av | Rule::sec_short_ton_av => Atom::ShortTonAV,
        Rule::pri_siemens | Rule::sec_siemens => Atom::Siemens,
        Rule::pri_sievert | Rule::sec_sievert => Atom::Sievert,
        Rule::pri_square_foot_i | Rule::sec_square_foot_i => Atom::SquareFootInternational,
        Rule::pri_square_inch_i | Rule::sec_square_inch_i => Atom::SquareInchInternational,
        Rule::pri_square_mile_us | Rule::sec_square_mile_us => Atom::SquareMileUS,
        Rule::pri_square_rod_us | Rule::sec_square_rod_us => Atom::SquareRodUS,
        Rule::pri_square_yard_i | Rule::sec_square_yard_i => Atom::SquareYardInternational,
        Rule::pri_standard_acceleration_of_free_fall | Rule::sec_standard_acceleration_of_free_fall => Atom::StandardAccelerationOfFreeFall,
        Rule::pri_standard_atmosphere | Rule::sec_standard_atmosphere => Atom::StandardAtmosphere,
        Rule::pri_steradian | Rule::sec_steradian => Atom::Steradian,
        Rule::pri_stilb | Rule::sec_stilb => Atom::Stilb,
        Rule::pri_stokes | Rule::sec_stokes => Atom::Stokes,
        Rule::pri_stone_av | Rule::sec_stone_av => Atom::StoneAV,
        Rule::pri_synodal_month | Rule::sec_synodal_month => Atom::SynodalMonth,

        Rule::pri_tablespoon_us | Rule::sec_tablespoon_us => Atom::TablespoonUS,
        Rule::pri_teaspoon_us | Rule::sec_teaspoon_us     => Atom::TeaspoonUS,
        Rule::tesla                                       => Atom::Tesla,
        Rule::pri_pi | Rule::sec_pi                       => Atom::TheNumberPi,
        Rule::ten_for_arbitrary_powers_caret              => Atom::TheNumberTenForArbitraryPowersCaret,
        Rule::ten_for_arbitrary_powers_star               => Atom::TheNumberTenForArbitraryPowersStar,
        Rule::the_unity                                   => Atom::TheUnity,
        Rule::pri_tonne | Rule::sec_tonne                 => Atom::Tonne,
        Rule::pri_township | Rule::sec_township           => Atom::Township,
        Rule::pri_tropical_year | Rule::sec_tropical_year => Atom::TropicalYear,

        Rule::pri_velocity_of_light | Rule::sec_velocity_of_light => Atom::VelocityOfLight,
        Rule::volt                                                => Atom::Volt,
        Rule::watt                                                => Atom::Watt,
        Rule::pri_weber | Rule::sec_weber                         => Atom::Weber,
        Rule::pri_week | Rule::sec_week                           => Atom::Week,
        Rule::pri_yard_br | Rule::sec_yard_br                     => Atom::YardBR,
        Rule::pri_yard_i | Rule::sec_yard_i                       => Atom::YardInternational,
        Rule::pri_yard_us | Rule::sec_yard_us                     => Atom::YardUS,
        Rule::pri_year | Rule::sec_year                           => Atom::Year,

        _ => {
            return Err(Error::UnknownUnitString(pair.as_str().to_string()))
        },
    };

    Ok(atom)
}

fn visit_prefix(pair: Pair<Rule>) -> Result<Prefix, Error> {
    let prefix = match pair.as_rule() {
        Rule::pri_atto  | Rule::sec_atto  => Prefix::Atto,
        Rule::pri_centi | Rule::sec_centi => Prefix::Centi,
        Rule::pri_deci  | Rule::sec_deci  => Prefix::Deci,
        Rule::pri_deka  | Rule::sec_deka  => Prefix::Deka,
        Rule::pri_exa   | Rule::sec_exa   => Prefix::Exa,
        Rule::pri_femto | Rule::sec_femto => Prefix::Femto,
        Rule::pri_gibi  | Rule::sec_gibi  => Prefix::Gibi,
        Rule::pri_giga  | Rule::sec_giga  => Prefix::Giga,
        Rule::pri_hecto | Rule::sec_hecto => Prefix::Hecto,
        Rule::pri_kilo  | Rule::sec_kilo  => Prefix::Kilo,
        Rule::pri_mebi  | Rule::sec_mebi  => Prefix::Mebi,
        Rule::pri_mega  | Rule::sec_mega  => Prefix::Mega,
        Rule::pri_micro | Rule::sec_micro => Prefix::Micro,
        Rule::pri_milli | Rule::sec_milli => Prefix::Milli,
        Rule::pri_nano  | Rule::sec_nano  => Prefix::Nano,
        Rule::pri_peta  | Rule::sec_peta  => Prefix::Peta,
        Rule::pri_tebi  | Rule::sec_tebi  => Prefix::Tebi,
        Rule::pri_tera  | Rule::sec_tera  => Prefix::Tera,
        Rule::pri_yocto | Rule::sec_yocto => Prefix::Yocto,
        Rule::pri_yotta | Rule::sec_yotta => Prefix::Yotta,
        Rule::pri_zepto | Rule::sec_zepto => Prefix::Zepto,
        Rule::pri_zetta | Rule::sec_zetta => Prefix::Zetta,
        _ => {
            return Err(Error::UnknownUnitString(pair.as_str().to_string()))
        }
    };

    Ok(prefix)
}
