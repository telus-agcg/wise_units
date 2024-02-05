use wise_units_parser::tokens::{
    Annotatable, AtomSymbol, Component, MainTerm, PrefixSymbol, Separator, Term,
};

use crate::{invert::Invert, Prefix};

use super::Term as WuTerm;

enum VariTerm {
    One(WuTerm),
    Many(Vec<WuTerm>),
}

impl From<VariTerm> for Vec<WuTerm> {
    fn from(value: VariTerm) -> Self {
        match value {
            VariTerm::One(t) => {
                vec![t]
            }
            VariTerm::Many(v) => v,
        }
    }
}

pub(super) trait FromToken<T> {
    fn from_token(token: T) -> Self;
}

impl<'input> FromToken<MainTerm<'input>> for Vec<WuTerm> {
    fn from_token(token: MainTerm<'input>) -> Self {
        if token.leading_slash() {
            let mut t = Self::from(VariTerm::from_token(token.into_term()));
            t.insert(
                0,
                WuTerm {
                    factor: None,
                    prefix: None,
                    atom: None,
                    exponent: Some(-1),
                    annotation: None,
                },
            );
            t
        } else {
            VariTerm::from_token(token.into_term()).into()
        }
    }
}

impl<'input> FromToken<Term<'input>> for VariTerm {
    fn from_token(token: Term<'input>) -> Self {
        match token {
            Term::Combined(component, separator, term) => {
                let lhs_component = Self::from_token(*component);
                let rhs_term = Self::from_token(*term);

                match (lhs_component, separator, rhs_term) {
                    (Self::One(lhs), Separator::Dot, Self::One(rhs)) => Self::Many(vec![lhs, rhs]),
                    (Self::One(lhs), Separator::Dot, Self::Many(mut rhs)) => {
                        rhs.insert(0, lhs);
                        Self::Many(rhs)
                    }
                    (Self::One(lhs), Separator::Slash, Self::One(mut rhs)) => {
                        rhs.invert();
                        Self::Many(vec![lhs, rhs])
                    }
                    (Self::One(lhs), Separator::Slash, Self::Many(mut rhs)) => {
                        rhs.invert();
                        rhs.insert(0, lhs);
                        Self::Many(rhs)
                    }
                    (Self::Many(mut lhs), Separator::Dot, Self::One(rhs)) => {
                        lhs.push(rhs);
                        Self::Many(lhs)
                    }
                    (Self::Many(mut lhs), Separator::Dot, Self::Many(rhs)) => {
                        lhs.extend(rhs);
                        Self::Many(lhs)
                    }
                    (Self::Many(mut lhs), Separator::Slash, Self::One(mut rhs)) => {
                        rhs.invert();
                        lhs.push(rhs);
                        Self::Many(lhs)
                    }
                    (Self::Many(mut lhs), Separator::Slash, Self::Many(mut rhs)) => {
                        rhs.invert();
                        lhs.extend(rhs);
                        Self::Many(lhs)
                    }
                }
            }
            Term::Basic(component) => Self::from_token(*component),
        }
    }
}

impl<'input> FromToken<Component<'input>> for VariTerm {
    fn from_token(token: Component<'input>) -> Self {
        match token {
            Component::AnnotatedAnnotatable(annotatable, annotation) => {
                let mut t = WuTerm::from_token(annotatable);
                t.annotation = Some(annotation.as_str().to_string());
                Self::One(t)
            }
            Component::Annotatable(annotatable) => Self::One(WuTerm::from_token(annotatable)),
            Component::Annotation(annotation) => Self::One(WuTerm {
                factor: None,
                prefix: None,
                atom: None,
                exponent: None,
                annotation: Some(annotation.as_str().to_string()),
            }),
            Component::Factor(factor) => Self::One(WuTerm {
                factor: Some(factor),
                prefix: None,
                atom: None,
                exponent: None,
                annotation: None,
            }),
            Component::NestedTerm(parser_term) => Self::from_token(parser_term),
        }
    }
}

impl FromToken<Annotatable> for WuTerm {
    fn from_token(token: Annotatable) -> Self {
        Self {
            factor: token.simple_unit().factor(),
            prefix: token.simple_unit().prefix_symbol().map(crate::Prefix::from),
            atom: Some(crate::Atom::from(token.simple_unit().atom_symbol())),
            exponent: token.exponent().map(i32::from),
            annotation: None,
        }
    }
}

