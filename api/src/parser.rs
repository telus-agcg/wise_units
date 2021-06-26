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
pub mod atom;
pub mod classification;
#[allow(clippy::non_ascii_literal)]
pub mod property;

pub(self) mod symbols;

mod annotation_composition;
#[cfg(test)]
mod atom_test;
mod composable;
mod composition;
mod definition;
mod dimension;
mod error;
mod function_set;
mod prefix;
mod term;
mod terms;
mod ucum_symbol;

pub use self::{
    annotation_composition::AnnotationComposition, atom::Atom, classification::Classification,
    composable::Composable, composition::Composition, dimension::Dimension, error::Error,
    prefix::Prefix, property::Property, term::Term, ucum_symbol::UcumSymbol,
};

use self::{
    symbols::symbol_parser::Rule as SymbolRule,
    terms::term_parser::{Rule as TermRule, TermParser},
};
use pest::{iterators::Pair, Parser};

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
impl Visit<SymbolRule> for Atom {
    fn visit(pair: Pair<'_, SymbolRule>) -> Result<Atom, Error> {
        let atom = match pair.as_rule() {
            SymbolRule::pri_acre_british | SymbolRule::sec_acre_british => Atom::AcreBritish,
            SymbolRule::pri_acre_us | SymbolRule::sec_acre_us => Atom::AcreUS,
            SymbolRule::pri_allergen_unit | SymbolRule::sec_allergen_unit => Atom::AllergenUnit,
            SymbolRule::pri_allergen_unit_for_ambrosia_artemisiifolia
            | SymbolRule::sec_allergen_unit_for_ambrosia_artemisiifolia => {
                Atom::AllergenUnitForAmbrosiaArtemisiifolia
            }
            SymbolRule::pri_ampere | SymbolRule::sec_ampere => Atom::Ampere,
            SymbolRule::pri_angstrom | SymbolRule::sec_angstrom => Atom::Angstrom,
            SymbolRule::pri_anti_factor_xa_unit | SymbolRule::sec_anti_factor_xa_unit => {
                Atom::AntiFactorXaUnit
            }
            SymbolRule::pri_apl_unit | SymbolRule::sec_apl_unit => Atom::AplUnit,
            SymbolRule::pri_arbitrary_unit | SymbolRule::sec_arbitrary_unit => Atom::ArbitraryUnit,
            SymbolRule::pri_are | SymbolRule::sec_are => Atom::Are,
            SymbolRule::pri_astronomic_unit | SymbolRule::sec_astronomic_unit => {
                Atom::AstronomicUnit
            }
            SymbolRule::pri_bar | SymbolRule::sec_bar => Atom::Bar,
            SymbolRule::pri_barn | SymbolRule::sec_barn => Atom::Barn,
            SymbolRule::pri_barrel_us | SymbolRule::sec_barrel_us => Atom::BarrelUS,
            SymbolRule::pri_baud | SymbolRule::sec_baud => Atom::Baud,
            SymbolRule::pri_becquerel | SymbolRule::sec_becquerel => Atom::Becquerel,
            SymbolRule::pri_bel | SymbolRule::sec_bel => Atom::Bel,
            SymbolRule::pri_bel10_nanovolt | SymbolRule::sec_bel10_nanovolt => Atom::Bel10Nanovolt,
            SymbolRule::pri_bel_kilowatt | SymbolRule::sec_bel_kilowatt => Atom::BelKilowatt,
            SymbolRule::pri_bel_microvolt | SymbolRule::sec_bel_microvolt => Atom::BelMicrovolt,
            SymbolRule::pri_bel_millivolt | SymbolRule::sec_bel_millivolt => Atom::BelMillivolt,
            SymbolRule::pri_bel_sound_pressure | SymbolRule::sec_bel_sound_pressure => {
                Atom::BelSoundPressure
            }
            SymbolRule::pri_bel_volt | SymbolRule::sec_bel_volt => Atom::BelVolt,
            SymbolRule::pri_bel_watt | SymbolRule::sec_bel_watt => Atom::BelWatt,
            SymbolRule::pri_bethesda_unit | SymbolRule::sec_bethesda_unit => Atom::BethesdaUnit,
            SymbolRule::pri_bioequivalent_allergen_unit
            | SymbolRule::sec_bioequivalent_allergen_unit => Atom::BioequivalentAllergenUnit,
            SymbolRule::pri_biot | SymbolRule::sec_biot => Atom::Biot,
            SymbolRule::pri_bit | SymbolRule::sec_bit => Atom::Bit,
            SymbolRule::pri_bit_logarithmus_dualis | SymbolRule::sec_bit_logarithmus_dualis => {
                Atom::BitLogarithmusDualis
            }
            SymbolRule::pri_board_foot_international | SymbolRule::sec_board_foot_international => {
                Atom::BoardFootInternational
            }
            SymbolRule::pri_bodansky_unit | SymbolRule::sec_bodansky_unit => Atom::BodanskyUnit,
            SymbolRule::pri_boltzmann_constant | SymbolRule::sec_boltzmann_constant => {
                Atom::BoltzmannConstant
            }
            SymbolRule::pri_british_thermal_unit | SymbolRule::sec_british_thermal_unit => {
                Atom::BritishThermalUnit
            }
            SymbolRule::pri_british_thermal_unit_at39_f
            | SymbolRule::sec_british_thermal_unit_at39_f => Atom::BritishThermalUnitAt39F,
            SymbolRule::pri_british_thermal_unit_at59_f
            | SymbolRule::sec_british_thermal_unit_at59_f => Atom::BritishThermalUnitAt59F,
            SymbolRule::pri_british_thermal_unit_at60_f
            | SymbolRule::sec_british_thermal_unit_at60_f => Atom::BritishThermalUnitAt60F,
            SymbolRule::pri_bushel_british | SymbolRule::sec_bushel_british => Atom::BushelBritish,
            SymbolRule::pri_bushel_us | SymbolRule::sec_bushel_us => Atom::BushelUS,
            SymbolRule::pri_byte | SymbolRule::sec_byte => Atom::Byte,
            SymbolRule::pri_calorie | SymbolRule::sec_calorie => Atom::Calorie,
            SymbolRule::pri_calorie_at15_c | SymbolRule::sec_calorie_at15_c => Atom::CalorieAt15C,
            SymbolRule::pri_calorie_at20_c | SymbolRule::sec_calorie_at20_c => Atom::CalorieAt20C,
            SymbolRule::pri_candela | SymbolRule::sec_candela => Atom::Candela,
            SymbolRule::pri_carat_of_gold_alloys | SymbolRule::sec_carat_of_gold_alloys => {
                Atom::CaratOfGoldAlloys
            }
            SymbolRule::pri_cell_culture_infectious_dose
            | SymbolRule::sec_cell_culture_infectious_dose => Atom::CellCultureInfectiousDose,
            SymbolRule::pri_charriere | SymbolRule::sec_charriere => Atom::Charriere,
            SymbolRule::pri_cicero | SymbolRule::sec_cicero => Atom::Cicero,
            SymbolRule::pri_circle | SymbolRule::sec_circle => Atom::Circle,
            SymbolRule::pri_circular_mil_international
            | SymbolRule::sec_circular_mil_international => Atom::CircularMilInternational,
            SymbolRule::pri_colony_forming_units | SymbolRule::sec_colony_forming_units => {
                Atom::ColonyFormingUnits
            }
            SymbolRule::pri_cord_international | SymbolRule::sec_cord_international => {
                Atom::CordInternational
            }
            SymbolRule::pri_cord_us | SymbolRule::sec_cord_us => Atom::CordUS,
            SymbolRule::pri_coulomb | SymbolRule::sec_coulomb => Atom::Coulomb,
            SymbolRule::pri_cubic_foot_international | SymbolRule::sec_cubic_foot_international => {
                Atom::CubicFootInternational
            }
            SymbolRule::pri_cubic_inch_international | SymbolRule::sec_cubic_inch_international => {
                Atom::CubicInchInternational
            }
            SymbolRule::pri_cubic_yard_international | SymbolRule::sec_cubic_yard_international => {
                Atom::CubicYardInternational
            }
            SymbolRule::pri_cup_us | SymbolRule::sec_cup_us => Atom::CupUS,
            SymbolRule::pri_curie | SymbolRule::sec_curie => Atom::Curie,
            SymbolRule::pri_d_antigen_unit | SymbolRule::sec_d_antigen_unit => Atom::DAntigenUnit,
            SymbolRule::pri_day | SymbolRule::sec_day => Atom::Day,
            SymbolRule::pri_degree | SymbolRule::sec_degree => Atom::Degree,
            SymbolRule::pri_degree_celsius | SymbolRule::sec_degree_celsius => Atom::DegreeCelsius,
            SymbolRule::pri_degree_fahrenheit | SymbolRule::sec_degree_fahrenheit => {
                Atom::DegreeFahrenheit
            }
            SymbolRule::pri_degree_rankine | SymbolRule::sec_degree_rankine => Atom::DegreeRankine,
            SymbolRule::pri_degree_reaumur | SymbolRule::sec_degree_reaumur => Atom::DegreeReaumur,
            SymbolRule::pri_denier | SymbolRule::sec_denier => Atom::Denier,
            SymbolRule::pri_didot | SymbolRule::sec_didot => Atom::Didot,
            SymbolRule::pri_diopter | SymbolRule::sec_diopter => Atom::Diopter,
            SymbolRule::pri_dram_apothecaries | SymbolRule::sec_dram_apothecaries => {
                Atom::DramApothecaries
            }
            SymbolRule::pri_dram_avoirdupois | SymbolRule::sec_dram_avoirdupois => {
                Atom::DramAvoirdupois
            }
            SymbolRule::pri_drop | SymbolRule::sec_drop => Atom::Drop,
            SymbolRule::pri_dry_pint_us | SymbolRule::sec_dry_pint_us => Atom::DryPintUS,
            SymbolRule::pri_dry_quart_us | SymbolRule::sec_dry_quart_us => Atom::DryQuartUS,
            SymbolRule::pri_dye_unit | SymbolRule::sec_dye_unit => Atom::DyeUnit,
            SymbolRule::pri_dyne | SymbolRule::sec_dyne => Atom::Dyne,
            SymbolRule::pri_ehrlich_unit | SymbolRule::sec_ehrlich_unit => Atom::EhrlichUnit,
            SymbolRule::pri_electron_mass | SymbolRule::sec_electron_mass => Atom::ElectronMass,
            SymbolRule::pri_electronvolt | SymbolRule::sec_electronvolt => Atom::Electronvolt,
            SymbolRule::pri_elementary_charge | SymbolRule::sec_elementary_charge => {
                Atom::ElementaryCharge
            }
            SymbolRule::pri_elisa_unit | SymbolRule::sec_elisa_unit => Atom::ElisaUnit,
            SymbolRule::pri_embryo_infectious_dose | SymbolRule::sec_embryo_infectious_dose => {
                Atom::EmbryoInfectiousDose
            }
            SymbolRule::pri_equivalents | SymbolRule::sec_equivalents => Atom::Equivalents,
            SymbolRule::pri_erg | SymbolRule::sec_erg => Atom::Erg,
            SymbolRule::pri_farad | SymbolRule::sec_farad => Atom::Farad,
            SymbolRule::pri_fathom_british | SymbolRule::sec_fathom_british => Atom::FathomBritish,
            SymbolRule::pri_fathom_international | SymbolRule::sec_fathom_international => {
                Atom::FathomInternational
            }
            SymbolRule::pri_fathom_us | SymbolRule::sec_fathom_us => Atom::FathomUS,
            SymbolRule::pri_fibrinogen_equivalent_unit
            | SymbolRule::sec_fibrinogen_equivalent_unit => Atom::FibrinogenEquivalentUnit,
            SymbolRule::pri_fluid_dram_british | SymbolRule::sec_fluid_dram_british => {
                Atom::FluidDramBritish
            }
            SymbolRule::pri_fluid_dram_us | SymbolRule::sec_fluid_dram_us => Atom::FluidDramUS,
            SymbolRule::pri_fluid_ounce_british | SymbolRule::sec_fluid_ounce_british => {
                Atom::FluidOunceBritish
            }
            SymbolRule::pri_fluid_ounce_us | SymbolRule::sec_fluid_ounce_us => Atom::FluidOunceUS,
            SymbolRule::pri_focus_forming_units | SymbolRule::sec_focus_forming_units => {
                Atom::FocusFormingUnits
            }
            SymbolRule::pri_foot_british | SymbolRule::sec_foot_british => Atom::FootBritish,
            SymbolRule::pri_foot_international | SymbolRule::sec_foot_international => {
                Atom::FootInternational
            }
            SymbolRule::pri_foot_us | SymbolRule::sec_foot_us => Atom::FootUS,
            SymbolRule::pri_furlong_us | SymbolRule::sec_furlong_us => Atom::FurlongUS,
            SymbolRule::pri_gal | SymbolRule::sec_gal => Atom::Gal,
            SymbolRule::pri_gallon_british | SymbolRule::sec_gallon_british => Atom::GallonBritish,
            SymbolRule::pri_gauss | SymbolRule::sec_gauss => Atom::Gauss,
            SymbolRule::pri_gilbert | SymbolRule::sec_gilbert => Atom::Gilbert,
            SymbolRule::pri_gill_british | SymbolRule::sec_gill_british => Atom::GillBritish,
            SymbolRule::pri_gill_us | SymbolRule::sec_gill_us => Atom::GillUS,
            SymbolRule::pri_gon | SymbolRule::sec_gon => Atom::Gon,
            SymbolRule::pri_gpl_unit | SymbolRule::sec_gpl_unit => Atom::GplUnit,
            SymbolRule::pri_grain | SymbolRule::sec_grain => Atom::Grain,
            SymbolRule::pri_gram | SymbolRule::sec_gram => Atom::Gram,
            SymbolRule::pri_gram_force | SymbolRule::sec_gram_force => Atom::GramForce,
            SymbolRule::pri_gram_percent | SymbolRule::sec_gram_percent => Atom::GramPercent,
            SymbolRule::pri_gray | SymbolRule::sec_gray => Atom::Gray,
            SymbolRule::pri_gunters_chain_british | SymbolRule::sec_gunters_chain_british => {
                Atom::GuntersChainBritish
            }
            SymbolRule::pri_gunters_chain_us | SymbolRule::sec_gunters_chain_us => {
                Atom::GuntersChainUS
            }
            SymbolRule::pri_hand_international | SymbolRule::sec_hand_international => {
                Atom::HandInternational
            }
            SymbolRule::pri_henry | SymbolRule::sec_henry => Atom::Henry,
            SymbolRule::pri_hertz | SymbolRule::sec_hertz => Atom::Hertz,
            SymbolRule::pri_high_power_field | SymbolRule::sec_high_power_field => {
                Atom::HighPowerField
            }
            SymbolRule::pri_historical_winchester_gallon
            | SymbolRule::sec_historical_winchester_gallon => Atom::HistoricalWinchesterGallon,
            SymbolRule::pri_homeopathic_potency_of_centesimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_centesimal_hahnemannian_series => {
                Atom::HomeopathicPotencyOfCentesimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_centesimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_centesimal_korsakovian_series => {
                Atom::HomeopathicPotencyOfCentesimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_centesimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_centesimal_series_retired => {
                Atom::HomeopathicPotencyOfCentesimalSeriesRetired
            }
            SymbolRule::pri_homeopathic_potency_of_decimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_decimal_hahnemannian_series => {
                Atom::HomeopathicPotencyOfDecimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_decimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_decimal_korsakovian_series => {
                Atom::HomeopathicPotencyOfDecimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_decimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_decimal_series_retired => {
                Atom::HomeopathicPotencyOfDecimalSeriesRetired
            }
            SymbolRule::pri_homeopathic_potency_of_millesimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_millesimal_hahnemannian_series => {
                Atom::HomeopathicPotencyOfMillesimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_millesimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_millesimal_korsakovian_series => {
                Atom::HomeopathicPotencyOfMillesimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_millesimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_millesimal_series_retired => {
                Atom::HomeopathicPotencyOfMillesimalSeriesRetired
            }
            SymbolRule::pri_homeopathic_potency_of_quintamillesimal_hahnemannian_series
            | SymbolRule::sec_homeopathic_potency_of_quintamillesimal_hahnemannian_series => {
                Atom::HomeopathicPotencyOfQuintamillesimalHahnemannianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_quintamillesimal_korsakovian_series
            | SymbolRule::sec_homeopathic_potency_of_quintamillesimal_korsakovian_series => {
                Atom::HomeopathicPotencyOfQuintamillesimalKorsakovianSeries
            }
            SymbolRule::pri_homeopathic_potency_of_quintamillesimal_series_retired
            | SymbolRule::sec_homeopathic_potency_of_quintamillesimal_series_retired => {
                Atom::HomeopathicPotencyOfQuintamillesimalSeriesRetired
            }
            SymbolRule::pri_horsepower | SymbolRule::sec_horsepower => Atom::Horsepower,
            SymbolRule::pri_hounsfield_unit | SymbolRule::sec_hounsfield_unit => {
                Atom::HounsfieldUnit
            }
            SymbolRule::pri_hour | SymbolRule::sec_hour => Atom::Hour,
            SymbolRule::pri_inch_british | SymbolRule::sec_inch_british => Atom::InchBritish,
            SymbolRule::pri_inch_international | SymbolRule::sec_inch_international => {
                Atom::InchInternational
            }
            SymbolRule::pri_inch_of_mercury_column | SymbolRule::sec_inch_of_mercury_column => {
                Atom::InchOfMercuryColumn
            }
            SymbolRule::pri_inch_of_water_column | SymbolRule::sec_inch_of_water_column => {
                Atom::InchOfWaterColumn
            }
            SymbolRule::pri_inch_us | SymbolRule::sec_inch_us => Atom::InchUS,
            SymbolRule::pri_index_of_reactivity | SymbolRule::sec_index_of_reactivity => {
                Atom::IndexOfReactivity
            }
            SymbolRule::pri_international_table_british_thermal_unit
            | SymbolRule::sec_international_table_british_thermal_unit => {
                Atom::InternationalTableBritishThermalUnit
            }
            SymbolRule::pri_international_table_calorie
            | SymbolRule::sec_international_table_calorie => Atom::InternationalTableCalorie,
            SymbolRule::pri_international_unit | SymbolRule::sec_international_unit => {
                Atom::InternationalUnit
            }
            SymbolRule::pri_international_unit_secondary
            | SymbolRule::sec_international_unit_secondary => Atom::InternationalUnitSecondary,
            SymbolRule::pri_joule | SymbolRule::sec_joule => Atom::Joule,
            SymbolRule::pri_katal | SymbolRule::sec_katal => Atom::Katal,
            SymbolRule::pri_kayser | SymbolRule::sec_kayser => Atom::Kayser,
            SymbolRule::pri_kelvin | SymbolRule::sec_kelvin => Atom::Kelvin,
            SymbolRule::pri_king_armstrong_unit | SymbolRule::sec_king_armstrong_unit => {
                Atom::KingArmstrongUnit
            }
            SymbolRule::pri_knot_british | SymbolRule::sec_knot_british => Atom::KnotBritish,
            SymbolRule::pri_knot_international | SymbolRule::sec_knot_international => {
                Atom::KnotInternational
            }
            SymbolRule::pri_kunkel_unit | SymbolRule::sec_kunkel_unit => Atom::KunkelUnit,
            SymbolRule::pri_lambert | SymbolRule::sec_lambert => Atom::Lambert,
            SymbolRule::pri_light_year | SymbolRule::sec_light_year => Atom::LightYear,
            SymbolRule::pri_ligne | SymbolRule::sec_ligne => Atom::Ligne,
            SymbolRule::pri_limit_of_flocculation | SymbolRule::sec_limit_of_flocculation => {
                Atom::LimitOfFlocculation
            }
            SymbolRule::pri_line | SymbolRule::sec_line => Atom::Line,
            SymbolRule::pri_link_for_gunters_chain_british
            | SymbolRule::sec_link_for_gunters_chain_british => Atom::LinkForGuntersChainBritish,
            SymbolRule::pri_link_for_gunters_chain_us
            | SymbolRule::sec_link_for_gunters_chain_us => Atom::LinkForGuntersChainUS,
            SymbolRule::pri_link_for_ramdens_chain_us
            | SymbolRule::sec_link_for_ramdens_chain_us => Atom::LinkForRamdensChainUS,
            SymbolRule::pri_liter | SymbolRule::sec_liter => Atom::Liter,
            SymbolRule::pri_liter_secondary => Atom::LiterSecondary,
            SymbolRule::pri_long_hunderdweight_avoirdupois
            | SymbolRule::sec_long_hunderdweight_avoirdupois => Atom::LongHunderdweightAvoirdupois,
            SymbolRule::pri_long_ton_avoirdupois | SymbolRule::sec_long_ton_avoirdupois => {
                Atom::LongTonAvoirdupois
            }
            SymbolRule::pri_low_power_field | SymbolRule::sec_low_power_field => {
                Atom::LowPowerField
            }
            SymbolRule::pri_lumen | SymbolRule::sec_lumen => Atom::Lumen,
            SymbolRule::pri_lux | SymbolRule::sec_lux => Atom::Lux,
            SymbolRule::pri_mac_lagan_unit | SymbolRule::sec_mac_lagan_unit => Atom::MacLaganUnit,
            SymbolRule::pri_maxwell | SymbolRule::sec_maxwell => Atom::Maxwell,
            SymbolRule::pri_mean_british_thermal_unit
            | SymbolRule::sec_mean_british_thermal_unit => Atom::MeanBritishThermalUnit,
            SymbolRule::pri_mean_calorie | SymbolRule::sec_mean_calorie => Atom::MeanCalorie,
            SymbolRule::pri_mean_gregorian_month | SymbolRule::sec_mean_gregorian_month => {
                Atom::MeanGregorianMonth
            }
            SymbolRule::pri_mean_gregorian_year | SymbolRule::sec_mean_gregorian_year => {
                Atom::MeanGregorianYear
            }
            SymbolRule::pri_mean_julian_month | SymbolRule::sec_mean_julian_month => {
                Atom::MeanJulianMonth
            }
            SymbolRule::pri_mean_julian_year | SymbolRule::sec_mean_julian_year => {
                Atom::MeanJulianYear
            }
            SymbolRule::pri_mesh_international | SymbolRule::sec_mesh_international => {
                Atom::MeshInternational
            }
            SymbolRule::pri_metabolic_equivalent | SymbolRule::sec_metabolic_equivalent => {
                Atom::MetabolicEquivalent
            }
            SymbolRule::pri_meter | SymbolRule::sec_meter => Atom::Meter,
            SymbolRule::pri_meter_of_mercury_column | SymbolRule::sec_meter_of_mercury_column => {
                Atom::MeterOfMercuryColumn
            }
            SymbolRule::pri_meter_of_water_column | SymbolRule::sec_meter_of_water_column => {
                Atom::MeterOfWaterColumn
            }
            SymbolRule::pri_meter_per_square_seconds_per_square_root_of_hertz
            | SymbolRule::sec_meter_per_square_seconds_per_square_root_of_hertz => {
                Atom::MeterPerSquareSecondsPerSquareRootOfHertz
            }
            SymbolRule::pri_metric_carat | SymbolRule::sec_metric_carat => Atom::MetricCarat,
            SymbolRule::pri_metric_cup | SymbolRule::sec_metric_cup => Atom::MetricCup,
            SymbolRule::pri_metric_fluid_ounce | SymbolRule::sec_metric_fluid_ounce => {
                Atom::MetricFluidOunce
            }
            SymbolRule::pri_metric_ounce | SymbolRule::sec_metric_ounce => Atom::MetricOunce,
            SymbolRule::pri_metric_tablespoon | SymbolRule::sec_metric_tablespoon => {
                Atom::MetricTablespoon
            }
            SymbolRule::pri_metric_teaspoon | SymbolRule::sec_metric_teaspoon => {
                Atom::MetricTeaspoon
            }
            SymbolRule::pri_mho | SymbolRule::sec_mho => Atom::Mho,
            SymbolRule::pri_mil_international | SymbolRule::sec_mil_international => {
                Atom::MilInternational
            }
            SymbolRule::pri_mil_us | SymbolRule::sec_mil_us => Atom::MilUS,
            SymbolRule::pri_mile_british | SymbolRule::sec_mile_british => Atom::MileBritish,
            SymbolRule::pri_mile_international | SymbolRule::sec_mile_international => {
                Atom::MileInternational
            }
            SymbolRule::pri_mile_us | SymbolRule::sec_mile_us => Atom::MileUS,
            SymbolRule::pri_minim_british | SymbolRule::sec_minim_british => Atom::MinimBritish,
            SymbolRule::pri_minim_us | SymbolRule::sec_minim_us => Atom::MinimUS,
            SymbolRule::pri_minute | SymbolRule::sec_minute => Atom::Minute,
            SymbolRule::pri_minute_angle | SymbolRule::sec_minute_angle => Atom::MinuteAngle,
            SymbolRule::pri_mole | SymbolRule::sec_mole => Atom::Mole,
            SymbolRule::pri_month | SymbolRule::sec_month => Atom::Month,
            SymbolRule::pri_mpl_unit | SymbolRule::sec_mpl_unit => Atom::MplUnit,
            SymbolRule::pri_nautical_mile_british | SymbolRule::sec_nautical_mile_british => {
                Atom::NauticalMileBritish
            }
            SymbolRule::pri_nautical_mile_international
            | SymbolRule::sec_nautical_mile_international => Atom::NauticalMileInternational,
            SymbolRule::pri_neper | SymbolRule::sec_neper => Atom::Neper,
            SymbolRule::pri_newton | SymbolRule::sec_newton => Atom::Newton,
            SymbolRule::pri_newtonian_constant_of_gravitation
            | SymbolRule::sec_newtonian_constant_of_gravitation => {
                Atom::NewtonianConstantOfGravitation
            }
            SymbolRule::pri_nutrition_label_calories | SymbolRule::sec_nutrition_label_calories => {
                Atom::NutritionLabelCalories
            }
            SymbolRule::pri_oersted | SymbolRule::sec_oersted => Atom::Oersted,
            SymbolRule::pri_ohm | SymbolRule::sec_ohm => Atom::Ohm,
            SymbolRule::pri_osmole | SymbolRule::sec_osmole => Atom::Osmole,
            SymbolRule::pri_ounce_apothecaries | SymbolRule::sec_ounce_apothecaries => {
                Atom::OunceApothecaries
            }
            SymbolRule::pri_ounce_avoirdupois | SymbolRule::sec_ounce_avoirdupois => {
                Atom::OunceAvoirdupois
            }
            SymbolRule::pri_ounce_troy | SymbolRule::sec_ounce_troy => Atom::OunceTroy,
            SymbolRule::pri_ph | SymbolRule::sec_ph => Atom::PH,
            SymbolRule::pri_pace_british | SymbolRule::sec_pace_british => Atom::PaceBritish,
            SymbolRule::pri_parsec | SymbolRule::sec_parsec => Atom::Parsec,
            SymbolRule::pri_parts_per_billion | SymbolRule::sec_parts_per_billion => {
                Atom::PartsPerBillion
            }
            SymbolRule::pri_parts_per_million | SymbolRule::sec_parts_per_million => {
                Atom::PartsPerMillion
            }
            SymbolRule::pri_parts_per_thousand | SymbolRule::sec_parts_per_thousand => {
                Atom::PartsPerThousand
            }
            SymbolRule::pri_parts_per_trillion | SymbolRule::sec_parts_per_trillion => {
                Atom::PartsPerTrillion
            }
            SymbolRule::pri_pascal | SymbolRule::sec_pascal => Atom::Pascal,
            SymbolRule::pri_peck_british | SymbolRule::sec_peck_british => Atom::PeckBritish,
            SymbolRule::pri_peck_us | SymbolRule::sec_peck_us => Atom::PeckUS,
            SymbolRule::pri_pennyweight_troy | SymbolRule::sec_pennyweight_troy => {
                Atom::PennyweightTroy
            }
            SymbolRule::pri_percent | SymbolRule::sec_percent => Atom::Percent,
            SymbolRule::pri_percent_of_slope | SymbolRule::sec_percent_of_slope => {
                Atom::PercentOfSlope
            }
            SymbolRule::pri_peripheral_vascular_resistance_unit
            | SymbolRule::sec_peripheral_vascular_resistance_unit => {
                Atom::PeripheralVascularResistanceUnit
            }
            SymbolRule::pri_permeability_of_vacuum | SymbolRule::sec_permeability_of_vacuum => {
                Atom::PermeabilityOfVacuum
            }
            SymbolRule::pri_permittivity_of_vacuum | SymbolRule::sec_permittivity_of_vacuum => {
                Atom::PermittivityOfVacuum
            }
            SymbolRule::pri_phot | SymbolRule::sec_phot => Atom::Phot,
            SymbolRule::pri_pica | SymbolRule::sec_pica => Atom::Pica,
            SymbolRule::pri_pied | SymbolRule::sec_pied => Atom::Pied,
            SymbolRule::pri_pint_british | SymbolRule::sec_pint_british => Atom::PintBritish,
            SymbolRule::pri_pint_us | SymbolRule::sec_pint_us => Atom::PintUS,
            SymbolRule::pri_planck_constant | SymbolRule::sec_planck_constant => {
                Atom::PlanckConstant
            }
            SymbolRule::pri_plaque_forming_units | SymbolRule::sec_plaque_forming_units => {
                Atom::PlaqueFormingUnits
            }
            SymbolRule::pri_point | SymbolRule::sec_point => Atom::Point,
            SymbolRule::pri_poise | SymbolRule::sec_poise => Atom::Poise,
            SymbolRule::pri_pouce | SymbolRule::sec_pouce => Atom::Pouce,
            SymbolRule::pri_pound_apothecaries | SymbolRule::sec_pound_apothecaries => {
                Atom::PoundApothecaries
            }
            SymbolRule::pri_pound_avoirdupois | SymbolRule::sec_pound_avoirdupois => {
                Atom::PoundAvoirdupois
            }
            SymbolRule::pri_pound_force_avoirdupois | SymbolRule::sec_pound_force_avoirdupois => {
                Atom::PoundForceAvoirdupois
            }
            SymbolRule::pri_pound_per_sqare_inch | SymbolRule::sec_pound_per_sqare_inch => {
                Atom::PoundPerSqareInch
            }
            SymbolRule::pri_pound_troy | SymbolRule::sec_pound_troy => Atom::PoundTroy,
            SymbolRule::pri_printers_pica | SymbolRule::sec_printers_pica => Atom::PrintersPica,
            SymbolRule::pri_printers_point | SymbolRule::sec_printers_point => Atom::PrintersPoint,
            SymbolRule::pri_prism_diopter | SymbolRule::sec_prism_diopter => Atom::PrismDiopter,
            SymbolRule::pri_protein_nitrogen_unit | SymbolRule::sec_protein_nitrogen_unit => {
                Atom::ProteinNitrogenUnit
            }
            SymbolRule::pri_proton_mass | SymbolRule::sec_proton_mass => Atom::ProtonMass,
            SymbolRule::pri_quart_british | SymbolRule::sec_quart_british => Atom::QuartBritish,
            SymbolRule::pri_quart_us | SymbolRule::sec_quart_us => Atom::QuartUS,
            SymbolRule::pri_queen_annes_wine_gallon_us
            | SymbolRule::sec_queen_annes_wine_gallon_us => Atom::QueenAnnesWineGallonUS,
            SymbolRule::pri_radian | SymbolRule::sec_radian => Atom::Radian,
            SymbolRule::pri_radiation_absorbed_dose | SymbolRule::sec_radiation_absorbed_dose => {
                Atom::RadiationAbsorbedDose
            }
            SymbolRule::pri_radiation_equivalent_man | SymbolRule::sec_radiation_equivalent_man => {
                Atom::RadiationEquivalentMan
            }
            SymbolRule::pri_ramdens_chain_us | SymbolRule::sec_ramdens_chain_us => {
                Atom::RamdensChainUS
            }
            SymbolRule::pri_rod_british | SymbolRule::sec_rod_british => Atom::RodBritish,
            SymbolRule::pri_rod_us | SymbolRule::sec_rod_us => Atom::RodUS,
            SymbolRule::pri_roentgen | SymbolRule::sec_roentgen => Atom::Roentgen,
            SymbolRule::pri_scruple_apothecaries | SymbolRule::sec_scruple_apothecaries => {
                Atom::ScrupleApothecaries
            }
            SymbolRule::pri_second | SymbolRule::sec_second => Atom::Second,
            SymbolRule::pri_second_angle | SymbolRule::sec_second_angle => Atom::SecondAngle,
            SymbolRule::pri_section | SymbolRule::sec_section => Atom::Section,
            SymbolRule::pri_short_hundredweight_avoirdupois
            | SymbolRule::sec_short_hundredweight_avoirdupois => {
                Atom::ShortHundredweightAvoirdupois
            }
            SymbolRule::pri_short_ton_avoirdupois | SymbolRule::sec_short_ton_avoirdupois => {
                Atom::ShortTonAvoirdupois
            }
            SymbolRule::pri_siemens | SymbolRule::sec_siemens => Atom::Siemens,
            SymbolRule::pri_sievert | SymbolRule::sec_sievert => Atom::Sievert,
            SymbolRule::pri_smoot | SymbolRule::sec_smoot => Atom::Smoot,
            SymbolRule::pri_somogyi_unit | SymbolRule::sec_somogyi_unit => Atom::SomogyiUnit,
            SymbolRule::pri_spere | SymbolRule::sec_spere => Atom::Spere,
            SymbolRule::pri_square_foot_international
            | SymbolRule::sec_square_foot_international => Atom::SquareFootInternational,
            SymbolRule::pri_square_inch_international
            | SymbolRule::sec_square_inch_international => Atom::SquareInchInternational,
            SymbolRule::pri_square_mile_us | SymbolRule::sec_square_mile_us => Atom::SquareMileUS,
            SymbolRule::pri_square_rod_us | SymbolRule::sec_square_rod_us => Atom::SquareRodUS,
            SymbolRule::pri_square_yard_international
            | SymbolRule::sec_square_yard_international => Atom::SquareYardInternational,
            SymbolRule::pri_standard_acceleration_of_free_fall
            | SymbolRule::sec_standard_acceleration_of_free_fall => {
                Atom::StandardAccelerationOfFreeFall
            }
            SymbolRule::pri_standard_atmosphere | SymbolRule::sec_standard_atmosphere => {
                Atom::StandardAtmosphere
            }
            SymbolRule::pri_steradian | SymbolRule::sec_steradian => Atom::Steradian,
            SymbolRule::pri_stere | SymbolRule::sec_stere => Atom::Stere,
            SymbolRule::pri_stilb | SymbolRule::sec_stilb => Atom::Stilb,
            SymbolRule::pri_stokes | SymbolRule::sec_stokes => Atom::Stokes,
            SymbolRule::pri_stone_avoirdupois | SymbolRule::sec_stone_avoirdupois => {
                Atom::StoneAvoirdupois
            }
            SymbolRule::pri_svedberg_unit | SymbolRule::sec_svedberg_unit => Atom::SvedbergUnit,
            SymbolRule::pri_synodal_month | SymbolRule::sec_synodal_month => Atom::SynodalMonth,
            SymbolRule::pri_tablespoon_us | SymbolRule::sec_tablespoon_us => Atom::TablespoonUS,
            SymbolRule::pri_teaspoon_us | SymbolRule::sec_teaspoon_us => Atom::TeaspoonUS,
            SymbolRule::pri_technical_atmosphere | SymbolRule::sec_technical_atmosphere => {
                Atom::TechnicalAtmosphere
            }
            SymbolRule::pri_tesla | SymbolRule::sec_tesla => Atom::Tesla,
            SymbolRule::pri_tex | SymbolRule::sec_tex => Atom::Tex,
            SymbolRule::pri_the_number_pi | SymbolRule::sec_the_number_pi => Atom::TheNumberPi,
            SymbolRule::pri_the_number_ten_for_arbitrary_powers_caret
            | SymbolRule::sec_the_number_ten_for_arbitrary_powers_caret => {
                Atom::TheNumberTenForArbitraryPowersCaret
            }
            SymbolRule::pri_the_number_ten_for_arbitrary_powers_star
            | SymbolRule::sec_the_number_ten_for_arbitrary_powers_star => {
                Atom::TheNumberTenForArbitraryPowersStar
            }
            SymbolRule::pri_thermochemical_british_thermal_unit
            | SymbolRule::sec_thermochemical_british_thermal_unit => {
                Atom::ThermochemicalBritishThermalUnit
            }
            SymbolRule::pri_thermochemical_calorie | SymbolRule::sec_thermochemical_calorie => {
                Atom::ThermochemicalCalorie
            }
            SymbolRule::pri_tissue_culture_infectious_dose
            | SymbolRule::sec_tissue_culture_infectious_dose => Atom::TissueCultureInfectiousDose,
            SymbolRule::pri_todd_unit | SymbolRule::sec_todd_unit => Atom::ToddUnit,
            SymbolRule::pri_tonne | SymbolRule::sec_tonne => Atom::Tonne,
            SymbolRule::pri_township | SymbolRule::sec_township => Atom::Township,
            SymbolRule::pri_tropical_year | SymbolRule::sec_tropical_year => Atom::TropicalYear,
            SymbolRule::pri_tuberculin_unit | SymbolRule::sec_tuberculin_unit => {
                Atom::TuberculinUnit
            }
            SymbolRule::pri_unified_atomic_mass_unit | SymbolRule::sec_unified_atomic_mass_unit => {
                Atom::UnifiedAtomicMassUnit
            }
            SymbolRule::pri_unit | SymbolRule::sec_unit => Atom::Unit,
            SymbolRule::pri_united_states_pharmacopeia_unit
            | SymbolRule::sec_united_states_pharmacopeia_unit => Atom::UnitedStatesPharmacopeiaUnit,
            SymbolRule::pri_velocity_of_light | SymbolRule::sec_velocity_of_light => {
                Atom::VelocityOfLight
            }
            SymbolRule::pri_volt | SymbolRule::sec_volt => Atom::Volt,
            SymbolRule::pri_watt | SymbolRule::sec_watt => Atom::Watt,
            SymbolRule::pri_weber | SymbolRule::sec_weber => Atom::Weber,
            SymbolRule::pri_week | SymbolRule::sec_week => Atom::Week,
            SymbolRule::pri_wood_unit | SymbolRule::sec_wood_unit => Atom::WoodUnit,
            SymbolRule::pri_yard_british | SymbolRule::sec_yard_british => Atom::YardBritish,
            SymbolRule::pri_yard_international | SymbolRule::sec_yard_international => {
                Atom::YardInternational
            }
            SymbolRule::pri_yard_us | SymbolRule::sec_yard_us => Atom::YardUS,
            SymbolRule::pri_year | SymbolRule::sec_year => Atom::Year,
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
