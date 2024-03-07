use pest::{iterators::Pairs, pratt_parser::PrattParser};

use crate::{Atom, Prefix};

use super::{Parse, Rule};

#[derive(Debug, PartialEq)]
pub(in crate::unit) enum SimpleUnit<'i> {
    PrefixAtom {
        prefix_symbol: &'i str,
        atom_symbol: &'i str,
    },
    Atom(&'i str),
}

impl<'i> Parse<'i> for SimpleUnit<'i> {
    fn parse(pairs: Pairs<'i, Rule>, pratt: &PrattParser<Rule>) -> Self {
        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::metric_atom_symbol | Rule::non_metric_atom_symbol => {
                    SimpleUnit::Atom(primary.as_str())
                }
                rule => unreachable!("expected *atom_symbol, found {rule:?}"),
            })
            .map_prefix(|prefix, atom| match prefix.as_rule() {
                Rule::prefix_symbol => match atom {
                    SimpleUnit::Atom(atom_symbol) => SimpleUnit::PrefixAtom {
                        prefix_symbol: prefix.as_str(),
                        atom_symbol,
                    },
                    SimpleUnit::PrefixAtom { .. } => unreachable!(),
                },
                rule => unreachable!("expected factor, found {rule:?}"),
            })
            .parse(pairs)
    }
}

impl<'i> From<SimpleUnit<'i>> for (Option<Prefix>, Option<Atom>) {
    fn from(value: SimpleUnit<'i>) -> Self {
        match value {
            SimpleUnit::PrefixAtom {
                prefix_symbol,
                atom_symbol,
            } => (
                Some(prefix_symbol_to_prefix(prefix_symbol)),
                Some(atom_symbol_to_atom(atom_symbol)),
            ),
            SimpleUnit::Atom(atom_symbol) => (None, Some(atom_symbol_to_atom(atom_symbol))),
        }
    }
}

// We can skip error handling here because the `prefix_symbol` was picked up by the parser, and all
// of those strings must match a `crate::Prefix`'s `primary_symbol`.
//
fn prefix_symbol_to_prefix(prefix_symbol: &str) -> Prefix {
    match prefix_symbol {
        "Y" => Prefix::Yotta,
        "Z" => Prefix::Zetta,
        "E" => Prefix::Exa,
        "P" => Prefix::Peta,
        "T" => Prefix::Tera,
        "G" => Prefix::Giga,
        "M" => Prefix::Mega,
        "k" => Prefix::Kilo,
        "h" => Prefix::Hecto,
        "da" => Prefix::Deka,
        "d" => Prefix::Deci,
        "c" => Prefix::Centi,
        "m" => Prefix::Milli,
        "u" => Prefix::Micro,
        "n" => Prefix::Nano,
        "p" => Prefix::Pico,
        "f" => Prefix::Femto,
        "a" => Prefix::Atto,
        "z" => Prefix::Zepto,
        "y" => Prefix::Yocto,
        symbol => unreachable!("unknown prefix symbol: {symbol:#?}"),
    }
}