impl<'a> From<&'a PrefixSymbol> for Prefix {
    fn from(value: &'a PrefixSymbol) -> Self {
        match value.primary_code() {
            "a" => Self::Atto,
            "c" => Self::Centi,
            "d" => Self::Deci,
            "da" => Self::Deka,
            "E" => Self::Exa,
            "f" => Self::Femto,
            "Gi" => Self::Gibi,
            "G" => Self::Giga,
            "h" => Self::Hecto,
            "Ki" => Self::Kibi,
            "k" => Self::Kilo,
            "Mi" => Self::Mebi,
            "M" => Self::Mega,
            "u" => Self::Micro,
            "m" => Self::Milli,
            "n" => Self::Nano,
            "P" => Self::Peta,
            "p" => Self::Pico,
            "Ti" => Self::Tebi,
            "T" => Self::Tera,
            "y" => Self::Yocto,
            "Y" => Self::Yotta,
            "z" => Self::Zepto,
            "Z" => Self::Zetta,
            _ => unreachable!(),
        }
    }
}

impl<'a> From<&'a AtomSymbol> for crate::Atom {
    fn from(value: &'a AtomSymbol) -> Self {
        match value.primary_code() {
            "m" => Self::Meter,
            "s" => Self::Second,
            "g" => Self::Gram,
            "rad" => Self::Radian,
            "K" => Self::Kelvin,
            "C" => Self::Coulomb,
            "cd" => Self::Candela,
            "10*" => Self::TheNumberTenForArbitraryPowersStar,
            "10^" => Self::TheNumberTenForArbitraryPowersCaret,
            "[pi]" => Self::TheNumberPi,
            "%" => Self::Percent,
            "[ppth]" => Self::PartsPerThousand,
            "[ppm]" => Self::PartsPerMillion,
            "[ppb]" => Self::PartsPerBillion,
            "[pptr]" => Self::PartsPerTrillion,
            "mol" => Self::Mole,
            "sr" => Self::Steradian,
            "Hz" => Self::Hertz,
            "N" => Self::Newton,
            "Pa" => Self::Pascal,
            "J" => Self::Joule,
            "W" => Self::Watt,
            "A" => Self::Ampere,
            "V" => Self::Volt,
            "F" => Self::Farad,
            "Ohm" => Self::Ohm,
            "S" => Self::Siemens,
            "Wb" => Self::Weber,
            "Cel" => Self::DegreeCelsius,
            "T" => Self::Tesla,
            "H" => Self::Henry,
            "lm" => Self::Lumen,
            "lx" => Self::Lux,
            "Bq" => Self::Becquerel,
            "Gy" => Self::Gray,
            "Sv" => Self::Sievert,
            "gon" => Self::Gon,
            "deg" => Self::Degree,
            "'" => Self::MinuteAngle,
            "''" => Self::SecondAngle,
            "l" => Self::Liter,
            "L" => Self::LiterSecondary,
            "ar" => Self::Are,
            "min" => Self::Minute,
            "h" => Self::Hour,
            "d" => Self::Day,
            "a_t" => Self::TropicalYear,
            "a_j" => Self::MeanJulianYear,
            "a_g" => Self::MeanGregorianYear,
            "a" => Self::Year,
            "wk" => Self::Week,
            "mo_s" => Self::SynodalMonth,
            "mo_j" => Self::MeanJulianMonth,
            "mo_g" => Self::MeanGregorianMonth,
            "mo" => Self::Month,
            "t" => Self::Tonne,
            "bar" => Self::Bar,
            "u" => Self::UnifiedAtomicMassUnit,
            "eV" => Self::Electronvolt,
            "AU" => Self::AstronomicUnit,
            "pc" => Self::Parsec,
            "[c]" => Self::VelocityOfLight,
            "[h]" => Self::PlanckConstant,
            "[k]" => Self::BoltzmannConstant,
            "[eps_0]" => Self::PermittivityOfVacuum,
            "[mu_0]" => Self::PermeabilityOfVacuum,
            "[e]" => Self::ElementaryCharge,
            "[m_e]" => Self::ElectronMass,
            "[m_p]" => Self::ProtonMass,
            "[G]" => Self::NewtonianConstantOfGravitation,
            "[g]" => Self::StandardAccelerationOfFreeFall,
            "atm" => Self::StandardAtmosphere,
            "[ly]" => Self::LightYear,
            "gf" => Self::GramForce,
            "[lbf_av]" => Self::PoundForceAvoirdupois,
            "Ky" => Self::Kayser,
            "Gal" => Self::Gal,
            "dyn" => Self::Dyne,
            "erg" => Self::Erg,
            "P" => Self::Poise,
            "Bi" => Self::Biot,
            "St" => Self::Stokes,
            "Mx" => Self::Maxwell,
            "G" => Self::Gauss,
            "Oe" => Self::Oersted,
            "Gb" => Self::Gilbert,
            "sb" => Self::Stilb,
            "Lmb" => Self::Lambert,
            "ph" => Self::Phot,
            "Ci" => Self::Curie,
            "R" => Self::Roentgen,
            "RAD" => Self::RadiationAbsorbedDose,
            "REM" => Self::RadiationEquivalentMan,
            "[in_i]" => Self::InchInternational,
            "[ft_i]" => Self::FootInternational,
            "[yd_i]" => Self::YardInternational,
            "[mi_i]" => Self::MileInternational,
            "[fth_i]" => Self::FathomInternational,
            "[nmi_i]" => Self::NauticalMileInternational,
            "[kn_i]" => Self::KnotInternational,
            "[sin_i]" => Self::SquareInchInternational,
            "[sft_i]" => Self::SquareFootInternational,
            "[syd_i]" => Self::SquareYardInternational,
            "[cin_i]" => Self::CubicInchInternational,
            "[cft_i]" => Self::CubicFootInternational,
            "[cyd_i]" => Self::CubicYardInternational,
            "[bf_i]" => Self::BoardFootInternational,
            "[cr_i]" => Self::CordInternational,
            "[mil_i]" => Self::MilInternational,
            "[cml_i]" => Self::CircularMilInternational,
            "[hd_i]" => Self::HandInternational,
            "[ft_us]" => Self::FootUS,
            "[yd_us]" => Self::YardUS,
            "[in_us]" => Self::InchUS,
            "[rd_us]" => Self::RodUS,
            "[ch_us]" => Self::GuntersChainUS,
            "[lk_us]" => Self::LinkForGuntersChainUS,
            "[rch_us]" => Self::RamdensChainUS,
            "[rlk_us]" => Self::LinkForRamdensChainUS,
            "[fth_us]" => Self::FathomUS,
            "[fur_us]" => Self::FurlongUS,
            "[mi_us]" => Self::MileUS,
            "[acr_us]" => Self::AcreUS,
            "[srd_us]" => Self::SquareRodUS,
            "[smi_us]" => Self::SquareMileUS,
            "[sct]" => Self::Section,
            "[twp]" => Self::Township,
            "[mil_us]" => Self::MilUS,
            "[in_br]" => Self::InchBritish,
            "[ft_br]" => Self::FootBritish,
            "[rd_br]" => Self::RodBritish,
            "[ch_br]" => Self::GuntersChainBritish,
            "[lk_br]" => Self::LinkForGuntersChainBritish,
            "[fth_br]" => Self::FathomBritish,
            "[pc_br]" => Self::PaceBritish,
            "[yd_br]" => Self::YardBritish,
            "[mi_br]" => Self::MileBritish,
            "[nmi_br]" => Self::NauticalMileBritish,
            "[kn_br]" => Self::KnotBritish,
            "[acr_br]" => Self::AcreBritish,
            "[gal_us]" => Self::QueenAnnesWineGallonUS,
            "[bbl_us]" => Self::BarrelUS,
            "[qt_us]" => Self::QuartUS,
            "[pt_us]" => Self::PintUS,
            "[gil_us]" => Self::GillUS,
            "[foz_us]" => Self::FluidOunceUS,
            "[fdr_us]" => Self::FluidDramUS,
            "[min_us]" => Self::MinimUS,
            "[crd_us]" => Self::CordUS,
            "[bu_us]" => Self::BushelUS,
            "[gal_wi]" => Self::HistoricalWinchesterGallon,
            "[pk_us]" => Self::PeckUS,
            "[dqt_us]" => Self::DryQuartUS,
            "[dpt_us]" => Self::DryPintUS,
            "[tbs_us]" => Self::TablespoonUS,
            "[tsp_us]" => Self::TeaspoonUS,
            "[cup_us]" => Self::CupUS,
            "[foz_m]" => Self::MetricFluidOunce,
            "[cup_m]" => Self::MetricCup,
            "[tsp_m]" => Self::MetricTeaspoon,
            "[tbs_m]" => Self::MetricTablespoon,
            "[gal_br]" => Self::GallonBritish,
            "[pk_br]" => Self::PeckBritish,
            "[bu_br]" => Self::BushelBritish,
            "[qt_br]" => Self::QuartBritish,
            "[pt_br]" => Self::PintBritish,
            "[gil_br]" => Self::GillBritish,
            "[foz_br]" => Self::FluidOunceBritish,
            "[fdr_br]" => Self::FluidDramBritish,
            "[min_br]" => Self::MinimBritish,
            "[gr]" => Self::Grain,
            "[lb_av]" => Self::PoundAvoirdupois,
            "[oz_av]" => Self::OunceAvoirdupois,
            "[dr_av]" => Self::DramAvoirdupois,
            "[scwt_av]" => Self::ShortHundredweightAvoirdupois,
            "[lcwt_av]" => Self::LongHunderdweightAvoirdupois,
            "[ston_av]" => Self::ShortTonAvoirdupois,
            "[lton_av]" => Self::LongTonAvoirdupois,
            "[stone_av]" => Self::StoneAvoirdupois,
            "[pwt_tr]" => Self::PennyweightTroy,
            "[oz_tr]" => Self::OunceTroy,
            "[lb_tr]" => Self::PoundTroy,
            "[sc_ap]" => Self::ScrupleApothecaries,
            "[dr_ap]" => Self::DramApothecaries,
            "[oz_ap]" => Self::OunceApothecaries,
            "[lb_ap]" => Self::PoundApothecaries,
            "[oz_m]" => Self::MetricOunce,
            "[lne]" => Self::Line,
            "[pnt]" => Self::Point,
            "[pca]" => Self::Pica,
            "[pnt_pr]" => Self::PrintersPoint,
            "[pca_pr]" => Self::PrintersPica,
            "[pied]" => Self::Pied,
            "[pouce]" => Self::Pouce,
            "[ligne]" => Self::Ligne,
            "[didot]" => Self::Didot,
            "[cicero]" => Self::Cicero,
            "[degF]" => Self::DegreeFahrenheit,
            "[degR]" => Self::DegreeRankine,
            "[degRe]" => Self::DegreeReaumur,
            "cal_[15]" => Self::CalorieAt15C,
            "cal_[20]" => Self::CalorieAt20C,
            "cal_m" => Self::MeanCalorie,
            "cal_IT" => Self::InternationalTableCalorie,
            "cal_th" => Self::ThermochemicalCalorie,
            "cal" => Self::Calorie,
            "[Cal]" => Self::NutritionLabelCalories,
            "[Btu_39]" => Self::BritishThermalUnitAt39F,
            "[Btu_59]" => Self::BritishThermalUnitAt59F,
            "[Btu_60]" => Self::BritishThermalUnitAt60F,
            "[Btu_m]" => Self::MeanBritishThermalUnit,
            "[Btu_IT]" => Self::InternationalTableBritishThermalUnit,
            "[Btu_th]" => Self::ThermochemicalBritishThermalUnit,
            "[Btu]" => Self::BritishThermalUnit,
            "[HP]" => Self::Horsepower,
            "tex" => Self::Tex,
            "[den]" => Self::Denier,
            "m[H2O]" => Self::MeterOfWaterColumn,
            "m[Hg]" => Self::MeterOfMercuryColumn,
            "[in_i'H2O]" => Self::InchOfWaterColumn,
            "[in_i'Hg]" => Self::InchOfMercuryColumn,
            "[PRU]" => Self::PeripheralVascularResistanceUnit,
            "[wood'U]" => Self::WoodUnit,
            "[diop]" => Self::Diopter,
            "[p'diop]" => Self::PrismDiopter,
            "%[slope]" => Self::PercentOfSlope,
            "[mesh_i]" => Self::MeshInternational,
            "[Ch]" => Self::Charriere,
            "[drp]" => Self::Drop,
            "[hnsf'U]" => Self::HounsfieldUnit,
            "[MET]" => Self::MetabolicEquivalent,
            "[hp'_X]" => Self::HomeopathicPotencyOfDecimalSeriesRetired,
            "[hp'_C]" => Self::HomeopathicPotencyOfCentesimalSeriesRetired,
            "[hp'_M]" => Self::HomeopathicPotencyOfMillesimalSeriesRetired,
            "[hp'_Q]" => Self::HomeopathicPotencyOfQuintamillesimalSeriesRetired,
            "[hp_X]" => Self::HomeopathicPotencyOfDecimalHahnemannianSeries,
            "[hp_C]" => Self::HomeopathicPotencyOfCentesimalHahnemannianSeries,
            "[hp_M]" => Self::HomeopathicPotencyOfMillesimalHahnemannianSeries,
            "[hp_Q]" => Self::HomeopathicPotencyOfQuintamillesimalHahnemannianSeries,
            "[kp_X]" => Self::HomeopathicPotencyOfDecimalKorsakovianSeries,
            "[kp_C]" => Self::HomeopathicPotencyOfCentesimalKorsakovianSeries,
            "[kp_M]" => Self::HomeopathicPotencyOfMillesimalKorsakovianSeries,
            "[kp_Q]" => Self::HomeopathicPotencyOfQuintamillesimalKorsakovianSeries,
            "eq" => Self::Equivalents,
            "osm" => Self::Osmole,
            "[pH]" => Self::PH,
            "g%" => Self::GramPercent,
            "[S]" => Self::SvedbergUnit,
            "[HPF]" => Self::HighPowerField,
            "[LPF]" => Self::LowPowerField,
            "kat" => Self::Katal,
            "U" => Self::Unit,
            "[iU]" => Self::InternationalUnit,
            "[IU]" => Self::InternationalUnitSecondary,
            "[arb'U]" => Self::ArbitraryUnit,
            "[USP'U]" => Self::UnitedStatesPharmacopeiaUnit,
            "[GPL'U]" => Self::GplUnit,
            "[MPL'U]" => Self::MplUnit,
            "[APL'U]" => Self::AplUnit,
            "[beth'U]" => Self::BethesdaUnit,
            "[anti'Xa'U]" => Self::AntiFactorXaUnit,
            "[todd'U]" => Self::ToddUnit,
            "[dye'U]" => Self::DyeUnit,
            "[smgy'U]" => Self::SomogyiUnit,
            "[bdsk'U]" => Self::BodanskyUnit,
            "[ka'U]" => Self::KingArmstrongUnit,
            "[knk'U]" => Self::KunkelUnit,
            "[mclg'U]" => Self::MacLaganUnit,
            "[tb'U]" => Self::TuberculinUnit,
            "[CCID_50]" => Self::CellCultureInfectiousDose,
            "[TCID_50]" => Self::TissueCultureInfectiousDose,
            "[EID_50]" => Self::EmbryoInfectiousDose,
            "[PFU]" => Self::PlaqueFormingUnits,
            "[FFU]" => Self::FocusFormingUnits,
            "[CFU]" => Self::ColonyFormingUnits,
            "[IR]" => Self::IndexOfReactivity,
            "[BAU]" => Self::BioequivalentAllergenUnit,
            "[AU]" => Self::AllergenUnit,
            "[Amb'a'1'U]" => Self::AllergenUnitForAmbrosiaArtemisiifolia,
            "[PNU]" => Self::ProteinNitrogenUnit,
            "[Lf]" => Self::LimitOfFlocculation,
            "[D'ag'U]" => Self::DAntigenUnit,
            "[FEU]" => Self::FibrinogenEquivalentUnit,
            "[ELU]" => Self::ElisaUnit,
            "[EU]" => Self::EhrlichUnit,
            "Np" => Self::Neper,
            "B" => Self::Bel,
            "B[SPL]" => Self::BelSoundPressure,
            "B[V]" => Self::BelVolt,
            "B[mV]" => Self::BelMillivolt,
            "B[uV]" => Self::BelMicrovolt,
            "B[10.nV]" => Self::Bel10Nanovolt,
            "B[W]" => Self::BelWatt,
            "B[kW]" => Self::BelKilowatt,
            "st" => Self::Stere,
            "Ao" => Self::Angstrom,
            "b" => Self::Barn,
            "att" => Self::TechnicalAtmosphere,
            "mho" => Self::Mho,
            "[psi]" => Self::PoundPerSqareInch,
            "circ" => Self::Circle,
            "sph" => Self::Spere,
            "[car_m]" => Self::MetricCarat,
            "[car_Au]" => Self::CaratOfGoldAlloys,
            "[smoot]" => Self::Smoot,
            "[m/s2/Hz^(1/2)]" => Self::MeterPerSquareSecondsPerSquareRootOfHertz,
            "bit_s" => Self::BitLogarithmusDualis,
            "bit" => Self::Bit,
            "By" => Self::Byte,
            "Bd" => Self::Baud,
            _ => unreachable!()
        }
    }
}
