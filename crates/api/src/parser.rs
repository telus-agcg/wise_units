#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

// Because long numbers are generated, there's no way (that I know of) to
// generate them using underscores (to make them pass the clippy lint).
#[cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::unreadable_literal,
        clippy::match_same_arms,
        clippy::too_many_lines,
        clippy::non_ascii_literal
    )
)]
#[allow(clippy::non_ascii_literal)]
pub mod property;

mod symbols;

mod annotation_composition;
mod error;
mod prefix;
pub(crate) mod term;
mod terms;

use pest::{iterators::Pair, Parser};

use crate::Atom;

pub use self::{
    annotation_composition::AnnotationComposition, error::Error, prefix::Prefix,
    property::Property, term::Term,
};

use self::{
    symbols::symbol_parser::Rule as SymbolRule,
    terms::term_parser::{Rule as TermRule, TermParser},
};

#[inline]
pub(crate) fn parse(expression: &str) -> Result<Vec<Term>, Error> {
    match TermParser::parse(TermRule::main_term, expression) {
        Ok(pairs) => Ok(terms::mapper::map(pairs)?),
        Err(_) => Err(Error::UnknownUnitString(expression.to_string())),
    }
}

trait Visit<R> {
    fn visit(pair: Pair<'_, R>) -> Result<Self, Error>
    where
        Self: Sized;
}

// TODO: Move to atom_generator.
//
#[allow(clippy::too_many_lines)]
impl Visit<SymbolRule> for Atom {
    fn visit(pair: Pair<'_, SymbolRule>) -> Result<Self, Error> {
        let atom = match pair.as_rule() {
            SymbolRule::pri_acre_british | SymbolRule::sec_acre_british => Self::AcreBritish,
            SymbolRule::pri_acre_us | SymbolRule::sec_acre_us => Self::AcreUS,
            SymbolRule::pri_allergen_unit | SymbolRule::sec_allergen_unit => Self::AllergenUnit,
            SymbolRule::pri_allergen_unit_for_ambrosia_artemisiifolia
            | SymbolRule::sec_allergen_unit_for_ambrosia_artemisiifolia => {
                Self::AllergenUnitForAmbrosiaArtemisiifolia
            }
            SymbolRule::pri_ampere | SymbolRule::sec_ampere => Self::Ampere,
            SymbolRule::pri_angstrom | SymbolRule::sec_angstrom => Self::Angstrom,
            SymbolRule::pri_anti_factor_xa_unit | SymbolRule::sec_anti_factor_xa_unit => {
                Self::AntiFactorXaUnit
            }
            SymbolRule::pri_apl_unit | SymbolRule::sec_apl_unit => Self::AplUnit,
            SymbolRule::pri_arbitrary_unit | SymbolRule::sec_arbitrary_unit => Self::ArbitraryUnit,
            SymbolRule::pri_are | SymbolRule::sec_are => Self::Are,
            SymbolRule::pri_astronomic_unit | SymbolRule::sec_astronomic_unit => {
                Self::AstronomicUnit
            }
            SymbolRule::pri_bar | SymbolRule::sec_bar => Self::Bar,
            SymbolRule::pri_barn | SymbolRule::sec_barn => Self::Barn,
            SymbolRule::pri_barrel_us | SymbolRule::sec_barrel_us => Self::BarrelUS,
            SymbolRule::pri_baud | SymbolRule::sec_baud => Self::Baud,
            SymbolRule::pri_becquerel | SymbolRule::sec_becquerel => Self::Becquerel,
            SymbolRule::pri_bel | SymbolRule::sec_bel => Self::Bel,
            SymbolRule::pri_bel10_nanovolt | SymbolRule::sec_bel10_nanovolt => Self::Bel10Nanovolt,
            SymbolRule::pri_bel_kilowatt | SymbolRule::sec_bel_kilowatt => Self::BelKilowatt,
            SymbolRule::pri_bel_microvolt | SymbolRule::sec_bel_microvolt => Self::BelMicrovolt,
            SymbolRule::pri_bel_millivolt | SymbolRule::sec_bel_millivolt => Self::BelMillivolt,
            SymbolRule::pri_bel_sound_pressure | SymbolRule::sec_bel_sound_pressure => {
                Self::BelSoundPressure
            }
            SymbolRule::pri_bel_volt | SymbolRule::sec_bel_volt => Self::BelVolt,
            SymbolRule::pri_bel_watt | SymbolRule::sec_bel_watt => Self::BelWatt,
            SymbolRule::pri_bethesda_unit | SymbolRule::sec_bethesda_unit => Self::BethesdaUnit,
            SymbolRule::pri_bioequivalent_allergen_unit
            | SymbolRule::sec_bioequivalent_allergen_unit => Self::BioequivalentAllergenUnit,
            SymbolRule::pri_biot | SymbolRule::sec_biot => Self::Biot,
            SymbolRule::pri_bit | SymbolRule::sec_bit => Self::Bit,
            SymbolRule::pri_bit_logarithmus_dualis | SymbolRule::sec_bit_logarithmus_dualis => {
                Self::BitLogarithmusDualis
            }
            SymbolRule::pri_board_foot_international | SymbolRule::sec_board_foot_international => {
                Self::BoardFootInternational
            }
            SymbolRule::pri_bodansky_unit | SymbolRule::sec_bodansky_unit => Self::BodanskyUnit,
            SymbolRule::pri_boltzmann_constant | SymbolRule::sec_boltzmann_constant => {
                Self::BoltzmannConstant
            }
            SymbolRule::pri_british_thermal_unit | SymbolRule::sec_british_thermal_unit => {
                Self::BritishThermalUnit
            }
            SymbolRule::pri_british_thermal_unit_at39_f
            | SymbolRule::sec_british_thermal_unit_at39_f => Self::BritishThermalUnitAt39F,
            SymbolRule::pri_british_thermal_unit_at59_f
            | SymbolRule::sec_british_thermal_unit_at59_f => Self::BritishThermalUnitAt59F,
            SymbolRule::pri_british_thermal_unit_at60_f
            | SymbolRule::sec_british_thermal_unit_at60_f => Self::BritishThermalUnitAt60F,
            SymbolRule::pri_bushel_british | SymbolRule::sec_bushel_british => Self::BushelBritish,
            SymbolRule::pri_bushel_us | SymbolRule::sec_bushel_us => Self::BushelUS,
            SymbolRule::pri_byte | SymbolRule::sec_byte => Self::Byte,
            SymbolRule::pri_calorie | SymbolRule::sec_calorie => Self::Calorie,
            SymbolRule::pri_calorie_at15_c | SymbolRule::sec_calorie_at15_c => Self::CalorieAt15C,
            SymbolRule::pri_calorie_at20_c | SymbolRule::sec_calorie_at20_c => Self::CalorieAt20C,
            SymbolRule::pri_candela | SymbolRule::sec_candela => Self::Candela,
            SymbolRule::pri_carat_of_gold_alloys | SymbolRule::sec_carat_of_gold_alloys => {
                Self::CaratOfGoldAlloys
            }
            SymbolRule::pri_cell_culture_infectious_dose
            | SymbolRule::sec_cell_culture_infectious_dose => Self::CellCultureInfectiousDose,
            SymbolRule::pri_charriere | SymbolRule::sec_charriere => Self::Charriere,
            SymbolRule::pri_cicero | SymbolRule::sec_cicero => Self::Cicero,
            SymbolRule::pri_circle | SymbolRule::sec_circle => Self::Circle,
            SymbolRule::pri_circular_mil_international
            | SymbolRule::sec_circular_mil_international => Self::CircularMilInternational,
            SymbolRule::pri_colony_forming_units | SymbolRule::sec_colony_forming_units => {
                Self::ColonyFormingUnits
            }
            SymbolRule::pri_cord_international | SymbolRule::sec_cord_international => {
                Self::CordInternational
            }
            SymbolRule::pri_cord_us | SymbolRule::sec_cord_us => Self::CordUS,
            SymbolRule::pri_coulomb | SymbolRule::sec_coulomb => Self::Coulomb,
            SymbolRule::pri_cubic_foot_international | SymbolRule::sec_cubic_foot_international => {
                Self::CubicFootInternational
            }
            SymbolRule::pri_cubic_inch_international | SymbolRule::sec_cubic_inch_international => {
                Self::CubicInchInternational
            }
            SymbolRule::pri_cubic_yard_international | SymbolRule::sec_cubic_yard_international => {
                Self::CubicYardInternational
            }
            SymbolRule::pri_cup_us | SymbolRule::sec_cup_us => Self::CupUS,
            SymbolRule::pri_curie | SymbolRule::sec_curie => Self::Curie,
            SymbolRule::pri_d_antigen_unit | SymbolRule::sec_d_antigen_unit => Self::DAntigenUnit,
            SymbolRule::pri_day | SymbolRule::sec_day => Self::Day,
            SymbolRule::pri_degree | SymbolRule::sec_degree => Self::Degree,
            SymbolRule::pri_degree_celsius | SymbolRule::sec_degree_celsius => Self::DegreeCelsius,
            SymbolRule::pri_degree_fahrenheit | SymbolRule::sec_degree_fahrenheit => {
                Self::DegreeFahrenheit
            }
            SymbolRule::pri_degree_rankine | SymbolRule::sec_degree_rankine => Self::DegreeRankine,
            SymbolRule::pri_degree_reaumur | SymbolRule::sec_degree_reaumur => Self::DegreeReaumur,
            SymbolRule::pri_denier | SymbolRule::sec_denier => Self::Denier,
            SymbolRule::pri_didot | SymbolRule::sec_didot => Self::Didot,
            SymbolRule::pri_diopter | SymbolRule::sec_diopter => Self::Diopter,
            SymbolRule::pri_dram_apothecaries | SymbolRule::sec_dram_apothecaries => {
                Self::DramApothecaries
            }
            SymbolRule::pri_dram_avoirdupois | SymbolRule::sec_dram_avoirdupois => {
                Self::DramAvoirdupois
            }
            SymbolRule::pri_drop | SymbolRule::sec_drop => Self::Drop,
            SymbolRule::pri_dry_pint_us | SymbolRule::sec_dry_pint_us => Self::DryPintUS,
            SymbolRule::pri_dry_quart_us | SymbolRule::sec_dry_quart_us => Self::DryQuartUS,
            SymbolRule::pri_dye_unit | SymbolRule::sec_dye_unit => Self::DyeUnit,
            SymbolRule::pri_dyne | SymbolRule::sec_dyne => Self::Dyne,
            SymbolRule::pri_ehrlich_unit | SymbolRule::sec_ehrlich_unit => Self::EhrlichUnit,
            SymbolRule::pri_electron_mass | SymbolRule::sec_electron_mass => Self::ElectronMass,
            SymbolRule::pri_electronvolt | SymbolRule::sec_electronvolt => Self::Electronvolt,
            SymbolRule::pri_elementary_charge | SymbolRule::sec_elementary_charge => {
                Self::ElementaryCharge
            }
            SymbolRule::pri_elisa_unit | SymbolRule::sec_elisa_unit => Self::ElisaUnit,
            SymbolRule::pri_embryo_infectious_dose | SymbolRule::sec_embryo_infectious_dose => {
                Self::EmbryoInfectiousDose
            }
            SymbolRule::pri_equivalents | SymbolRule::sec_equivalents => Self::Equivalents,
            SymbolRule::pri_erg | SymbolRule::sec_erg => Self::Erg,
            SymbolRule::pri_farad | SymbolRule::sec_farad => Self::Farad,
            SymbolRule::pri_fathom_british | SymbolRule::sec_fathom_british => Self::FathomBritish,
            SymbolRule::pri_fathom_international | SymbolRule::sec_fathom_international => {
                Self::FathomInternational
            }
            SymbolRule::pri_fathom_us | SymbolRule::sec_fathom_us => Self::FathomUS,
            SymbolRule::pri_fibrinogen_equivalent_unit
            | SymbolRule::sec_fibrinogen_equivalent_unit => Self::FibrinogenEquivalentUnit,
            SymbolRule::pri_fluid_dram_british | SymbolRule::sec_fluid_dram_british => {
                Self::FluidDramBritish
            }
            SymbolRule::pri_fluid_dram_us | SymbolRule::sec_fluid_dram_us => Self::FluidDramUS,
            SymbolRule::pri_fluid_ounce_british | SymbolRule::sec_fluid_ounce_british => {
                Self::FluidOunceBritish
            }
            SymbolRule::pri_fluid_ounce_us | SymbolRule::sec_fluid_ounce_us => Self::FluidOunceUS,
            SymbolRule::pri_focus_forming_units | SymbolRule::sec_focus_forming_units => {
                Self::FocusFormingUnits
            }
            SymbolRule::pri_foot_british | SymbolRule::sec_foot_british => Self::FootBritish,
            SymbolRule::pri_foot_international | SymbolRule::sec_foot_international => {
                Self::FootInternational
            }
            SymbolRule::pri_foot_us | SymbolRule::sec_foot_us => Self::FootUS,
            SymbolRule::pri_furlong_us | SymbolRule::sec_furlong_us => Self::FurlongUS,
            SymbolRule::pri_gal | SymbolRule::sec_gal => Self::Gal,
            SymbolRule::pri_gallon_british | SymbolRule::sec_gallon_british => Self::GallonBritish,
            SymbolRule::pri_gauss | SymbolRule::sec_gauss => Self::Gauss,
            SymbolRule::pri_gilbert | SymbolRule::sec_gilbert => Self::Gilbert,
            SymbolRule::pri_gill_british | SymbolRule::sec_gill_british => Self::GillBritish,
            SymbolRule::pri_gill_us | SymbolRule::sec_gill_us => Self::GillUS,
            SymbolRule::pri_gon | SymbolRule::sec_gon => Self::Gon,
            SymbolRule::pri_gpl_unit | SymbolRule::sec_gpl_unit => Self::GplUnit,
            SymbolRule::pri_grain | SymbolRule::sec_grain => Self::Grain,
            SymbolRule::pri_gram | SymbolRule::sec_gram => Self::Gram,
            SymbolRule::pri_gram_force | SymbolRule::sec_gram_force => Self::GramForce,
            SymbolRule::pri_gram_percent | SymbolRule::sec_gram_percent => Self::GramPercent,
            SymbolRule::pri_gray | SymbolRule::sec_gray => Self::Gray,
            SymbolRule::pri_gunters_chain_british | SymbolRule::sec_gunters_chain_british => {
                Self::GuntersChainBritish
            }
            SymbolRule::pri_gunters_chain_us | SymbolRule::sec_gunters_chain_us => {
                Self::GuntersChainUS
            }
            SymbolRule::pri_hand_international | SymbolRule::sec_hand_international => {
                Self::HandInternational
            }
            SymbolRule::pri_henry | SymbolRule::sec_henry => Self::Henry,
            SymbolRule::pri_hertz | SymbolRule::sec_hertz => Self::Hertz,
            SymbolRule::pri_high_power_field | SymbolRule::sec_high_power_field => {
                Self::HighPowerField
            }
            SymbolRule::pri_historical_winchester_gallon
            | SymbolRule::sec_historical_winchester_gallon => Self::HistoricalWinchesterGallon,
            SymbolRule::pri_homeopathic_potency_of_centesimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_centesimal_hahnemannian_series => {
                Self::HomeopathicPotencyOfCentesimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_centesimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_centesimal_korsakovian_series => {
                Self::HomeopathicPotencyOfCentesimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_centesimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_centesimal_series_retired => {
                Self::HomeopathicPotencyOfCentesimalSeriesRetired
            }
            SymbolRule::pri_homeopathic_potency_of_decimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_decimal_hahnemannian_series => {
                Self::HomeopathicPotencyOfDecimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_decimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_decimal_korsakovian_series => {
                Self::HomeopathicPotencyOfDecimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_decimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_decimal_series_retired => {
                Self::HomeopathicPotencyOfDecimalSeriesRetired
            }
            SymbolRule::pri_homeopathic_potency_of_millesimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_millesimal_hahnemannian_series => {
                Self::HomeopathicPotencyOfMillesimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_millesimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_millesimal_korsakovian_series => {
                Self::HomeopathicPotencyOfMillesimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_millesimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_millesimal_series_retired => {
                Self::HomeopathicPotencyOfMillesimalSeriesRetired
            }
            SymbolRule::pri_homeopathic_potency_of_quintamillesimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_quintamillesimal_hahnemannian_series => {
                Self::HomeopathicPotencyOfQuintamillesimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_quintamillesimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_quintamillesimal_korsakovian_series => {
                Self::HomeopathicPotencyOfQuintamillesimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_quintamillesimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_quintamillesimal_series_retired => {
                Self::HomeopathicPotencyOfQuintamillesimalSeriesRetired
            }
            SymbolRule::pri_horsepower | SymbolRule::sec_horsepower => Self::Horsepower,
            SymbolRule::pri_hounsfield_unit | SymbolRule::sec_hounsfield_unit => {
                Self::HounsfieldUnit
            }
            SymbolRule::pri_hour | SymbolRule::sec_hour => Self::Hour,
            SymbolRule::pri_inch_british | SymbolRule::sec_inch_british => Self::InchBritish,
            SymbolRule::pri_inch_international | SymbolRule::sec_inch_international => {
                Self::InchInternational
            }
            SymbolRule::pri_inch_of_mercury_column | SymbolRule::sec_inch_of_mercury_column => {
                Self::InchOfMercuryColumn
            }
            SymbolRule::pri_inch_of_water_column | SymbolRule::sec_inch_of_water_column => {
                Self::InchOfWaterColumn
            }
            SymbolRule::pri_inch_us | SymbolRule::sec_inch_us => Self::InchUS,
            SymbolRule::pri_index_of_reactivity | SymbolRule::sec_index_of_reactivity => {
                Self::IndexOfReactivity
            }
            SymbolRule::pri_international_table_british_thermal_unit
            | SymbolRule::sec_international_table_british_thermal_unit => {
                Self::InternationalTableBritishThermalUnit
            }
            SymbolRule::pri_international_table_calorie
            | SymbolRule::sec_international_table_calorie => Self::InternationalTableCalorie,
            SymbolRule::pri_international_unit | SymbolRule::sec_international_unit => {
                Self::InternationalUnit
            }
            SymbolRule::pri_international_unit_secondary
            | SymbolRule::sec_international_unit_secondary => Self::InternationalUnitSecondary,
            SymbolRule::pri_joule | SymbolRule::sec_joule => Self::Joule,
            SymbolRule::pri_katal | SymbolRule::sec_katal => Self::Katal,
            SymbolRule::pri_kayser | SymbolRule::sec_kayser => Self::Kayser,
            SymbolRule::pri_kelvin | SymbolRule::sec_kelvin => Self::Kelvin,
            SymbolRule::pri_king_armstrong_unit | SymbolRule::sec_king_armstrong_unit => {
                Self::KingArmstrongUnit
            }
            SymbolRule::pri_knot_british | SymbolRule::sec_knot_british => Self::KnotBritish,
            SymbolRule::pri_knot_international | SymbolRule::sec_knot_international => {
                Self::KnotInternational
            }
            SymbolRule::pri_kunkel_unit | SymbolRule::sec_kunkel_unit => Self::KunkelUnit,
            SymbolRule::pri_lambert | SymbolRule::sec_lambert => Self::Lambert,
            SymbolRule::pri_light_year | SymbolRule::sec_light_year => Self::LightYear,
            SymbolRule::pri_ligne | SymbolRule::sec_ligne => Self::Ligne,
            SymbolRule::pri_limit_of_flocculation | SymbolRule::sec_limit_of_flocculation => {
                Self::LimitOfFlocculation
            }
            SymbolRule::pri_line | SymbolRule::sec_line => Self::Line,
            SymbolRule::pri_link_for_gunters_chain_british
            | SymbolRule::sec_link_for_gunters_chain_british => Self::LinkForGuntersChainBritish,
            SymbolRule::pri_link_for_gunters_chain_us
            | SymbolRule::sec_link_for_gunters_chain_us => Self::LinkForGuntersChainUS,
            SymbolRule::pri_link_for_ramdens_chain_us
            | SymbolRule::sec_link_for_ramdens_chain_us => Self::LinkForRamdensChainUS,
            SymbolRule::pri_liter | SymbolRule::sec_liter => Self::Liter,
            SymbolRule::pri_liter_secondary => Self::LiterSecondary,
            SymbolRule::pri_long_hunderdweight_avoirdupois
            | SymbolRule::sec_long_hunderdweight_avoirdupois => Self::LongHunderdweightAvoirdupois,
            SymbolRule::pri_long_ton_avoirdupois | SymbolRule::sec_long_ton_avoirdupois => {
                Self::LongTonAvoirdupois
            }
            SymbolRule::pri_low_power_field | SymbolRule::sec_low_power_field => {
                Self::LowPowerField
            }
            SymbolRule::pri_lumen | SymbolRule::sec_lumen => Self::Lumen,
            SymbolRule::pri_lux | SymbolRule::sec_lux => Self::Lux,
            SymbolRule::pri_mac_lagan_unit | SymbolRule::sec_mac_lagan_unit => Self::MacLaganUnit,
            SymbolRule::pri_maxwell | SymbolRule::sec_maxwell => Self::Maxwell,
            SymbolRule::pri_mean_british_thermal_unit
            | SymbolRule::sec_mean_british_thermal_unit => Self::MeanBritishThermalUnit,
            SymbolRule::pri_mean_calorie | SymbolRule::sec_mean_calorie => Self::MeanCalorie,
            SymbolRule::pri_mean_gregorian_month | SymbolRule::sec_mean_gregorian_month => {
                Self::MeanGregorianMonth
            }
            SymbolRule::pri_mean_gregorian_year | SymbolRule::sec_mean_gregorian_year => {
                Self::MeanGregorianYear
            }
            SymbolRule::pri_mean_julian_month | SymbolRule::sec_mean_julian_month => {
                Self::MeanJulianMonth
            }
            SymbolRule::pri_mean_julian_year | SymbolRule::sec_mean_julian_year => {
                Self::MeanJulianYear
            }
            SymbolRule::pri_mesh_international | SymbolRule::sec_mesh_international => {
                Self::MeshInternational
            }
            SymbolRule::pri_metabolic_equivalent | SymbolRule::sec_metabolic_equivalent => {
                Self::MetabolicEquivalent
            }
            SymbolRule::pri_meter | SymbolRule::sec_meter => Self::Meter,
            SymbolRule::pri_meter_of_mercury_column | SymbolRule::sec_meter_of_mercury_column => {
                Self::MeterOfMercuryColumn
            }
            SymbolRule::pri_meter_of_water_column | SymbolRule::sec_meter_of_water_column => {
                Self::MeterOfWaterColumn
            }
            SymbolRule::pri_meter_per_square_seconds_per_square_root_of_hertz
            | SymbolRule::sec_meter_per_square_seconds_per_square_root_of_hertz => {
                Self::MeterPerSquareSecondsPerSquareRootOfHertz
            }
            SymbolRule::pri_metric_carat | SymbolRule::sec_metric_carat => Self::MetricCarat,
            SymbolRule::pri_metric_cup | SymbolRule::sec_metric_cup => Self::MetricCup,
            SymbolRule::pri_metric_fluid_ounce | SymbolRule::sec_metric_fluid_ounce => {
                Self::MetricFluidOunce
            }
            SymbolRule::pri_metric_ounce | SymbolRule::sec_metric_ounce => Self::MetricOunce,
            SymbolRule::pri_metric_tablespoon | SymbolRule::sec_metric_tablespoon => {
                Self::MetricTablespoon
            }
            SymbolRule::pri_metric_teaspoon | SymbolRule::sec_metric_teaspoon => {
                Self::MetricTeaspoon
            }
            SymbolRule::pri_mho | SymbolRule::sec_mho => Self::Mho,
            SymbolRule::pri_mil_international | SymbolRule::sec_mil_international => {
                Self::MilInternational
            }
            SymbolRule::pri_mil_us | SymbolRule::sec_mil_us => Self::MilUS,
            SymbolRule::pri_mile_british | SymbolRule::sec_mile_british => Self::MileBritish,
            SymbolRule::pri_mile_international | SymbolRule::sec_mile_international => {
                Self::MileInternational
            }
            SymbolRule::pri_mile_us | SymbolRule::sec_mile_us => Self::MileUS,
            SymbolRule::pri_minim_british | SymbolRule::sec_minim_british => Self::MinimBritish,
            SymbolRule::pri_minim_us | SymbolRule::sec_minim_us => Self::MinimUS,
            SymbolRule::pri_minute | SymbolRule::sec_minute => Self::Minute,
            SymbolRule::pri_minute_angle | SymbolRule::sec_minute_angle => Self::MinuteAngle,
            SymbolRule::pri_mole | SymbolRule::sec_mole => Self::Mole,
            SymbolRule::pri_month | SymbolRule::sec_month => Self::Month,
            SymbolRule::pri_mpl_unit | SymbolRule::sec_mpl_unit => Self::MplUnit,
            SymbolRule::pri_nautical_mile_british | SymbolRule::sec_nautical_mile_british => {
                Self::NauticalMileBritish
            }
            SymbolRule::pri_nautical_mile_international
            | SymbolRule::sec_nautical_mile_international => Self::NauticalMileInternational,
            SymbolRule::pri_neper | SymbolRule::sec_neper => Self::Neper,
            SymbolRule::pri_newton | SymbolRule::sec_newton => Self::Newton,
            SymbolRule::pri_newtonian_constant_of_gravitation
            | SymbolRule::sec_newtonian_constant_of_gravitation => {
                Self::NewtonianConstantOfGravitation
            }
            SymbolRule::pri_nutrition_label_calories | SymbolRule::sec_nutrition_label_calories => {
                Self::NutritionLabelCalories
            }
            SymbolRule::pri_oersted | SymbolRule::sec_oersted => Self::Oersted,
            SymbolRule::pri_ohm | SymbolRule::sec_ohm => Self::Ohm,
            SymbolRule::pri_osmole | SymbolRule::sec_osmole => Self::Osmole,
            SymbolRule::pri_ounce_apothecaries | SymbolRule::sec_ounce_apothecaries => {
                Self::OunceApothecaries
            }
            SymbolRule::pri_ounce_avoirdupois | SymbolRule::sec_ounce_avoirdupois => {
                Self::OunceAvoirdupois
            }
            SymbolRule::pri_ounce_troy | SymbolRule::sec_ounce_troy => Self::OunceTroy,
            SymbolRule::pri_ph | SymbolRule::sec_ph => Self::PH,
            SymbolRule::pri_pace_british | SymbolRule::sec_pace_british => Self::PaceBritish,
            SymbolRule::pri_parsec | SymbolRule::sec_parsec => Self::Parsec,
            SymbolRule::pri_parts_per_billion | SymbolRule::sec_parts_per_billion => {
                Self::PartsPerBillion
            }
            SymbolRule::pri_parts_per_million | SymbolRule::sec_parts_per_million => {
                Self::PartsPerMillion
            }
            SymbolRule::pri_parts_per_thousand | SymbolRule::sec_parts_per_thousand => {
                Self::PartsPerThousand
            }
            SymbolRule::pri_parts_per_trillion | SymbolRule::sec_parts_per_trillion => {
                Self::PartsPerTrillion
            }
            SymbolRule::pri_pascal | SymbolRule::sec_pascal => Self::Pascal,
            SymbolRule::pri_peck_british | SymbolRule::sec_peck_british => Self::PeckBritish,
            SymbolRule::pri_peck_us | SymbolRule::sec_peck_us => Self::PeckUS,
            SymbolRule::pri_pennyweight_troy | SymbolRule::sec_pennyweight_troy => {
                Self::PennyweightTroy
            }
            SymbolRule::pri_percent | SymbolRule::sec_percent => Self::Percent,
            SymbolRule::pri_percent_of_slope | SymbolRule::sec_percent_of_slope => {
                Self::PercentOfSlope
            }
            SymbolRule::pri_peripheral_vascular_resistance_unit
            | SymbolRule::sec_peripheral_vascular_resistance_unit => {
                Self::PeripheralVascularResistanceUnit
            }
            SymbolRule::pri_permeability_of_vacuum | SymbolRule::sec_permeability_of_vacuum => {
                Self::PermeabilityOfVacuum
            }
            SymbolRule::pri_permittivity_of_vacuum | SymbolRule::sec_permittivity_of_vacuum => {
                Self::PermittivityOfVacuum
            }
            SymbolRule::pri_phot | SymbolRule::sec_phot => Self::Phot,
            SymbolRule::pri_pica | SymbolRule::sec_pica => Self::Pica,
            SymbolRule::pri_pied | SymbolRule::sec_pied => Self::Pied,
            SymbolRule::pri_pint_british | SymbolRule::sec_pint_british => Self::PintBritish,
            SymbolRule::pri_pint_us | SymbolRule::sec_pint_us => Self::PintUS,
            SymbolRule::pri_planck_constant | SymbolRule::sec_planck_constant => {
                Self::PlanckConstant
            }
            SymbolRule::pri_plaque_forming_units | SymbolRule::sec_plaque_forming_units => {
                Self::PlaqueFormingUnits
            }
            SymbolRule::pri_point | SymbolRule::sec_point => Self::Point,
            SymbolRule::pri_poise | SymbolRule::sec_poise => Self::Poise,
            SymbolRule::pri_pouce | SymbolRule::sec_pouce => Self::Pouce,
            SymbolRule::pri_pound_apothecaries | SymbolRule::sec_pound_apothecaries => {
                Self::PoundApothecaries
            }
            SymbolRule::pri_pound_avoirdupois | SymbolRule::sec_pound_avoirdupois => {
                Self::PoundAvoirdupois
            }
            SymbolRule::pri_pound_force_avoirdupois | SymbolRule::sec_pound_force_avoirdupois => {
                Self::PoundForceAvoirdupois
            }
            SymbolRule::pri_pound_per_sqare_inch | SymbolRule::sec_pound_per_sqare_inch => {
                Self::PoundPerSqareInch
            }
            SymbolRule::pri_pound_troy | SymbolRule::sec_pound_troy => Self::PoundTroy,
            SymbolRule::pri_printers_pica | SymbolRule::sec_printers_pica => Self::PrintersPica,
            SymbolRule::pri_printers_point | SymbolRule::sec_printers_point => Self::PrintersPoint,
            SymbolRule::pri_prism_diopter | SymbolRule::sec_prism_diopter => Self::PrismDiopter,
            SymbolRule::pri_protein_nitrogen_unit | SymbolRule::sec_protein_nitrogen_unit => {
                Self::ProteinNitrogenUnit
            }
            SymbolRule::pri_proton_mass | SymbolRule::sec_proton_mass => Self::ProtonMass,
            SymbolRule::pri_quart_british | SymbolRule::sec_quart_british => Self::QuartBritish,
            SymbolRule::pri_quart_us | SymbolRule::sec_quart_us => Self::QuartUS,
            SymbolRule::pri_queen_annes_wine_gallon_us
            | SymbolRule::sec_queen_annes_wine_gallon_us => Self::QueenAnnesWineGallonUS,
            SymbolRule::pri_radian | SymbolRule::sec_radian => Self::Radian,
            SymbolRule::pri_radiation_absorbed_dose | SymbolRule::sec_radiation_absorbed_dose => {
                Self::RadiationAbsorbedDose
            }
            SymbolRule::pri_radiation_equivalent_man | SymbolRule::sec_radiation_equivalent_man => {
                Self::RadiationEquivalentMan
            }
            SymbolRule::pri_ramdens_chain_us | SymbolRule::sec_ramdens_chain_us => {
                Self::RamdensChainUS
            }
            SymbolRule::pri_rod_british | SymbolRule::sec_rod_british => Self::RodBritish,
            SymbolRule::pri_rod_us | SymbolRule::sec_rod_us => Self::RodUS,
            SymbolRule::pri_roentgen | SymbolRule::sec_roentgen => Self::Roentgen,
            SymbolRule::pri_scruple_apothecaries | SymbolRule::sec_scruple_apothecaries => {
                Self::ScrupleApothecaries
            }
            SymbolRule::pri_second | SymbolRule::sec_second => Self::Second,
            SymbolRule::pri_second_angle | SymbolRule::sec_second_angle => Self::SecondAngle,
            SymbolRule::pri_section | SymbolRule::sec_section => Self::Section,
            SymbolRule::pri_short_hundredweight_avoirdupois
            | SymbolRule::sec_short_hundredweight_avoirdupois => {
                Self::ShortHundredweightAvoirdupois
            }
            SymbolRule::pri_short_ton_avoirdupois | SymbolRule::sec_short_ton_avoirdupois => {
                Self::ShortTonAvoirdupois
            }
            SymbolRule::pri_siemens | SymbolRule::sec_siemens => Self::Siemens,
            SymbolRule::pri_sievert | SymbolRule::sec_sievert => Self::Sievert,
            SymbolRule::pri_smoot | SymbolRule::sec_smoot => Self::Smoot,
            SymbolRule::pri_somogyi_unit | SymbolRule::sec_somogyi_unit => Self::SomogyiUnit,
            SymbolRule::pri_spere | SymbolRule::sec_spere => Self::Spere,
            SymbolRule::pri_square_foot_international
            | SymbolRule::sec_square_foot_international => Self::SquareFootInternational,
            SymbolRule::pri_square_inch_international
            | SymbolRule::sec_square_inch_international => Self::SquareInchInternational,
            SymbolRule::pri_square_mile_us | SymbolRule::sec_square_mile_us => Self::SquareMileUS,
            SymbolRule::pri_square_rod_us | SymbolRule::sec_square_rod_us => Self::SquareRodUS,
            SymbolRule::pri_square_yard_international
            | SymbolRule::sec_square_yard_international => Self::SquareYardInternational,
            SymbolRule::pri_standard_acceleration_of_free_fall
            | SymbolRule::sec_standard_acceleration_of_free_fall => {
                Self::StandardAccelerationOfFreeFall
            }
            SymbolRule::pri_standard_atmosphere | SymbolRule::sec_standard_atmosphere => {
                Self::StandardAtmosphere
            }
            SymbolRule::pri_steradian | SymbolRule::sec_steradian => Self::Steradian,
            SymbolRule::pri_stere | SymbolRule::sec_stere => Self::Stere,
            SymbolRule::pri_stilb | SymbolRule::sec_stilb => Self::Stilb,
            SymbolRule::pri_stokes | SymbolRule::sec_stokes => Self::Stokes,
            SymbolRule::pri_stone_avoirdupois | SymbolRule::sec_stone_avoirdupois => {
                Self::StoneAvoirdupois
            }
            SymbolRule::pri_svedberg_unit | SymbolRule::sec_svedberg_unit => Self::SvedbergUnit,
            SymbolRule::pri_synodal_month | SymbolRule::sec_synodal_month => Self::SynodalMonth,
            SymbolRule::pri_tablespoon_us | SymbolRule::sec_tablespoon_us => Self::TablespoonUS,
            SymbolRule::pri_teaspoon_us | SymbolRule::sec_teaspoon_us => Self::TeaspoonUS,
            SymbolRule::pri_technical_atmosphere | SymbolRule::sec_technical_atmosphere => {
                Self::TechnicalAtmosphere
            }
            SymbolRule::pri_tesla | SymbolRule::sec_tesla => Self::Tesla,
            SymbolRule::pri_tex | SymbolRule::sec_tex => Self::Tex,
            SymbolRule::pri_the_number_pi | SymbolRule::sec_the_number_pi => Self::TheNumberPi,
            SymbolRule::pri_the_number_ten_for_arbitrary_powers_caret
            | SymbolRule::sec_the_number_ten_for_arbitrary_powers_caret => {
                Self::TheNumberTenForArbitraryPowersCaret
            }
            SymbolRule::pri_the_number_ten_for_arbitrary_powers_star
            | SymbolRule::sec_the_number_ten_for_arbitrary_powers_star => {
                Self::TheNumberTenForArbitraryPowersStar
            }
            SymbolRule::pri_thermochemical_british_thermal_unit
            | SymbolRule::sec_thermochemical_british_thermal_unit => {
                Self::ThermochemicalBritishThermalUnit
            }
            SymbolRule::pri_thermochemical_calorie | SymbolRule::sec_thermochemical_calorie => {
                Self::ThermochemicalCalorie
            }
            SymbolRule::pri_tissue_culture_infectious_dose
            | SymbolRule::sec_tissue_culture_infectious_dose => Self::TissueCultureInfectiousDose,
            SymbolRule::pri_todd_unit | SymbolRule::sec_todd_unit => Self::ToddUnit,
            SymbolRule::pri_tonne | SymbolRule::sec_tonne => Self::Tonne,
            SymbolRule::pri_township | SymbolRule::sec_township => Self::Township,
            SymbolRule::pri_tropical_year | SymbolRule::sec_tropical_year => Self::TropicalYear,
            SymbolRule::pri_tuberculin_unit | SymbolRule::sec_tuberculin_unit => {
                Self::TuberculinUnit
            }
            SymbolRule::pri_unified_atomic_mass_unit | SymbolRule::sec_unified_atomic_mass_unit => {
                Self::UnifiedAtomicMassUnit
            }
            SymbolRule::pri_unit | SymbolRule::sec_unit => Self::Unit,
            SymbolRule::pri_united_states_pharmacopeia_unit
            | SymbolRule::sec_united_states_pharmacopeia_unit => Self::UnitedStatesPharmacopeiaUnit,
            SymbolRule::pri_velocity_of_light | SymbolRule::sec_velocity_of_light => {
                Self::VelocityOfLight
            }
            SymbolRule::pri_volt | SymbolRule::sec_volt => Self::Volt,
            SymbolRule::pri_watt | SymbolRule::sec_watt => Self::Watt,
            SymbolRule::pri_weber | SymbolRule::sec_weber => Self::Weber,
            SymbolRule::pri_week | SymbolRule::sec_week => Self::Week,
            SymbolRule::pri_wood_unit | SymbolRule::sec_wood_unit => Self::WoodUnit,
            SymbolRule::pri_yard_british | SymbolRule::sec_yard_british => Self::YardBritish,
            SymbolRule::pri_yard_international | SymbolRule::sec_yard_international => {
                Self::YardInternational
            }
            SymbolRule::pri_yard_us | SymbolRule::sec_yard_us => Self::YardUS,
            SymbolRule::pri_year | SymbolRule::sec_year => Self::Year,
            _ => {
                return Err(Error::BadFragment {
                    fragment: pair.as_span().as_str().to_string(),
                    position: pair.as_span().start(),
                });
            }
        };

        Ok(atom)
    }
}