// We can skip error handling here because the `atom_symbol` was picked up by the parser, and all
// of those strings must match a `crate::Atom`'s `primary_symbol`.
//
#[allow(clippy::too_many_lines)]
fn atom_symbol_to_atom(atom_symbol: &str) -> Atom {
    match atom_symbol {
        "m" => Atom::Meter,
        "s" => Atom::Second,
        "g" => Atom::Gram,
        "rad" => Atom::Radian,
        "K" => Atom::Kelvin,
        "C" => Atom::Coulomb,
        "cd" => Atom::Candela,
        "10*" => Atom::TheNumberTenForArbitraryPowersStar,
        "10^" => Atom::TheNumberTenForArbitraryPowersCaret,
        "[pi]" => Atom::TheNumberPi,
        "%" => Atom::Percent,
        "[ppth]" => Atom::PartsPerThousand,
        "[ppm]" => Atom::PartsPerMillion,
        "[ppb]" => Atom::PartsPerBillion,
        "[pptr]" => Atom::PartsPerTrillion,
        "mol" => Atom::Mole,
        "sr" => Atom::Steradian,
        "Hz" => Atom::Hertz,
        "N" => Atom::Newton,
        "Pa" => Atom::Pascal,
        "J" => Atom::Joule,
        "W" => Atom::Watt,
        "A" => Atom::Ampere,
        "V" => Atom::Volt,
        "F" => Atom::Farad,
        "Ohm" => Atom::Ohm,
        "S" => Atom::Siemens,
        "Wb" => Atom::Weber,
        "Cel" => Atom::DegreeCelsius,
        "T" => Atom::Tesla,
        "H" => Atom::Henry,
        "lm" => Atom::Lumen,
        "lx" => Atom::Lux,
        "Bq" => Atom::Becquerel,
        "Gy" => Atom::Gray,
        "Sv" => Atom::Sievert,
        "gon" => Atom::Gon,
        "deg" => Atom::Degree,
        "'" => Atom::MinuteAngle,
        "''" => Atom::SecondAngle,
        "l" => Atom::Liter,
        "L" => Atom::LiterSecondary,
        "ar" => Atom::Are,
        "min" => Atom::Minute,
        "h" => Atom::Hour,
        "d" => Atom::Day,
        "a_t" => Atom::TropicalYear,
        "a_j" => Atom::MeanJulianYear,
        "a_g" => Atom::MeanGregorianYear,
        "a" => Atom::Year,
        "wk" => Atom::Week,
        "mo_s" => Atom::SynodalMonth,
        "mo_j" => Atom::MeanJulianMonth,
        "mo_g" => Atom::MeanGregorianMonth,
        "mo" => Atom::Month,
        "t" => Atom::Tonne,
        "bar" => Atom::Bar,
        "u" => Atom::UnifiedAtomicMassUnit,
        "eV" => Atom::Electronvolt,
        "AU" => Atom::AstronomicUnit,
        "pc" => Atom::Parsec,
        "[c]" => Atom::VelocityOfLight,
        "[h]" => Atom::PlanckConstant,
        "[k]" => Atom::BoltzmannConstant,
        "[eps_0]" => Atom::PermittivityOfVacuum,
        "[mu_0]" => Atom::PermeabilityOfVacuum,
        "[e]" => Atom::ElementaryCharge,
        "[m_e]" => Atom::ElectronMass,
        "[m_p]" => Atom::ProtonMass,
        "[G]" => Atom::NewtonianConstantOfGravitation,
        "[g]" => Atom::StandardAccelerationOfFreeFall,
        "atm" => Atom::StandardAtmosphere,
        "[ly]" => Atom::LightYear,
        "gf" => Atom::GramForce,
        "[lbf_av]" => Atom::PoundForceAvoirdupois,
        "Ky" => Atom::Kayser,
        "Gal" => Atom::Gal,
        "dyn" => Atom::Dyne,
        "erg" => Atom::Erg,
        "P" => Atom::Poise,
        "Bi" => Atom::Biot,
        "St" => Atom::Stokes,
        "Mx" => Atom::Maxwell,
        "G" => Atom::Gauss,
        "Oe" => Atom::Oersted,
        "Gb" => Atom::Gilbert,
        "sb" => Atom::Stilb,
        "Lmb" => Atom::Lambert,
        "ph" => Atom::Phot,
        "Ci" => Atom::Curie,
        "R" => Atom::Roentgen,
        "RAD" => Atom::RadiationAbsorbedDose,
        "REM" => Atom::RadiationEquivalentMan,
        "[in_i]" => Atom::InchInternational,
        "[ft_i]" => Atom::FootInternational,
        "[yd_i]" => Atom::YardInternational,
        "[mi_i]" => Atom::MileInternational,
        "[fth_i]" => Atom::FathomInternational,
        "[nmi_i]" => Atom::NauticalMileInternational,
        "[kn_i]" => Atom::KnotInternational,
        "[sin_i]" => Atom::SquareInchInternational,
        "[sft_i]" => Atom::SquareFootInternational,
        "[syd_i]" => Atom::SquareYardInternational,
        "[cin_i]" => Atom::CubicInchInternational,
        "[cft_i]" => Atom::CubicFootInternational,
        "[cyd_i]" => Atom::CubicYardInternational,
        "[bf_i]" => Atom::BoardFootInternational,
        "[cr_i]" => Atom::CordInternational,
        "[mil_i]" => Atom::MilInternational,
        "[cml_i]" => Atom::CircularMilInternational,
        "[hd_i]" => Atom::HandInternational,
        "[ft_us]" => Atom::FootUS,
        "[yd_us]" => Atom::YardUS,
        "[in_us]" => Atom::InchUS,
        "[rd_us]" => Atom::RodUS,
        "[ch_us]" => Atom::GuntersChainUS,
        "[lk_us]" => Atom::LinkForGuntersChainUS,
        "[rch_us]" => Atom::RamdensChainUS,
        "[rlk_us]" => Atom::LinkForRamdensChainUS,
        "[fth_us]" => Atom::FathomUS,
        "[fur_us]" => Atom::FurlongUS,
        "[mi_us]" => Atom::MileUS,
        "[acr_us]" => Atom::AcreUS,
        "[srd_us]" => Atom::SquareRodUS,
        "[smi_us]" => Atom::SquareMileUS,
        "[sct]" => Atom::Section,
        "[twp]" => Atom::Township,
        "[mil_us]" => Atom::MilUS,
        "[in_br]" => Atom::InchBritish,
        "[ft_br]" => Atom::FootBritish,
        "[rd_br]" => Atom::RodBritish,
        "[ch_br]" => Atom::GuntersChainBritish,
        "[lk_br]" => Atom::LinkForGuntersChainBritish,
        "[fth_br]" => Atom::FathomBritish,
        "[pc_br]" => Atom::PaceBritish,
        "[yd_br]" => Atom::YardBritish,
        "[mi_br]" => Atom::MileBritish,
        "[nmi_br]" => Atom::NauticalMileBritish,
        "[kn_br]" => Atom::KnotBritish,
        "[acr_br]" => Atom::AcreBritish,
        "[gal_us]" => Atom::QueenAnnesWineGallonUS,
        "[bbl_us]" => Atom::BarrelUS,
        "[qt_us]" => Atom::QuartUS,
        "[pt_us]" => Atom::PintUS,
        "[gil_us]" => Atom::GillUS,
        "[foz_us]" => Atom::FluidOunceUS,
        "[fdr_us]" => Atom::FluidDramUS,
        "[min_us]" => Atom::MinimUS,
        "[crd_us]" => Atom::CordUS,
        "[bu_us]" => Atom::BushelUS,
        "[gal_wi]" => Atom::HistoricalWinchesterGallon,
        "[pk_us]" => Atom::PeckUS,
        "[dqt_us]" => Atom::DryQuartUS,
        "[dpt_us]" => Atom::DryPintUS,
        "[tbs_us]" => Atom::TablespoonUS,
        "[tsp_us]" => Atom::TeaspoonUS,
        "[cup_us]" => Atom::CupUS,
        "[foz_m]" => Atom::MetricFluidOunce,
        "[cup_m]" => Atom::MetricCup,
        "[tsp_m]" => Atom::MetricTeaspoon,
        "[tbs_m]" => Atom::MetricTablespoon,
        "[gal_br]" => Atom::GallonBritish,
        "[pk_br]" => Atom::PeckBritish,
        "[bu_br]" => Atom::BushelBritish,
        "[qt_br]" => Atom::QuartBritish,
        "[pt_br]" => Atom::PintBritish,
        "[gil_br]" => Atom::GillBritish,
        "[foz_br]" => Atom::FluidOunceBritish,
        "[fdr_br]" => Atom::FluidDramBritish,
        "[min_br]" => Atom::MinimBritish,
        "[gr]" => Atom::Grain,
        "[lb_av]" => Atom::PoundAvoirdupois,
        "[oz_av]" => Atom::OunceAvoirdupois,
        "[dr_av]" => Atom::DramAvoirdupois,
        "[scwt_av]" => Atom::ShortHundredweightAvoirdupois,
        "[lcwt_av]" => Atom::LongHunderdweightAvoirdupois,
        "[ston_av]" => Atom::ShortTonAvoirdupois,
        "[lton_av]" => Atom::LongTonAvoirdupois,
        "[stone_av]" => Atom::StoneAvoirdupois,
        "[pwt_tr]" => Atom::PennyweightTroy,
        "[oz_tr]" => Atom::OunceTroy,
        "[lb_tr]" => Atom::PoundTroy,
        "[sc_ap]" => Atom::ScrupleApothecaries,
        "[dr_ap]" => Atom::DramApothecaries,
        "[oz_ap]" => Atom::OunceApothecaries,
        "[lb_ap]" => Atom::PoundApothecaries,
        "[oz_m]" => Atom::MetricOunce,
        "[lne]" => Atom::Line,
        "[pnt]" => Atom::Point,
        "[pca]" => Atom::Pica,
        "[pnt_pr]" => Atom::PrintersPoint,
        "[pca_pr]" => Atom::PrintersPica,
        "[pied]" => Atom::Pied,
        "[pouce]" => Atom::Pouce,
        "[ligne]" => Atom::Ligne,
        "[didot]" => Atom::Didot,
        "[cicero]" => Atom::Cicero,
        "[degF]" => Atom::DegreeFahrenheit,
        "[degR]" => Atom::DegreeRankine,
        "[degRe]" => Atom::DegreeReaumur,
        "cal_[15]" => Atom::CalorieAt15C,
        "cal_[20]" => Atom::CalorieAt20C,
        "cal_m" => Atom::MeanCalorie,
        "cal_IT" => Atom::InternationalTableCalorie,
        "cal_th" => Atom::ThermochemicalCalorie,
        "cal" => Atom::Calorie,
        "[Cal]" => Atom::NutritionLabelCalories,
        "[Btu_39]" => Atom::BritishThermalUnitAt39F,
        "[Btu_59]" => Atom::BritishThermalUnitAt59F,
        "[Btu_60]" => Atom::BritishThermalUnitAt60F,
        "[Btu_m]" => Atom::MeanBritishThermalUnit,
        "[Btu_IT]" => Atom::InternationalTableBritishThermalUnit,
        "[Btu_th]" => Atom::ThermochemicalBritishThermalUnit,
        "[Btu]" => Atom::BritishThermalUnit,
        "[HP]" => Atom::Horsepower,
        "tex" => Atom::Tex,
        "[den]" => Atom::Denier,
        "m[H2O]" => Atom::MeterOfWaterColumn,
        "m[Hg]" => Atom::MeterOfMercuryColumn,
        "[in_i'H2O]" => Atom::InchOfWaterColumn,
        "[in_i'Hg]" => Atom::InchOfMercuryColumn,
        "[PRU]" => Atom::PeripheralVascularResistanceUnit,
        "[wood'U]" => Atom::WoodUnit,
        "[diop]" => Atom::Diopter,
        "[p'diop]" => Atom::PrismDiopter,
        "%[slope]" => Atom::PercentOfSlope,
        "[mesh_i]" => Atom::MeshInternational,
        "[Ch]" => Atom::Charriere,
        "[drp]" => Atom::Drop,
        "[hnsf'U]" => Atom::HounsfieldUnit,
        "[MET]" => Atom::MetabolicEquivalent,
        "[hp'_X]" => Atom::HomeopathicPotencyOfDecimalSeriesRetired,
        "[hp'_C]" => Atom::HomeopathicPotencyOfCentesimalSeriesRetired,
        "[hp'_M]" => Atom::HomeopathicPotencyOfMillesimalSeriesRetired,
        "[hp'_Q]" => Atom::HomeopathicPotencyOfQuintamillesimalSeriesRetired,
        "[hp_X]" => Atom::HomeopathicPotencyOfDecimalHahnemannianSeries,
        "[hp_C]" => Atom::HomeopathicPotencyOfCentesimalHahnemannianSeries,
        "[hp_M]" => Atom::HomeopathicPotencyOfMillesimalHahnemannianSeries,
        "[hp_Q]" => Atom::HomeopathicPotencyOfQuintamillesimalHahnemannianSeries,
        "[kp_X]" => Atom::HomeopathicPotencyOfDecimalKorsakovianSeries,
        "[kp_C]" => Atom::HomeopathicPotencyOfCentesimalKorsakovianSeries,
        "[kp_M]" => Atom::HomeopathicPotencyOfMillesimalKorsakovianSeries,
        "[kp_Q]" => Atom::HomeopathicPotencyOfQuintamillesimalKorsakovianSeries,
        "eq" => Atom::Equivalents,
        "osm" => Atom::Osmole,
        "[pH]" => Atom::PH,
        "g%" => Atom::GramPercent,
        "[S]" => Atom::SvedbergUnit,
        "[HPF]" => Atom::HighPowerField,
        "[LPF]" => Atom::LowPowerField,
        "kat" => Atom::Katal,
        "U" => Atom::Unit,
        "[iU]" => Atom::InternationalUnit,
        "[IU]" => Atom::InternationalUnitSecondary,
        "[arb'U]" => Atom::ArbitraryUnit,
        "[USP'U]" => Atom::UnitedStatesPharmacopeiaUnit,
        "[GPL'U]" => Atom::GplUnit,
        "[MPL'U]" => Atom::MplUnit,
        "[APL'U]" => Atom::AplUnit,
        "[beth'U]" => Atom::BethesdaUnit,
        "[anti'Xa'U]" => Atom::AntiFactorXaUnit,
        "[todd'U]" => Atom::ToddUnit,
        "[dye'U]" => Atom::DyeUnit,
        "[smgy'U]" => Atom::SomogyiUnit,
        "[bdsk'U]" => Atom::BodanskyUnit,
        "[ka'U]" => Atom::KingArmstrongUnit,
        "[knk'U]" => Atom::KunkelUnit,
        "[mclg'U]" => Atom::MacLaganUnit,
        "[tb'U]" => Atom::TuberculinUnit,
        "[CCID_50]" => Atom::CellCultureInfectiousDose,
        "[TCID_50]" => Atom::TissueCultureInfectiousDose,
        "[EID_50]" => Atom::EmbryoInfectiousDose,
        "[PFU]" => Atom::PlaqueFormingUnits,
        "[FFU]" => Atom::FocusFormingUnits,
        "[CFU]" => Atom::ColonyFormingUnits,
        "[IR]" => Atom::IndexOfReactivity,
        "[BAU]" => Atom::BioequivalentAllergenUnit,
        "[AU]" => Atom::AllergenUnit,
        "[Amb'a'1'U]" => Atom::AllergenUnitForAmbrosiaArtemisiifolia,
        "[PNU]" => Atom::ProteinNitrogenUnit,
        "[Lf]" => Atom::LimitOfFlocculation,
        "[D'ag'U]" => Atom::DAntigenUnit,
        "[FEU]" => Atom::FibrinogenEquivalentUnit,
        "[ELU]" => Atom::ElisaUnit,
        "[EU]" => Atom::EhrlichUnit,
        "Np" => Atom::Neper,
        "B" => Atom::Bel,
        "B[SPL]" => Atom::BelSoundPressure,
        "B[V]" => Atom::BelVolt,
        "B[mV]" => Atom::BelMillivolt,
        "B[uV]" => Atom::BelMicrovolt,
        "B[10.nV]" => Atom::Bel10Nanovolt,
        "B[W]" => Atom::BelWatt,
        "B[kW]" => Atom::BelKilowatt,
        "st" => Atom::Stere,
        "Ao" => Atom::Angstrom,
        "b" => Atom::Barn,
        "att" => Atom::TechnicalAtmosphere,
        "mho" => Atom::Mho,
        "[psi]" => Atom::PoundPerSqareInch,
        "circ" => Atom::Circle,
        "sph" => Atom::Spere,
        "[car_m]" => Atom::MetricCarat,
        "[car_Au]" => Atom::CaratOfGoldAlloys,
        "[smoot]" => Atom::Smoot,
        "[m/s2/Hz^(1/2)]" => Atom::MeterPerSquareSecondsPerSquareRootOfHertz,
        "bit_s" => Atom::BitLogarithmusDualis,
        "bit" => Atom::Bit,
        "By" => Atom::Byte,
        "Bd" => Atom::Baud,
        symbol => unreachable!("unknown atom symbol: {symbol:#?}"),
    }
}
