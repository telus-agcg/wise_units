// Internal structs for mapping parser Rule data to an intermediate
// representation of a Unit.
pub(self) mod annotatable;
pub(self) mod ast_term;
pub(self) mod basic_component;
pub(self) mod component;
pub(self) mod main_term;
pub(self) mod simple_unit;

use atom::Atom;
use error::Error;
use pest::iterators::{Pair, Pairs};
use prefix::Prefix;
use self::annotatable::Annotatable;
use self::ast_term::AstTerm;
use self::basic_component::BasicComponent;
use self::component::Component;
use self::main_term::MainTerm;
use self::simple_unit::SimpleUnit;
use term::Term;
use unit::Unit;
use unit_parser::Rule;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, pairs: Pairs<Rule>) -> Result<Unit, Error> {
        let unit = self.visit_pairs(pairs)?;

        Ok(unit)
    }

    fn visit_pairs(&mut self, pairs: Pairs<Rule>) -> Result<Unit, Error> {
        let mut main_term = MainTerm::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_term => {
                    let child_main_term = self.visit_main_term(pair)?;

                    main_term = child_main_term;
                }
                _ => {
                    println!("visit_with_pairs: unreachable rule: {:?}", pair);
                    unreachable!()
                }
            };
        }

        Ok(main_term.into())
    }

    fn visit_atom_symbol(&mut self, pair: Pair<Rule>) -> Result<Atom, Error> {
        let pair_str = pair.into_span();

        let atom = match pair_str.as_str() {
            // Base units first.
            "1"                       => Atom::TheUnity,
                        "cd"  | "CD"  => Atom::Candela,
                        "C"           => Atom::Coulomb,
            "gram"    | "g"           => Atom::Gram,
                        "K"           => Atom::Kelvin,
            "meter"   | "m"   | "M"   => Atom::Meter,
                        "rad"         => Atom::Radian,
            "second"  | "s"           => Atom::Second,

            // Derived units last.
                            "[acr_br]"  | "[ACR_BR]"           => Atom::AcreBR,
            "acre"        | "[acr_us]"  | "[ACR_US]"           => Atom::AcreUS,
                            "A"                                => Atom::Ampere,
            "are"         | "ar"        | "AR"                 => Atom::Are,
                            "AU"        | "ASU"                => Atom::AstronomicUnit,
                            "u"         | "AMU"                => Atom::AtomicMassUnit,
                            "bar"       | "BAR"                => Atom::Bar,
                            "[bbl_us]"  | "[BBL_US]"           => Atom::BarrelUS,
                            "Bq"        | "BQ"                 => Atom::Becquerel,
                            "Bi"        | "BI"                 => Atom::Biot,
                            "[bf_i]"    | "[BF_I]"             => Atom::BoardFootInternational,
                            "[k]"       | "[K]"      | "𝑘"     => Atom::BoltzmannConstant,
                            "[bu_br]"   | "[BU_BR]"            => Atom::BushelBR,
                            "[bu_us]"   | "[BU_US]"            => Atom::BushelUS,
                            "[cml_i]"   | "[CML_I]"            => Atom::CircularMilInternational,
                            "[cr_i]"    | "[CR_I]"             => Atom::CordInternational,
                            "[crd_us]"  | "[CRD_US]"           => Atom::CordUS,
                            "[cft_i]"   | "[CFT_I]"            => Atom::CubicFootInternational,
                            "[cin_i]"   | "[CIN_I]"            => Atom::CubicInchInternational,
                            "[cyd_i]"   | "[CYD_I]"            => Atom::CubicYardInternational,
                            "[cup_us]"  | "[CUP_US]"           => Atom::CupUS,
                            "Ci"        | "CI"                 => Atom::Curie,

            "day"         | "d"         | "D"                  => Atom::Day,
            "Cel"         | "CEL"       | "°C"                 => Atom::DegreeCelsius,
                            "[degF]"    | "[DEGF]"   | "°F"    => Atom::DegreeFahrenheit,
                            "'"                                => Atom::DegreeMinute,
                            "''"                               => Atom::DegreeSecond,
                            "[degRe]"   | "°Ré"                => Atom::DegreeReaumur,
            "degree"      | "deg"       | "DEG"      | "°"     => Atom::Degree,
                            "[dr_av]"   | "[DR_AV]"            => Atom::DramAV,
                            "[dpt_us]"  | "[DPT_US]"           => Atom::DryPintUS,
                            "[dqt_us]"  | "[DQT_US]"           => Atom::DryQuartUS,
                            "dyn"       | "DYN"                => Atom::Dyne,
                            "eV"        | "EV"                 => Atom::ElectronVolt,
                            "[m_e]"     | "[M_E]"              => Atom::ElectronMass,
                            "[e]"       | "[E]"      | "𝑒"     => Atom::ElementaryCharge,
                            "eq"        | "EQ"                 => Atom::Equivalents,
                            "F"                                => Atom::Farad,
                            "[fth_br]"  | "[FTH_BR]"           => Atom::FathomBR,
                            "[fth_i]"   | "[FTH_I]"            => Atom::FathomInternational,
                            "[fth_us]"  | "[FTH_US]"           => Atom::FathomUS,
                            "[fdr_br]"  | "[FDR_BR]"           => Atom::FluidDramBR,
                            "[fdr_us]"  | "[FDR_US]"           => Atom::FluidDramUS,
                            "[foz_br]"  | "[FOZ_BR]"           => Atom::FluidOunceBR,
                            "[foz_us]"  | "[FOZ_US]"           => Atom::FluidOunceUS,
                            "[ft_br]"   | "[FT_BR]"            => Atom::FootBR,
            "foot"        | "[ft_i]"    | "[FT_I]"   | "ft"    => Atom::FootInternational,
                            "[ft_us]"   | "[FT_US]"            => Atom::FootUS,
                            "[fur_us]"  | "[FUR_US]"           => Atom::FurlongUS,

                            "Gal"       | "GL"                 => Atom::Gal,
                            "[gal_br]"  | "[GAL_BR]"           => Atom::GallonBR,
                            "G"         | "GS"                 => Atom::Gauss,
                            "Gb"        | "GB"                 => Atom::Gilbert,
                            "[gil_br]"  | "[GIL_BR]"           => Atom::GillBR,
                            "[gil_us]"  | "[GIL_US]"           => Atom::GillUS,
                            "gon"       | "GON"      | "grade" => Atom::Gon,
                            "gf"        | "GF"                 => Atom::GramForce,
                            "g%"        | "G%"                 => Atom::GramPercent,
                            "[gr]"      | "[GR]"               => Atom::Grain,
                            "Gy"        | "GY"                 => Atom::Gray,
                            "[ch_br]"   | "[CH_BR]"            => Atom::GuntersChainBR,
                            "[ch_us]"   | "[CH_US]"            => Atom::GuntersChainUS,
                            "[hd_i]"    | "[HD_I]"             => Atom::HandInternational,
                            "Hz"        | "HZ"                 => Atom::Hertz,
                            "H"                                => Atom::Henry,
                            "[gal_wi]"  | "[GAL_WI]"           => Atom::HistoricalWinchesterGallon,
                            "[HP]"                             => Atom::Horsepower,
            "hour"        | "h"         | "HR"                 => Atom::Hour,
                            "[in_br]"   | "[IN_BR]"            => Atom::InchBR,
            "inch"        | "[in_i]"    | "[IN_I]"   | "in"    => Atom::InchInternational,
                            "[in_us]"   | "[IN_US]"            => Atom::InchUS,

                            "J"                                => Atom::Joule,
                            "Ky"        | "KY"                 => Atom::Kayser,
                            "[kn_br]"   | "[KN_BR]"            => Atom::KnotBR,
                            "[kn_i]"    | "[KN_I]"             => Atom::KnotInternational,
                            "Lmb"       | "LMB"                => Atom::Lambert,
                            "[lcwt_av]" | "[LCWT_AV]"          => Atom::LongHundredweightAV,
                            "[lton_av]" | "[LTON_AV]"          => Atom::LongTonAV,
                            "[ly]"      | "[LY]"     | "l.y."  => Atom::LightYear,
                            "[lk_br]"   | "[LK_BR]"            => Atom::LinkForGuntersChainBR,
                            "[lk_us]"   | "[LK_US]"            => Atom::LinkForGuntersChainUS,
                            "[rlk_us]"  | "[RLK_US]"           => Atom::LinkForRamdensChainUS,
            "liter"       | "l"         | "L"                  => Atom::Liter,
                            "lm"        | "LM"                 => Atom::Lumen,
                            "lx"        | "LX"                 => Atom::Lux,

                            "Mx"        | "MX"                 => Atom::Maxwell,
                            "mo_g"      | "MO_G"               => Atom::MeanGregorianMonth,
                            "a_g"       | "ANN_G"              => Atom::MeanGregorianYear,
                            "mo_j"      | "MO_J"     | "moⱼ"   => Atom::MeanJulianMonth,
                            "a_j"       | "ANN_J"    | "aⱼ"    => Atom::MeanJulianYear,
                            "[cup_m]"   | "[CUP_M]"            => Atom::MetricCup,
                            "[foz_m]"   | "[FOZ_M]"            => Atom::MetricFluidOunce,
                            "[tbs_m]"   | "[TBS_M]"            => Atom::MetricTablespoon,
                            "[tsp_m]"   | "[TSP_M]"            => Atom::MetricTeaspoon,
                            "[mil_i]"   | "[MIL_I]"            => Atom::MilInternational,
                            "[mil_us]"  | "[MIL_US]"           => Atom::MilUS,
                            "[mi_br]"   | "[MI_BR]"            => Atom::MileBR,
            "mile"        | "[mi_i]"    | "[MI_I]"             => Atom::MileInternational,
                            "[mi_us]"   | "[MI_US]"            => Atom::MileUS,
                            "[min_br]"  | "[MIN_BR]"           => Atom::MinimBR,
                            "[min_us]"  | "[MIN_US]"           => Atom::MinimUS,
                            "mol"       | "MOL"                => Atom::Mole,
            "month"       | "mo"        | "MO"                 => Atom::Month,
                            "[nmi_br]"  | "[NMI_BR]"           => Atom::NauticalMileBR,
                            "[nmi_i]"   | "[NMI_I]"            => Atom::NauticalMileInternational,
                            "N"                                => Atom::Newton,
                            "Ohm"       | "OHM"                => Atom::Ohm,
                            "Oe"        | "OE"                 => Atom::Oersted,
                            "[oz_av]"   | "[OZ_AV]"            => Atom::OunceAV,
                            "[oz_tr]"   | "[OZ_TR]"            => Atom::OunceTR,

                            "pc"        | "PRS"                => Atom::Parsec,
                            "Pa"        | "PAL"                => Atom::Pascal,
                            "%"                                => Atom::Percent,
                            "[mu_0]"    | "[MU_0]"    | "μ₀"   => Atom::PermeabilityOfVacuum,
                            "[eps_0]"   | "[EPS_0]"   | "ε₀"   => Atom::PermittivityOfVacuum,
            "pH"          | "[pH]"      | "[PH]"               => Atom::PH,
                            "ph"        | "PHT"                => Atom::Phot,
                            "[pc_br]"   | "[PC_BR]"            => Atom::PaceBR,
                            "[ppb]"     | "[PPB]"     | "ppb"  => Atom::PartsPerBillion,
                            "[ppm]"     | "[PPM]"     | "ppm"  => Atom::PartsPerMillion,
                            "[ppth]"    | "[PPTH]"    | "ppt"  => Atom::PartsPerThousand,
                            "[pk_br]"   | "[PK_BR]"            => Atom::PeckBR,
                            "[pk_us]"   | "[PK_US]"            => Atom::PeckUS,
                            "[pwt_tr]"  | "[PWT_TR]"           => Atom::PennyweightTR,
                            "[pt_br]"   | "[PT_BR]"            => Atom::PintBR,
            "pint"        | "[pt_us]"   | "[PT_US]"            => Atom::PintUS,
                            "[h]"       | "[H]"                => Atom::PlanckConstant,
                            "[lb_av]"   | "[LB_AV]"            => Atom::PoundAV,
                            "[lb_tr]"   | "[LB_TR]"            => Atom::PoundTR,
                            "[lbf_av]"  | "[LBF_AV]"  | "lbf"  => Atom::PoundForce,
                            "P"                                => Atom::Poise,
                            "[p'diop]"  | "[P'DIOP]"  | "PD"   => Atom::PrismDiopter,
                            "[PNU]"                            => Atom::ProteinNitrogenUnit,
                            "[m_p]"     | "[M_P]"     | "𝑚ₚ"   => Atom::ProtonMass,
                            "[qt_br]"   | "[QT_BR]"            => Atom::QuartBR,
            "quart"       | "[qt_us]"   | "[QT_US]"            => Atom::QuartUS,
                            "[gal_us]"  | "[GAL_US]"           => Atom::QueenAnnesWineGallon,
                            "[rch_us]"  | "[RCH_US]"           => Atom::RamdensChainUS,
                            "RAD"       | "[RAD]"              => Atom::RadiationAbsorbedDose,
                            "REM"       | "[REM]"              => Atom::RadiationEquivalentMan,
                            "[rd_br]"   | "[RD_BR]"            => Atom::RodBR,
                            "[rd_us]"   | "[RD_US]"            => Atom::RodUS,
                            "R"         | "ROE"                => Atom::Roentgen,

                            "[sct]"      | "[SCT]"              => Atom::Section,
                            "[scwt_av]"  | "[SCWT_AV]"          => Atom::ShortHundredweightAV,
                            "[ston_av]"  | "[STON_AV]"          => Atom::ShortTonAV,
                            "S"          | "SIE"                => Atom::Siemens,
                            "Sv"         | "SV"                 => Atom::Sievert,
                            "[sft_i]"    | "[SFT_I]"            => Atom::SquareFootInternational,
                            "[sin_i]"    | "[SIN_I]"            => Atom::SquareInchInternational,
                            "[smi_us]"   | "[SMI_US]"           => Atom::SquareMileUS,
                            "[srd_us]"   | "[SRD_US]"           => Atom::SquareRodUS,
                            "[syd_i]"    | "[SYD_I]"            => Atom::SquareYardInternational,
                            "[g]"        | "𝑔"                  => Atom::StandardAccelerationOfFreeFall,
                            "atm"        | "ATM"                => Atom::StandardAtmosphere,
                            "sr"         | "SR"                 => Atom::Steradian,
                            "sb"         | "SB"                 => Atom::Stilb,
                            "[stone_av]" | "[STONE_AV]"         => Atom::StoneAV,
                            "mo_s"       | "MO_S"       | "moₛ" => Atom::SynodalMonth,
                            "[tbs_us]"   | "[TBS_US]"           => Atom::TablespoonUS,
                            "[tsp_us]"   | "[TSP_US]"           => Atom::TeaspoonUS,
                            "T"                                 => Atom::Tesla,
                            "[pi]"      | "[PI]"      | "π"     => Atom::TheNumberPi,
                            "10*"                               => Atom::TheNumberTenForArbitraryPowersStar,
                            "10^"                               => Atom::TheNumberTenForArbitraryPowersCaret,
            "tonne"       | "t"         | "TNE"                 => Atom::Tonne,
                            "[twp]"     | "[TWP]"               => Atom::Township,
                            "a_t"       | "ANN_T"     | "aₜ"    => Atom::TropicalYear,

                            "[c]"       | "[C]"       | "𝑐"    => Atom::VelocityOfLight,
                            "V"                                => Atom::Volt,
                            "W"                                => Atom::Watt,
                            "Wb"        | "WB"                 => Atom::Weber,
            "week"        | "wk"        | "WK"                 => Atom::Week,
            "yard"        | "[yd_i]"    | "[YD_I]"             => Atom::YardInternational,
                            "[yd_br]"   | "[YD_BR]"            => Atom::YardBR,
                            "[yd_us]"   | "[YD_US]"            => Atom::YardUS,
            "year"        | "a"         | "ANN"                => Atom::Year,

            _ => return Err(Error::UnknownUnitString(pair_str.as_str().to_string())),
        };

        Ok(atom)
    }

    fn visit_prefix_symbol(&mut self, pair: Pair<Rule>) -> Result<Prefix, Error> {
        let prefix_str = pair.into_span();

        let prefix = match prefix_str.as_str() {
            "atto"  | "a"   | "A"         => Prefix::Atto,
            "centi" | "c"   | "C"         => Prefix::Centi,
            "deci"  | "d"   | "D"         => Prefix::Deci,
            "deka"  | "da"  | "DA"        => Prefix::Deka,
            "exa"   | "E"   | "EX"        => Prefix::Exa,
            "femto" | "f"   | "F"         => Prefix::Femto,
            "gibi"  | "Gi"  | "GIB"       => Prefix::Gibi,
            "giga"  | "G"   | "GA"        => Prefix::Giga,
            "hecto" | "h"   | "H"         => Prefix::Hecto,
            "kilo"  | "k"   | "K"         => Prefix::Kilo,
            "mebi"  | "Mi"  | "MIB"       => Prefix::Mebi,
            "mega"  | "M"   | "MA"        => Prefix::Mega,
            "micro" | "u"   | "U"   | "µ" => Prefix::Micro,
            "milli" | "m"                 => Prefix::Milli,
            "nano"  | "n"   | "N"         => Prefix::Nano,
            "peta"  | "P"   | "PT"        => Prefix::Peta,
            "tebi"  | "Ti"  | "TIB"       => Prefix::Tebi,
            "tera"  | "T"   | "TR"        => Prefix::Tera,
            "yocto" | "y"   | "YO"        => Prefix::Yocto,
            "yotta" | "Y"   | "YA"        => Prefix::Yotta,
            "zepto" | "z"   | "ZO"        => Prefix::Zepto,
            "zetta" | "Z"   | "ZA"        => Prefix::Zetta,
            _                             => return Err(Error::UnknownUnitString(prefix_str.as_str().to_string()))
        };

        Ok(prefix)
    }

    fn visit_digits(&mut self, pair: Pair<Rule>) -> Result<i32, Error> {
        let span = pair.into_span();
        let string = span.as_str();

        string.parse::<i32>().map_err(|e| Error::ParsingError {
            expression: e.to_string(),
        })
    }

    fn visit_exponent(&mut self, pair: Pair<Rule>) -> Result<i32, Error> {
        let mut e = 1;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::sign => {
                    let span = inner_pair.into_span();
                    let string = span.as_str();

                    match string {
                        "+" => {}
                        "-" => {
                            e = -e;
                        }
                        _ => unreachable!(),
                    }
                }
                Rule::digits => {
                    e *= self.visit_digits(inner_pair)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(e)
    }

    fn visit_simple_unit(&mut self, pair: Pair<Rule>) -> Result<SimpleUnit, Error> {
        let mut simple_unit = SimpleUnit::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::prefix_symbol => {
                    let prefix = self.visit_prefix_symbol(inner_pair)?;

                    simple_unit.prefix = Some(prefix);
                }
                Rule::atom_symbol => {
                    let atom = self.visit_atom_symbol(inner_pair)?;

                    simple_unit.atom = Some(atom);
                }
                _ => unreachable!(),
            }
        }

        Ok(simple_unit)
    }

    // TODO
    // fn visit_special_unit(&mut self, su: &SpecialUnit) -> Term {
    //     // Term::new(None, None)
    //     unimplemented!()
    // }

    fn visit_annotatable(&mut self, pair: Pair<Rule>) -> Result<Annotatable, Error> {
        let mut annotatable = Annotatable::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::simple_unit => {
                    let simple_unit = self.visit_simple_unit(inner_pair)?;

                    annotatable.atom = simple_unit.atom;
                    annotatable.prefix = simple_unit.prefix;
                }
                Rule::exponent => {
                    annotatable.exponent = self.visit_exponent(inner_pair)?;
                }
                // Rule::special_unit => {}
                _ => unreachable!(),
            }
        }

        Ok(annotatable)
    }

    fn visit_annotation(&mut self, pair: Pair<Rule>) -> Result<String, Error> {
        let string = pair.into_span().as_str().to_string();

        Ok(string)
    }

    fn visit_factor(&mut self, pair: Pair<Rule>) -> Result<u32, Error> {
        let span = pair.into_span();
        let string = span.as_str();

        string.parse::<u32>().map_err(|e| Error::ParsingError {
            expression: e.to_string(),
        })
    }

    fn visit_basic_component(&mut self, pair: Pair<Rule>) -> Result<BasicComponent, Error> {
        let mut bc = BasicComponent::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::annotatable => {
                    let annotatable = self.visit_annotatable(inner_pair)?;

                    bc.prefix = annotatable.prefix;
                    bc.atom = annotatable.atom;
                    bc.exponent = annotatable.exponent;
                }
                Rule::annotation => {
                    let annotation = self.visit_annotation(inner_pair)?;

                    bc.annotation = Some(annotation);
                }
                Rule::factor => {
                    bc.factor = self.visit_factor(inner_pair)?;
                }
                Rule::term => {
                    let ast_term = self.visit_term(inner_pair)?;

                    bc.terms.append(&mut ast_term.into());
                }
                _ => unreachable!(),
            }
        }

        Ok(bc)
    }

    fn visit_component(&mut self, pair: Pair<Rule>) -> Result<Component, Error> {
        let mut component = Component::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::factor => {
                    component.factor = Some(self.visit_factor(inner_pair)?);
                }
                Rule::basic_component => {
                    let bc = self.visit_basic_component(inner_pair)?;

                    component.terms.append(&mut bc.into());
                }
                _ => unreachable!(),
            }
        }

        Ok(component)
    }

    fn visit_term(&mut self, pair: Pair<Rule>) -> Result<AstTerm, Error> {
        let mut has_slash = false;
        let mut ast_term = AstTerm::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::term => {
                    let child_ast_term = self.visit_term(inner_pair)?;
                    let mut new_terms: Vec<Term> = child_ast_term.into();

                    if has_slash {
                        for new_term in &mut new_terms {
                            new_term.exponent = -new_term.exponent;
                        }
                        has_slash = false;
                    }

                    ast_term.terms.append(&mut new_terms);
                }
                Rule::slash => {
                    has_slash = true;
                }
                Rule::component => {
                    let component = self.visit_component(inner_pair)?;

                    ast_term.component = Some(component);
                }
                _ => {
                    println!("visit_term: unreachable rule: {:?}", inner_pair);
                    unreachable!()
                }
            }
        }

        Ok(ast_term)
    }

    fn visit_main_term(&mut self, pair: Pair<Rule>) -> Result<MainTerm, Error> {
        let mut main_term = MainTerm::new();
        let mut has_slash = false;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::slash => {
                    has_slash = true;
                }
                Rule::term => {
                    let mut ast_term = self.visit_term(inner_pair)?;
                    let mut new_terms: Vec<Term> = ast_term.into();

                    if has_slash {
                        for new_term in &mut new_terms {
                            new_term.exponent = -new_term.exponent;
                        }
                        has_slash = false;
                    }

                    main_term.terms.append(&mut new_terms);
                }
                _ => unreachable!(),
            }
        }

        Ok(main_term)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atom::Atom;
    use pest::Parser;
    use unit::Unit;
    use unit_parser::{Rule, UnitParser};

    #[test]
    fn validate_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m-3").unwrap();
        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = -3;

        let expected = Unit {
            terms: vec![expected_term],
        };

        assert_eq!(actual, expected);

        let pairs = UnitParser::parse(Rule::main_term, "km2/m-3").unwrap();
        let actual = i.interpret(pairs).unwrap();

        let mut term1 = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        term1.exponent = 2;

        let mut term2 = Term::new(Some(Atom::Meter), None);
        term2.exponent = 3;

        let expected = Unit {
            terms: vec![term1, term2],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_component_with_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.exponent = 2;

        let expected = Unit {
            terms: vec![expected_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_component_with_prefix() {
        let pairs = UnitParser::parse(Rule::main_term, "km").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();

        let expected = Unit {
            terms: vec![Term::new(Some(Atom::Meter), Some(Prefix::Kilo))],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_component_with_factor() {
        let pairs = UnitParser::parse(Rule::main_term, "2m").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();

        let mut expected_term = Term::new(Some(Atom::Meter), None);
        expected_term.factor = 2;

        let expected = Unit {
            terms: vec![expected_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term() {
        let pairs = UnitParser::parse(Rule::main_term, "m/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_prefix() {
        let pairs = UnitParser::parse(Rule::main_term, "km/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let expected_numerator_term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
        let mut expected_denominator_term = Term::new(Some(Atom::Second), None);
        expected_denominator_term.exponent = -1;

        let expected = Unit {
            terms: vec![expected_numerator_term, expected_denominator_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_prefix() {
        let pairs = UnitParser::parse(Rule::main_term, "m/ks").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let expected_numerator_term = Term::new(Some(Atom::Meter), None);
        let mut expected_denominator_term = Term::new(Some(Atom::Second), Some(Prefix::Kilo));
        expected_denominator_term.exponent = -1;

        let expected = Unit {
            terms: vec![expected_numerator_term, expected_denominator_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m2/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_denominator_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m/s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_numerator_and_denominator_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m2/s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_numerator() {
        let pairs = UnitParser::parse(Rule::main_term, "2m/s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_denominator() {
        let pairs = UnitParser::parse(Rule::main_term, "m/2s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = -1;
        second_term.factor = 2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term() {
        let pairs = UnitParser::parse(Rule::main_term, "m.s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_side_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m2.s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_right_side_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m.s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_left_and_right_side_exponent() {
        let pairs = UnitParser::parse(Rule::main_term, "m2.s2").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.exponent = 2;
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.exponent = 2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_dot_term_with_factor_in_left_side() {
        let pairs = UnitParser::parse(Rule::main_term, "2m.s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let mut meter_term = Term::new(Some(Atom::Meter), None);
        meter_term.factor = 2;
        let second_term = Term::new(Some(Atom::Second), None);

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_slash_term_with_factor_in_right_side() {
        let pairs = UnitParser::parse(Rule::main_term, "m.2s").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let meter_term = Term::new(Some(Atom::Meter), None);
        let mut second_term = Term::new(Some(Atom::Second), None);
        second_term.factor = 2;

        let expected = Unit {
            terms: vec![meter_term, second_term],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_interpret_term_with_dot_term_then_slash_component() {
        let pairs = UnitParser::parse(Rule::main_term, "[acr_us].[in_i]/[acr_us]").unwrap();

        let mut i = Interpreter;
        let actual = i.interpret(pairs).unwrap();
        let acre_term = Term::new(Some(Atom::AcreUS), None);
        let inch_term = Term::new(Some(Atom::InchInternational), None);
        let mut acre2_term = Term::new(Some(Atom::AcreUS), None);
        acre2_term.exponent = -1;

        let expected = Unit {
            terms: vec![acre_term, inch_term, acre2_term],
        };

        assert_eq!(actual, expected);
    }
}
