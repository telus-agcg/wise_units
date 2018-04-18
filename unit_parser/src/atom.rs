use classification::Classification;
use definition::Definition;
// use dimension::Dimension;
use property::Property;
use std::f64::consts::PI;
use std::fmt;
use term::Term;
use ucum_symbol::UcumSymbol;

// TODO: Implement PartialEq
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Atom {
    TheUnity,

    Candela,
    Coulomb,
    Gram,
    Kelvin,
    Meter,
    Radian,
    Second,

    AcreBR,
    AcreUS,
    Ampere,
    Are,
    AstronomicUnit,
    AtomicMassUnit,
    Bar,
    BarrelUS,
    Becquerel,
    Biot,
    BoardFootInternational,
    BoltzmannConstant,
    BushelBR,
    BushelUS,
    CircularMilInternational,
    CordInternational,
    CordUS,
    CubicFootInternational,
    CubicInchInternational,
    CubicYardInternational,
    CupUS,
    Curie,

    Day,
    Degree,
    DegreeCelsius,
    DegreeFahrenheit,
    DegreeReaumur,
    DegreeMinute,
    DegreeSecond,
    DramAV,
    DryPintUS,
    DryQuartUS,
    Dyne,
    ElectronMass,
    ElectronVolt,
    ElementaryCharge,
    Equivalents,
    Erg,
    Farad,
    FathomInternational,
    FathomBR,
    FathomUS,
    FluidDramBR,
    FluidDramUS,
    FluidOunceBR,
    FluidOunceUS,
    FootInternational,
    FootBR,
    FootUS,
    FurlongUS,

    Gal,
    GallonBR,
    GuntersChainBR,
    GuntersChainUS,
    Gilbert,
    GillBR,
    GillUS,
    Gon,
    GramForce,
    GramPercent,
    Grain,
    Gray,
    Gauss,
    HandInternational,
    Hertz,
    Henry,
    HistoricalWinchesterGallon,
    Horsepower,
    Hour,
    InchInternational,
    InchBR,
    InchUS,

    Joule,
    Kayser,
    KnotBR,
    KnotInternational,
    Lambert,
    LongHundredweightAV,
    LongTonAV,
    LightYear,
    LinkForGuntersChainBR,
    LinkForGuntersChainUS,
    LinkForRamdensChainUS,
    Liter,
    Lumen,
    Lux,

    Maxwell,
    MeanGregorianMonth,
    MeanGregorianYear,
    MeanJulianMonth,
    MeanJulianYear,
    MetricCup,
    MetricFluidOunce,
    MetricTablespoon,
    MetricTeaspoon,
    MilInternational,
    MilUS,
    MileInternational,
    MileBR,
    MileUS,
    MinimBR,
    MinimUS,
    Minute,
    Mole,
    Month,
    NauticalMileBR,
    NauticalMileInternational,
    Newton,
    NewtonianConstantOfGravitation,
    Ohm,
    Oersted,
    OunceAV,
    OunceTR,

    PaceBR,
    Parsec,
    Pascal,
    PartsPerBillion,
    PartsPerMillion,
    PartsPerThousand,
    Percent,
    PermeabilityOfVacuum,
    PermittivityOfVacuum,
    PH,
    Phot,
    PeckBR,
    PeckUS,
    PennyweightTR,
    PintBR,
    PintUS,
    Poise,
    PoundAV,
    PoundTR,
    PoundForce,
    PlanckConstant,
    PrismDiopter,
    ProteinNitrogenUnit,
    ProtonMass,
    QuartBR,
    QuartUS,
    QueenAnnesWineGallon,
    RadiationAbsorbedDose,
    RadiationEquivalentMan,
    RamdensChainUS,
    RodBR,
    RodUS,
    Roentgen,

    Section,
    ShortHundredweightAV,
    ShortTonAV,
    Siemens,
    Sievert,
    SquareInchInternational,
    SquareFootInternational,
    SquareMileUS,
    SquareRodUS,
    SquareYardInternational,
    StandardAccelerationOfFreeFall,
    StandardAtmosphere,
    Steradian,
    Stilb,
    StoneAV,
    Stokes,
    SynodalMonth,
    TablespoonUS,
    TeaspoonUS,
    Tesla,
    TheNumberPi,
    TheNumberTenForArbitraryPowersCaret,
    TheNumberTenForArbitraryPowersStar,
    Tonne,
    Township,
    TropicalYear,

    VelocityOfLight,
    Volt,
    Watt,
    Weber,
    Week,
    YardInternational,
    YardBR,
    YardUS,
    Year,
}

impl UcumSymbol for Atom {
    fn classification(&self) -> Classification {
        match *self {
            Atom::PartsPerBillion                         |
                Atom::PartsPerMillion                     |
                Atom::PartsPerThousand                    |
                Atom::Percent                             |
                Atom::TheNumberPi                         |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar    => Classification::Dimless,
            Atom::TheUnity          |
                Atom::Candela       |
                Atom::Coulomb       |
                Atom::DegreeCelsius |
                Atom::Gram          |

                Atom::Ampere        |
                Atom::Becquerel     |
                Atom::Farad         |
                Atom::Gray          |
                Atom::Henry         |
                Atom::Hertz         |
                Atom::Joule         |
                Atom::Kelvin        |
                Atom::Lumen         |
                Atom::Lux           |
                Atom::Meter         |
                Atom::Mole          |
                Atom::Newton        |
                Atom::Ohm           |
                Atom::Pascal        |
                Atom::Radian        |
                Atom::Second        |
                Atom::Siemens       |
                Atom::Sievert       |
                Atom::Steradian     |
                Atom::Tesla         |
                Atom::Volt          |
                Atom::Watt          |
                Atom::Weber                                 => Classification::SI,
            Atom::Are                    |
                Atom::AstronomicUnit     |
                Atom::AtomicMassUnit     |
                Atom::Bar                |
                Atom::Day                |
                Atom::Degree             |
                Atom::DegreeMinute       |
                Atom::DegreeSecond       |
                Atom::ElectronMass       |
                Atom::ElectronVolt       |
                Atom::Gon                |
                Atom::Hour               |
                Atom::Liter              |
                Atom::MeanGregorianMonth |
                Atom::MeanGregorianYear  |
                Atom::MeanJulianMonth    |
                Atom::MeanJulianYear     |
                Atom::Minute             |
                Atom::Month              |
                Atom::Parsec             |
                Atom::SynodalMonth       |
                Atom::Tonne              |
                Atom::TropicalYear       |
                Atom::Week               |
                Atom::Year                                  => Classification::ISO1000,
            Atom::BoltzmannConstant                  |
                Atom::ElementaryCharge               |
                Atom::GramForce                      |
                Atom::LightYear                      |
                Atom::NewtonianConstantOfGravitation |
                Atom::PermeabilityOfVacuum           |
                Atom::PermittivityOfVacuum           |
                Atom::PlanckConstant                 |
                Atom::PoundForce                     |
                Atom::ProtonMass                     |
                Atom::StandardAccelerationOfFreeFall |
                Atom::StandardAtmosphere             |
                Atom::VelocityOfLight                       => Classification::Const,
            Atom::Biot                       |
                Atom::Curie                  |
                Atom::Dyne                   |
                Atom::Erg                    |
                Atom::Gal                    |
                Atom::Gauss                  |
                Atom::Gilbert                |
                Atom::Kayser                 |
                Atom::Lambert                |
                Atom::Maxwell                |
                Atom::Oersted                |
                Atom::Phot                   |
                Atom::Poise                  |
                Atom::RadiationAbsorbedDose  |
                Atom::RadiationEquivalentMan |
                Atom::Roentgen               |
                Atom::Stilb                  |
                Atom::Stokes                                => Classification::CGS,
            Atom::BoardFootInternational        |
                Atom::CircularMilInternational  |
                Atom::CordInternational         |
                Atom::CubicFootInternational    |
                Atom::CubicInchInternational    |
                Atom::CubicYardInternational    |
                Atom::HandInternational         |
                Atom::InchInternational         |
                Atom::FathomInternational       |
                Atom::FootInternational         |
                Atom::KnotInternational         |
                Atom::MilInternational          |
                Atom::MileInternational         |
                Atom::NauticalMileInternational |
                Atom::SquareFootInternational   |
                Atom::SquareInchInternational   |
                Atom::SquareYardInternational   |
                Atom::YardInternational                     => Classification::Intcust,
            Atom::AcreUS                    |
                Atom::FathomUS              |
                Atom::FootUS                |
                Atom::FurlongUS             |
                Atom::GuntersChainUS        |
                Atom::InchUS                |
                Atom::LinkForGuntersChainUS |
                Atom::LinkForRamdensChainUS |
                Atom::MilUS                 |
                Atom::MileUS                |
                Atom::RamdensChainUS        |
                Atom::RodUS                 |
                Atom::Section               |
                Atom::SquareMileUS          |
                Atom::SquareRodUS           |
                Atom::Township              |
                Atom::YardUS                                => Classification::USLengths,
            Atom::AcreBR                    |
                Atom::FathomBR              |
                Atom::FootBR                |
                Atom::GuntersChainBR        |
                Atom::InchBR                |
                Atom::KnotBR                |
                Atom::LinkForGuntersChainBR |
                Atom::MileBR                |
                Atom::NauticalMileBR        |
                Atom::PaceBR                |
                Atom::RodBR                 |
                Atom::YardBR                                => Classification::BritLength,
            Atom::BarrelUS                       |
                Atom::BushelUS                   |
                Atom::CordUS                     |
                Atom::CupUS                      |
                Atom::DryPintUS                  |
                Atom::DryQuartUS                 |
                Atom::FluidDramUS                |
                Atom::FluidOunceUS               |
                Atom::GillUS                     |
                Atom::HistoricalWinchesterGallon |
                Atom::MetricCup                  |
                Atom::MetricFluidOunce           |
                Atom::MetricTablespoon           |
                Atom::MetricTeaspoon             |
                Atom::MinimUS                    |
                Atom::PeckUS                     |
                Atom::PintUS                     |
                Atom::QuartUS                    |
                Atom::QueenAnnesWineGallon       |
                Atom::TablespoonUS               |
                Atom::TeaspoonUS                            => Classification::USVolumes,
                Atom::BushelBR                   |
                Atom::FluidDramBR                |
                Atom::FluidOunceBR               |
                Atom::GallonBR                   |
                Atom::GillBR                     |
                Atom::MinimBR                    |
                Atom::PeckBR                     |
                Atom::PintBR                     |
                Atom::QuartBR                               => Classification::BritVolumes,
            Atom::DramAV                         |
                Atom::Grain                      |
                Atom::OunceAV                    |
                Atom::LongHundredweightAV        |
                Atom::LongTonAV                  |
                Atom::PoundAV                    |
                Atom::ShortHundredweightAV       |
                Atom::ShortTonAV                 |
                Atom::StoneAV                               => Classification::Avoirdupois,
            Atom::OunceTR                        |
                Atom::PennyweightTR              |
                Atom::PoundTR                               => Classification::Troy,
            Atom::DegreeFahrenheit               |
                Atom::DegreeReaumur              |
                Atom::Horsepower                            => Classification::Heat,
            Atom::PrismDiopter                              => Classification::Clinical,
            Atom::Equivalents                    |
                Atom::GramPercent                |
                Atom::PH                         |
                Atom::ProteinNitrogenUnit                   => Classification::Chemical,
        }
    }

    fn definition(&self) -> Definition {
        let result = match *self {
            Atom::TheUnity                |
                Atom::Candela             |
                Atom::Coulomb             |
                Atom::Gram                |
                Atom::Kelvin              |
                Atom::Meter               |
                Atom::ProteinNitrogenUnit |
                Atom::Radian              |
                Atom::Second                     => {
                    // Manually build the Definition here for speed.
                    let term = Term::new(Some(Atom::TheUnity), None);

                    Ok(Definition { value: 1.0, terms: vec![term] })
                },
            Atom::AcreBR                         => Definition::new(4840.0, "[yd_br]2"),
            Atom::AcreUS                         => Definition::new(160.0, "[rd_us]2"),
            Atom::Ampere                         => Definition::new(1.0, "C/s"),
            Atom::Are                            => Definition::new(100.0, "m2"),
            Atom::AstronomicUnit                 => Definition::new(149_597.870_691, "Mm"),
            Atom::AtomicMassUnit                 => Definition::new(1.660_540_2e-24, "g"),
            Atom::Bar                            => Definition::new(1.0e5, "Pa"),
            Atom::BarrelUS                       => Definition::new(42.0, "[gal_us]"),
            Atom::Becquerel |
                Atom::Hertz                      => Definition::new(1.0, "s-1"),
            Atom::Biot                           => Definition::new(10.0, "A"),
            Atom::BoardFootInternational         => Definition::new(144.0, "[in_i]3"),
            Atom::BoltzmannConstant              => Definition::new(1.380_658e-23, "J/K"),
            Atom::BushelBR                       => Definition::new(4.0, "[pk_br]"),
            Atom::BushelUS                       => Definition::new(2150.42, "[in_i]3"),
            Atom::CircularMilInternational       => Definition::new(1.0, "[pi]/4.[mil_i]2"),
            Atom::CordInternational |
                Atom::CordUS                     => Definition::new(128.0, "[ft_i]3"),
            Atom::CubicFootInternational         => Definition::new(1.0, "[ft_i]3"),
            Atom::CubicInchInternational         => Definition::new(1.0, "[in_i]3"),
            Atom::CubicYardInternational         => Definition::new(1.0, "[yd_i]3"),
            Atom::CupUS                          => Definition::new(16.0, "[tbs_us]"),
            Atom::Curie                          => Definition::new(37e9, "Bq"),

            Atom::Day                            => Definition::new(24.0, "h"),
            Atom::Degree                         => Definition::new(2.0, "[pi].rad/360"),
            Atom::DegreeCelsius                  => Definition::new(1.0, "cel(1.0 K)"),
            Atom::DegreeFahrenheit               => Definition::new(1.0, "degf(5.0 K/9)"),
            Atom::DegreeMinute                   => Definition::new(1.0, "deg/60"),
            Atom::DegreeReaumur                  => Definition::new(1.0, "degre(5.0 K/4)"),
            Atom::DegreeSecond                   => Definition::new(1.0, "/60"),
            Atom::DramAV                         => Definition::new(1.0, "[oz_av]/16"),
            Atom::DryPintUS                      => Definition::new(1.0, "[dqt_us]/2"),
            Atom::DryQuartUS                     => Definition::new(1.0, "[pk_us]/8"),
            Atom::Dyne                           => Definition::new(1.0, "g.cm/s2"),
            Atom::ElectronMass                   => Definition::new(9.109_389_7e-28, "g"),
            Atom::ElectronVolt                   => Definition::new(1.0, "[e].V"),
            Atom::ElementaryCharge               => Definition::new(1.602_177_33e-19, "C"),
            Atom::Equivalents                    => Definition::new(1.0, "mol"),
            Atom::Erg                            => Definition::new(1.0, "dyn.cm"),
            Atom::Farad                          => Definition::new(1.0, "C/V"),
            Atom::FathomInternational            => Definition::new(6.0, "[ft_i]"),
            Atom::FathomBR                       => Definition::new(6.0, "[ft_br]"),
            Atom::FathomUS                       => Definition::new(6.0, "[ft_us]"),
            Atom::FluidDramBR                    => Definition::new(1.0, "[foz_br]/8"),
            Atom::FluidDramUS                    => Definition::new(1.0, "[foz_us]/8"),
            Atom::FluidOunceBR                   => Definition::new(1.0, "[gil_br]/5"),
            Atom::FluidOunceUS                   => Definition::new(1.0, "[gil_us]/4"),
            Atom::FootInternational              => Definition::new(12.0, "[in_i]"),
            Atom::FootBR                         => Definition::new(12.0, "[in_br]"),
            Atom::FootUS                         => Definition::new(1200.0, "m/3937"),
            Atom::FurlongUS                      => Definition::new(40.0, "[rd_us]"),

            Atom::Gal                            => Definition::new(1.0, "cm/s2"),
            Atom::GallonBR                       => Definition::new(4.546_09, "l"),
            Atom::Gauss                          => Definition::new(1.0e-4, "T"),
            Atom::Gilbert                        => Definition::new(1.0, "Oe.cm"),
            Atom::GillBR                         => Definition::new(1.0, "[pt_br]/4"),
            Atom::GillUS                         => Definition::new(1.0, "[pt_us]/4"),
            Atom::Gon                            => Definition::new(0.9, "deg"),
            Atom::GramForce                      => Definition::new(1.0, "g.[g]"),
            Atom::GramPercent                    => Definition::new(1.0, "g/dl"),
            Atom::Grain                          => Definition::new(64.798_91, "mg"),
            Atom::Gray |
                Atom::Sievert                    => Definition::new(1.0, "J/kg"),
            Atom::GuntersChainBR                 => Definition::new(4.0, "[rd_br]"),
            Atom::GuntersChainUS                 => Definition::new(4.0, "[rd_us]"),
            Atom::HandInternational              => Definition::new(4.0, "[in_i]"),
            Atom::Henry                          => Definition::new(1.0, "Wb/A"),
            Atom::HistoricalWinchesterGallon     => Definition::new(1.0, "[bu_us]"),
            Atom::Hour                           => Definition::new(60.0, "min"),
            Atom::Horsepower                     => Definition::new(550.0, "[ft_i].[lbf_av]/s"),
            Atom::InchInternational              => Definition::new(254.0e-2, "cm"),
            Atom::InchBR                         => Definition::new(2.539_998, "cm"),
            Atom::InchUS                         => Definition::new(1.0, "[ft_us]/12"),

            Atom::Joule                          => Definition::new(1.0, "N.m"),
            Atom::Kayser                         => Definition::new(1.0, "cm-1"),
            Atom::KnotBR                         => Definition::new(1.0, "[nmi_br]/h"),
            Atom::KnotInternational              => Definition::new(1.0, "[nmi_i]/h"),
            Atom::Lambert                        => Definition::new(1.0, "cd/cm2/[pi]"),
            Atom::LongHundredweightAV            => Definition::new(112.0, "[lb_av]"),
            Atom::LongTonAV                      => Definition::new(20.0, "[lcwt_av]"),
            Atom::LightYear                      => Definition::new(1.0, "[c].a_j"),
            Atom::LinkForGuntersChainBR          => Definition::new(1.0, "[ch_br]/100"),
            Atom::LinkForGuntersChainUS          => Definition::new(1.0, "[ch_us]/100"),
            Atom::LinkForRamdensChainUS          => Definition::new(1.0, "[rch_us]/100"),
            Atom::Liter                          => Definition::new(1.0, "dm3"),
            Atom::Lumen                          => Definition::new(1.0, "cd.sr"),
            Atom::Lux                            => Definition::new(1.0, "lm/m2"),

            Atom::Maxwell                        => Definition::new(1.0e-8, "Wb"),
            Atom::MeanGregorianMonth             => Definition::new(1.0, "a_g/12"),
            Atom::MeanGregorianYear              => Definition::new(365.2425, "d"),
            Atom::MeanJulianMonth                => Definition::new(1.0, "a_j/12"),
            Atom::MeanJulianYear                 => Definition::new(365.25, "d"),
            Atom::MetricCup                      => Definition::new(240.0, "mL"),
            Atom::MetricFluidOunce               => Definition::new(30.0, "mL"),
            Atom::MetricTablespoon               => Definition::new(15.0, "mL"),
            Atom::MetricTeaspoon                 => Definition::new(5.0, "mL"),
            Atom::MilInternational               => Definition::new(1.0e-3, "[in_i]"),
            Atom::MilUS                          => Definition::new(1.0e-3, "[in_us]"),
            Atom::MileInternational              => Definition::new(5280.0, "[ft_i]"),
            Atom::MileBR                         => Definition::new(5280.0, "[ft_br]"),
            Atom::MileUS                         => Definition::new(8.0, "[fur_us]"),
            Atom::MinimBR                        => Definition::new(1.0, "[fdr_br]/60"),
            Atom::MinimUS                        => Definition::new(1.0, "[fdr_us]/60"),
            Atom::Minute                         => Definition::new(60.0, "s"),
            Atom::Mole                           => Definition::new(6.022_136_7, "10*23"),
            Atom::Month                          => Definition::new(1.0, "mo_j"),
            Atom::NauticalMileBR                 => Definition::new(6080.0, "[ft_br]"),
            Atom::NauticalMileInternational      => Definition::new(1852.0, "m"),
            Atom::Newton                         => Definition::new(1.0, "kg.m/s2"),
            Atom::NewtonianConstantOfGravitation => Definition::new(6.67259e-11, "m3.kg-1.s-2"),
            Atom::Ohm                            => Definition::new(1.0, "V/A"),
            Atom::Oersted                        => Definition::new(250.0, "/[pi].A/m"),
            Atom::OunceAV                        => Definition::new(1.0, "[lb_av]/16"),
            Atom::OunceTR                        => Definition::new(20.0, "[pwt_tr]"),

            Atom::PH                             => Definition::new(1.0, "ph(1.0 mol/l)"),
            Atom::PaceBR                         => Definition::new(2.5, "[ft_br]"),
            Atom::Parsec                         => Definition::new(3.085_678e16, "m"),
            Atom::PartsPerBillion                => Definition::new(1.0, "10*-9"),
            Atom::PartsPerMillion                => Definition::new(1.0, "10*-6"),
            Atom::PartsPerThousand               => Definition::new(1.0, "10*-3"),
            Atom::Pascal                         => Definition::new(1.0, "N/m2"),
            Atom::Percent                        => Definition::new(1.0, "10*-2"),
            Atom::PermeabilityOfVacuum           => Definition::new(1.0, "4.[PI].10*-7.N/A2"),
            Atom::PermittivityOfVacuum           => Definition::new(8.854_187_817e-12, "F/m"),
            Atom::Phot                           => Definition::new(1.0e-4, "lx"),
            Atom::PeckBR                         => Definition::new(2.0, "[gal_br]"),
            Atom::PeckUS                         => Definition::new(1.0, "[bu_us]/4"),
            Atom::PennyweightTR                  => Definition::new(24.0, "[gr]"),
            Atom::PintBR                         => Definition::new(1.0, "[qt_br]/2"),
            Atom::PintUS                         => Definition::new(1.0, "[qt_us]/2"),
            Atom::PlanckConstant                 => Definition::new(6.626_075_5e-34, "J.s"),
            Atom::Poise                          => Definition::new(1.0, "dyn.s/cm2"),
            Atom::PoundAV                        => Definition::new(7000.0, "[gr]"),
            Atom::PoundTR                        => Definition::new(12.0, "[oz_tr]"),
            Atom::PoundForce                     => Definition::new(1.0, "[lb_av].[g]"),
            Atom::PrismDiopter                   => Definition::new(1.0, "100tan(1.0 rad)"),
            Atom::ProtonMass                     => Definition::new(1.672_623_1e-24, "g"),
            Atom::QuartBR                        => Definition::new(1.0, "[gal_br]/4"),
            Atom::QuartUS                        => Definition::new(1.0, "[gal_us]/4"),
            Atom::QueenAnnesWineGallon           => Definition::new(231.0, "[in_i]3"),
            Atom::RadiationAbsorbedDose          => Definition::new(100.0, "erg/g"),
            Atom::RadiationEquivalentMan         => Definition::new(1.0, "RAD"),
            Atom::RamdensChainUS                 => Definition::new(100.0, "[ft_us]"),
            Atom::RodBR                          => Definition::new(16.5, "[ft_br]"),
            Atom::RodUS                          => Definition::new(16.5, "[ft_us]"),
            Atom::Roentgen                       => Definition::new(2.58e-4, "C/kg"),

            Atom::Section |
                Atom::SquareMileUS               => Definition::new(1.0, "[mi_us]2"),
            Atom::ShortHundredweightAV           => Definition::new(100.0, "[lb_av]"),
            Atom::ShortTonAV                     => Definition::new(20.0, "[scwt_at]"),
            Atom::Siemens                        => Definition::new(1.0, "Ohm-1"),
            Atom::SquareFootInternational        => Definition::new(1.0, "[ft_i]2"),
            Atom::SquareInchInternational        => Definition::new(1.0, "[in_i]2"),
            Atom::SquareRodUS                    => Definition::new(1.0, "[rd_us]2"),
            Atom::SquareYardInternational        => Definition::new(1.0, "[yd_i]2"),
            Atom::StandardAccelerationOfFreeFall => Definition::new(980_665e-5, "m/s2"),
            Atom::StandardAtmosphere             => Definition::new(101_325.0, "Pa"),
            Atom::Steradian                      => Definition::new(1.0, "rad2"),
            Atom::Stokes                         => Definition::new(1.0, "cm2/s"),
            Atom::Stilb                          => Definition::new(1.0, "cd/cm2"),
            Atom::StoneAV                        => Definition::new(14.0, "[lb_av]"),
            Atom::SynodalMonth                   => Definition::new(29.530_59, "d"),
            Atom::TablespoonUS                   => Definition::new(1.0, "[foz_us]/2"),
            Atom::TeaspoonUS                     => Definition::new(1.0, "[tbs_us]/3"),
            Atom::Tesla                          => Definition::new(1.0, "Wb/m2"),
            Atom::TheNumberPi                    => Definition::new(PI, "1"),
            Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => Definition::new(10.0, "1"),
            Atom::TropicalYear                           => Definition::new(365.242_19, "d"),
            Atom::Tonne                                  => Definition::new(1.0e3, "kg"),
            Atom::Township                               => Definition::new(36.0, "[sct]"),

            Atom::VelocityOfLight                        => Definition::new(299_792_458.0, "m/s"),
            Atom::Volt                                   => Definition::new(1.0, "J/C"),
            Atom::Watt                                   => Definition::new(1.0, "J/s"),
            Atom::Weber                                  => Definition::new(1.0, "V.s"),
            Atom::Week                                   => Definition::new(7.0, "d"),
            Atom::YardInternational                      => Definition::new(3.0, "[ft_i]"),
            Atom::YardBR                                 => Definition::new(3.0, "[ft_br]"),
            Atom::YardUS                                 => Definition::new(3.0, "[ft_us]"),
            Atom::Year                                   => Definition::new(1.0, "a_j"),
        };

        result.expect("BUG! Bad Atom -> Definition mapping!")
    }

    fn is_arbitrary(&self) -> bool {
        match *self {
            Atom::ProteinNitrogenUnit => true,
            _ => false,
        }
    }

    fn is_metric(&self) -> bool {
        match *self {
            Atom::TheUnity          |
                Atom::Candela       |
                Atom::Coulomb       |
                Atom::DegreeCelsius |
                Atom::Gram          |
                Atom::Kelvin        |
                Atom::Meter         |
                Atom::Radian        |
                Atom::Second        |

                Atom::Ampere                         |
                Atom::Are                            |
                Atom::AtomicMassUnit                 |
                Atom::Bar                            |
                Atom::Becquerel                      |
                Atom::Biot                           |
                Atom::BoltzmannConstant              |
                Atom::Curie                          |

                Atom::Dyne                           |
                Atom::ElectronMass                   |
                Atom::ElectronVolt                   |
                Atom::ElementaryCharge               |
                Atom::Equivalents                    |
                Atom::Erg                            |
                Atom::Farad                          |

                Atom::Gal                            |
                Atom::Gauss                          |
                Atom::Gilbert                        |
                Atom::GramForce                      |
                Atom::GramPercent                    |
                Atom::Gray                           |
                Atom::Henry                          |
                Atom::Hertz                          |

                Atom::Joule                          |
                Atom::Kayser                         |
                Atom::Lambert                        |
                Atom::LightYear                      |
                Atom::Liter                          |
                Atom::Lumen                          |
                Atom::Lux                            |

                Atom::Maxwell                        |
                Atom::Mole                           |
                Atom::Newton                         |
                Atom::NewtonianConstantOfGravitation |
                Atom::Ohm                            |
                Atom::Oersted                        |

                Atom::Parsec                         |
                Atom::Pascal                         |
                Atom::PermeabilityOfVacuum           |
                Atom::PermittivityOfVacuum           |
                Atom::Phot                           |
                Atom::PlanckConstant                 |
                Atom::Poise                          |
                Atom::ProtonMass                     |
                Atom::RadiationAbsorbedDose          |
                Atom::RadiationEquivalentMan         |
                Atom::Roentgen                       |

                Atom::StandardAccelerationOfFreeFall |
                Atom::Steradian                      |
                Atom::Stokes                         |
                Atom::Stilb                          |
                Atom::Siemens                        |
                Atom::Sievert                        |
                Atom::Tesla                          |
                Atom::Tonne                          |

                Atom::VelocityOfLight                |
                Atom::Volt                           |
                Atom::Watt                           |
                Atom::Weber => true,
            _ => false,
        }
    }

    fn is_special(&self) -> bool {
        match *self {
            Atom::DegreeCelsius        |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur    |
                Atom::PH               |
                Atom::PrismDiopter => true,
            _ => false,
        }
    }

    fn names(&self) -> Vec<&'static str> {
        match *self {
            Atom::TheUnity => vec!["the unity"],
            Atom::Candela  => vec!["candela"],
            Atom::Coulomb  => vec!["Coulomb"],
            Atom::Gram     => vec!["gram"],
            Atom::Kelvin   => vec!["Kelvin"],
            Atom::Meter    => vec!["meter"],
            Atom::Radian   => vec!["radian"],
            Atom::Second   |
               Atom::DegreeSecond => vec!["second"],

            // Derived units
            Atom::AcreBR |
                Atom::AcreUS               => vec!["acre"],
            Atom::Ampere                   => vec!["ampère"],
            Atom::Are                      => vec!["are"],
            Atom::AstronomicUnit           => vec!["astronomic unit"],
            Atom::AtomicMassUnit           => vec!["unified atomic mass unit"],
            Atom::Bar                      => vec!["bar"],
            Atom::BarrelUS                 => vec!["barrel"],
            Atom::Becquerel                => vec!["becquerel"],
            Atom::Biot                     => vec!["Biot"],
            Atom::BoardFootInternational   => vec!["board foot"],
            Atom::BoltzmannConstant        => vec!["Boltzmann constant"],
            Atom::BushelBR |
                Atom::BushelUS             => vec!["bushel"],
            Atom::CircularMilInternational => vec!["circular mil"],
            Atom::CordInternational |
                Atom::CordUS               => vec!["cord"],
            Atom::CubicFootInternational   => vec!["cubic foot"],
            Atom::CubicInchInternational   => vec!["cubic inch"],
            Atom::CubicYardInternational   => vec!["cubic yard"],
            Atom::CupUS                    => vec!["cup"],
            Atom::Curie                    => vec!["Curie"],

            Atom::Day                 => vec!["day"],
            Atom::Degree              => vec!["degree"],
            Atom::DegreeCelsius       => vec!["degree Celsius"],
            Atom::DegreeFahrenheit    => vec!["degree Fahrenheit"],
            Atom::DegreeMinute |
                Atom::Minute          => vec!["minute"],
            Atom::DegreeReaumur       => vec!["degree Réaumur"],
            Atom::DramAV              => vec!["dram"],
            Atom::DryPintUS           => vec!["dry pint"],
            Atom::DryQuartUS          => vec!["dry quart"],
            Atom::Dyne                => vec!["dyne"],
            Atom::ElectronMass        => vec!["electron mass"],
            Atom::ElectronVolt        => vec!["electronvolt"],
            Atom::ElementaryCharge    => vec!["elementary charge"],
            Atom::Equivalents         => vec!["equivalents"],
            Atom::Erg                 => vec!["erg"],
            Atom::Farad               => vec!["farad"],
            Atom::FathomInternational |
                Atom::FathomBR        |
                Atom::FathomUS => vec!["fathom"],
            Atom::FluidDramBR  |
                Atom::FluidDramUS  => vec!["fluid dram"],
            Atom::FluidOunceBR |
                Atom::FluidOunceUS => vec!["fluid ounce"],
            Atom::FootInternational |
                Atom::FootBR        |
                Atom::FootUS                     => vec!["foot"],
            Atom::FurlongUS                      => vec!["furlong"],

            Atom::Gal                            => vec!["Gal"],
            Atom::GallonBR                       => vec!["gallon"],
            Atom::Gauss                          => vec!["Gauss"],
            Atom::Gilbert                        => vec!["Gilbert"],
            Atom::GillBR |
                Atom::GillUS                     => vec!["gill"],
            Atom::Gon                            => vec!["gon", "grade"],
            Atom::GramForce                      => vec!["gram-force"],
            Atom::GramPercent                    => vec!["gram percent"],
            Atom::Grain                          => vec!["grain"],
            Atom::Gray                           => vec!["gray"],
            Atom::GuntersChainBR                 => vec!["Gunter's chain"],
            Atom::GuntersChainUS                 => vec!["Gunter's chain", "Surveyor's chain"],
            Atom::HandInternational              => vec!["hand"],
            Atom::Henry                          => vec!["henry"],
            Atom::Hertz                          => vec!["hertz"],
            Atom::HistoricalWinchesterGallon     => vec!["historical winchester gallon"],
            Atom::Horsepower                     => vec!["horsepower"],
            Atom::Hour                           => vec!["hour"],
            Atom::InchInternational |
                Atom::InchBR        |
                Atom::InchUS                     => vec!["inch"],

            Atom::Joule                          => vec!["joule"],
            Atom::Lambert                        => vec!["Lambert"],
            Atom::LongHundredweightAV            => vec!["long hundredweight", "British hundredweight"],
            Atom::LongTonAV                      => vec!["long ton", "British ton"],
            Atom::Kayser                         => vec!["Kayser"],
            Atom::KnotBR |
                Atom::KnotInternational          => vec!["knot"],
            Atom::LinkForGuntersChainBR |
                Atom::LinkForGuntersChainUS      => vec!["link for Gunter's chain"],
            Atom::LinkForRamdensChainUS          => vec!["link for Ramden's chain"],
            Atom::LightYear                      => vec!["light-year"],
            Atom::Liter                          => vec!["liter"],
            Atom::Lumen                          => vec!["lumen"],
            Atom::Lux                            => vec!["lux"],

            Atom::Maxwell                        => vec!["Maxwell"],
            Atom::MeanGregorianMonth             => vec!["mean Gregorian month"],
            Atom::MeanGregorianYear              => vec!["mean Gregorian year"],
            Atom::MeanJulianMonth                => vec!["mean Julian month"],
            Atom::MeanJulianYear                 => vec!["mean Julian year"],
            Atom::MetricCup                      => vec!["metric cup"],
            Atom::MetricFluidOunce               => vec!["metric fluid ounce"],
            Atom::MetricTablespoon               => vec!["metric tablespoon"],
            Atom::MetricTeaspoon                 => vec!["metric teaspoon"],
            Atom::MilInternational |
                Atom::MilUS                      => vec!["mil"],
            Atom::MileInternational |
                Atom::MileBR        |
                Atom::MileUS                     => vec!["mile"],
            Atom::MinimBR |
                Atom::MinimUS                    => vec!["minim"],
            Atom::Mole                           => vec!["mole"],
            Atom::Month                          => vec!["month"],
            Atom::Newton                         => vec!["newton"],
            Atom::NauticalMileBR |
                Atom::NauticalMileInternational  => vec!["nautical mile"],
            Atom::NewtonianConstantOfGravitation => vec!["Newtonian constant of gravitation"],
            Atom::Ohm                            => vec!["ohm"],
            Atom::Oersted                        => vec!["Oersted"],
            Atom::OunceAV |
                Atom::OunceTR                    => vec!["ounce"],

            Atom::PH                             => vec!["pH"],
            Atom::PaceBR                         => vec!["pace"],
            Atom::Parsec                         => vec!["parsec"],
            Atom::PartsPerBillion                => vec!["parts per billion"],
            Atom::PartsPerMillion                => vec!["parts per million"],
            Atom::PartsPerThousand               => vec!["parts per thousand"],
            Atom::Pascal                         => vec!["pascal"],
            Atom::Percent                        => vec!["percent"],
            Atom::PermeabilityOfVacuum           => vec!["permeability of vacuum"],
            Atom::PermittivityOfVacuum           => vec!["permittivity of vacuum"],
            Atom::Phot                           => vec!["phot"],
            Atom::PeckBR |
                Atom::PeckUS                     => vec!["peck"],
            Atom::PennyweightTR                  => vec!["pennyweight"],
            Atom::PintBR |
                Atom::PintUS                     => vec!["pint"],
            Atom::PlanckConstant                 => vec!["Planck constant"],
            Atom::Poise                          => vec!["Poise"],
            Atom::PoundAV |
                Atom::PoundTR                    => vec!["pound"],
            Atom::PoundForce                     => vec!["pound force"],
            Atom::PrismDiopter                   => vec!["prism diopter"],
            Atom::ProteinNitrogenUnit            => vec!["protein nitrogen unit"],
            Atom::ProtonMass                     => vec!["proton mass"],
            Atom::QuartBR |
                Atom::QuartUS                    => vec!["quart"],
            Atom::QueenAnnesWineGallon           => vec!["Queen Ann's wine gallon"],
            Atom::RadiationAbsorbedDose          => vec!["radiation absorbed dose"],
            Atom::RadiationEquivalentMan         => vec!["radiation equivalent man"],
            Atom::RamdensChainUS                 => vec!["Ramden's chain", "Engineer's chain"],
            Atom::RodBR |
                Atom::RodUS                      => vec!["rod"],
            Atom::Roentgen                       => vec!["Roentgen"],

            Atom::Section                        => vec!["section"],
            Atom::ShortHundredweightAV           => vec!["short hundredweight", "U.S. hundredweight"],
            Atom::ShortTonAV                     => vec!["short ton", "U.S. ton"],
            Atom::Siemens                        => vec!["siemens"],
            Atom::Sievert                        => vec!["sievert"],
            Atom::SquareFootInternational        => vec!["square foot"],
            Atom::SquareInchInternational        => vec!["square inch"],
            Atom::SquareMileUS                   => vec!["square mile"],
            Atom::SquareRodUS                    => vec!["square rod"],
            Atom::SquareYardInternational        => vec!["square yard"],
            Atom::StandardAccelerationOfFreeFall => vec!["standard acceleration of free fall"],
            Atom::StandardAtmosphere             => vec!["standard atmosphere"],
            Atom::Steradian                      => vec!["steradian"],
            Atom::Stilb                          => vec!["stilb"],
            Atom::StoneAV                        => vec!["stone", "British stone"],
            Atom::Stokes                         => vec!["Stokes"],
            Atom::SynodalMonth                   => vec!["synodal month"],
            Atom::TablespoonUS                   => vec!["tablespoon"],
            Atom::TeaspoonUS                     => vec!["teaspoon"],
            Atom::Tesla                          => vec!["tesla"],
            Atom::TheNumberPi                    => vec!["the number pi"],
            Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => {
                vec!["the number ten for arbitrary powers"]
            },
            Atom::Tonne           => vec!["tonne"],
            Atom::Township        => vec!["township"],
            Atom::TropicalYear    => vec!["tropical year"],

            Atom::VelocityOfLight   => vec!["velocity of light"],
            Atom::Volt              => vec!["volt"],
            Atom::Watt              => vec!["watt"],
            Atom::Weber             => vec!["weber"],
            Atom::Week              => vec!["week"],
            Atom::YardBR |
                Atom::YardInternational |
                Atom::YardUS        => vec!["yard"],
            Atom::Year              => vec!["year"],
        }
    }

    fn primary_code(&self) -> &'static str {
        match *self {
            Atom::TheUnity => "1",
            Atom::Candela  => "cd",
            Atom::Coulomb  => "C",
            Atom::Gram     => "g",
            Atom::Kelvin   => "K",
            Atom::Meter    => "m",
            Atom::Radian   => "rad",
            Atom::Second   => "s",

            // Derived units
            Atom::AcreBR                              => "[acr_br]",
            Atom::AcreUS                              => "[acr_us]",
            Atom::Ampere                              => "A",
            Atom::Are                                 => "ar",
            Atom::AstronomicUnit                      => "AU",
            Atom::AtomicMassUnit                      => "u",
            Atom::Bar                                 => "bar",
            Atom::BarrelUS                            => "[bbl_us]",
            Atom::Becquerel                           => "Bq",
            Atom::Biot                                => "Bi",
            Atom::BoardFootInternational              => "[bf_i]",
            Atom::BoltzmannConstant                   => "[k]",
            Atom::BushelBR                            => "[bu_br]",
            Atom::BushelUS                            => "[bu_us]",
            Atom::CircularMilInternational            => "[cml_i]",
            Atom::CordInternational                   => "[cr_i]",
            Atom::CordUS                              => "[crd_us]",
            Atom::CubicFootInternational              => "[cft_i]",
            Atom::CubicInchInternational              => "[cin_i]",
            Atom::CubicYardInternational              => "[cyd_i]",
            Atom::CupUS                               => "[cup_us]",
            Atom::Curie                               => "Ci",

            Atom::Day                                 => "d",
            Atom::Degree                              => "deg",
            Atom::DegreeCelsius                       => "Cel",
            Atom::DegreeFahrenheit                    => "[degF]",
            Atom::DegreeMinute                        => "'",
            Atom::DegreeReaumur                       => "[degRe]",
            Atom::DegreeSecond                        => "''",
            Atom::DramAV                              => "[dr_av]",
            Atom::DryPintUS                           => "[dpt_us]",
            Atom::DryQuartUS                          => "[dqt_us]",
            Atom::Dyne                                => "dyn",
            Atom::ElectronMass                        => "[m_e]",
            Atom::ElectronVolt                        => "eV",
            Atom::ElementaryCharge                    => "[e]",
            Atom::Equivalents                         => "eq",
            Atom::Erg                                 => "erg",
            Atom::Farad                               => "F",
            Atom::FathomInternational                 => "[fth_i]",
            Atom::FathomBR                            => "[fth_br]",
            Atom::FathomUS                            => "[fth_us]",
            Atom::FluidDramBR                         => "[fdr_br]",
            Atom::FluidDramUS                         => "[fdr_us]",
            Atom::FluidOunceBR                        => "[foz_br]",
            Atom::FluidOunceUS                        => "[foz_us]",
            Atom::FootInternational                   => "[ft_i]",
            Atom::FootBR                              => "[ft_br]",
            Atom::FootUS                              => "[ft_us]",
            Atom::FurlongUS                           => "[fur_us]",

            Atom::Gal                                 => "Gal",
            Atom::GallonBR                            => "[gal_br]",
            Atom::Gauss                               => "G",
            Atom::Gilbert                             => "Gb",
            Atom::GillBR                              => "[gil_br]",
            Atom::GillUS                              => "[gil_us]",
            Atom::Gon                                 => "gon",
            Atom::GramForce                           => "gf",
            Atom::GramPercent                         => "g%",
            Atom::Grain                               => "[gr]",
            Atom::Gray                                => "Gy",
            Atom::GuntersChainBR                      => "[ch_br]",
            Atom::GuntersChainUS                      => "[ch_us]",
            Atom::HandInternational                   => "[hd_i]",
            Atom::Henry                               => "H",
            Atom::Hertz                               => "Hz",
            Atom::HistoricalWinchesterGallon          => "[gal_wi]",
            Atom::Horsepower                          => "[HP]",
            Atom::Hour                                => "h",
            Atom::InchInternational                   => "[in_i]",
            Atom::InchBR                              => "[in_br]",
            Atom::InchUS                              => "[in_us]",

            Atom::Joule                               => "J",
            Atom::Kayser                              => "Ky",
            Atom::KnotBR                              => "[kn_br]",
            Atom::KnotInternational                   => "[kn_i]",
            Atom::Lambert                             => "Lmb",
            Atom::LongHundredweightAV                 => "[lcwt_av]",
            Atom::LongTonAV                           => "[lton_av]",
            Atom::LightYear                           => "[ly]",
            Atom::LinkForGuntersChainBR               => "[lk_br]",
            Atom::LinkForGuntersChainUS               => "[lk_us]",
            Atom::LinkForRamdensChainUS               => "[rlk_us]",
            Atom::Liter                               => "l",
            Atom::Lumen                               => "lm",
            Atom::Lux                                 => "lx",

            Atom::Maxwell                             => "Mx",
            Atom::MeanGregorianMonth                  => "mo_g",
            Atom::MeanGregorianYear                   => "a_g",
            Atom::MeanJulianMonth                     => "mo_j",
            Atom::MeanJulianYear                      => "a_j",
            Atom::MetricCup                           => "[cup_m]",
            Atom::MetricFluidOunce                    => "[foz_m]",
            Atom::MetricTablespoon                    => "[tbs_m]",
            Atom::MetricTeaspoon                      => "[tsp_m]",
            Atom::MilInternational                    => "[mil_i]",
            Atom::MilUS                               => "[mil_us]",
            Atom::MileInternational                   => "[mi_i]",
            Atom::MileBR                              => "[mi_br]",
            Atom::MileUS                              => "[mi_us]",
            Atom::MinimBR                             => "[min_br]",
            Atom::MinimUS                             => "[min_us]",
            Atom::Minute                              => "min",
            Atom::Mole                                => "mol",
            Atom::Month                               => "mo",
            Atom::NauticalMileBR                      => "[nmi_br]",
            Atom::NauticalMileInternational           => "[nmi_i]",
            Atom::Newton                              => "N",
            Atom::NewtonianConstantOfGravitation      => "[G]",
            Atom::Ohm                                 => "Ohm",
            Atom::Oersted                             => "Oe",
            Atom::OunceAV                             => "[oz_av]",
            Atom::OunceTR                             => "[oz_tr]",

            Atom::PH                                  => "[pH]",
            Atom::PaceBR                              => "[pc_br]",
            Atom::Parsec                              => "pc",
            Atom::PartsPerBillion                     => "[ppb]",
            Atom::PartsPerMillion                     => "[ppm]",
            Atom::PartsPerThousand                    => "[ppth]",
            Atom::Pascal                              => "Pa",
            Atom::Percent                             => "%",
            Atom::PermeabilityOfVacuum                => "[mu_0]",
            Atom::PermittivityOfVacuum                => "[eps_0]",
            Atom::Phot                                => "ph",
            Atom::PeckBR                              => "[pk_br]",
            Atom::PeckUS                              => "[pk_us]",
            Atom::PennyweightTR                       => "[pwt_tr]",
            Atom::PintBR                              => "[pt_br]",
            Atom::PintUS                              => "[pt_us]",
            Atom::PlanckConstant                      => "[h]",
            Atom::Poise                               => "P",
            Atom::PoundAV                             => "[lb_av]",
            Atom::PoundTR                             => "[lb_tr]",
            Atom::PoundForce                          => "[lbf_av]",
            Atom::PrismDiopter                        => "[p'diop]",
            Atom::ProteinNitrogenUnit                 => "[PNU]",
            Atom::ProtonMass                          => "[m_p]",
            Atom::QuartBR                             => "[qt_br]",
            Atom::QuartUS                             => "[qt_us]",
            Atom::QueenAnnesWineGallon                => "[gal_us]",
            Atom::RadiationAbsorbedDose               => "RAD",
            Atom::RadiationEquivalentMan              => "REM",
            Atom::RamdensChainUS                      => "[rch_us]",
            Atom::RodBR                               => "[rd_br]",
            Atom::RodUS                               => "[rd_us]",
            Atom::Roentgen                            => "R",

            Atom::Section                             => "[sct]",
            Atom::ShortHundredweightAV                => "[scwt_av]",
            Atom::ShortTonAV                          => "[ston_av]",
            Atom::Siemens                             => "S",
            Atom::Sievert                             => "Sv",
            Atom::SquareFootInternational             => "[sft_i]",
            Atom::SquareInchInternational             => "[sin_i]",
            Atom::SquareMileUS                        => "[smi_us]",
            Atom::SquareRodUS                         => "[srd_us]",
            Atom::SquareYardInternational             => "[syd_i]",
            Atom::StandardAccelerationOfFreeFall      => "[g]",
            Atom::StandardAtmosphere                  => "atm",
            Atom::Steradian                           => "sr",
            Atom::Stilb                               => "sb",
            Atom::StoneAV                             => "[stone_av]",
            Atom::Stokes                              => "St",
            Atom::SynodalMonth                        => "mo_s",
            Atom::TablespoonUS                        => "[tbs_us]",
            Atom::TeaspoonUS                          => "[tsp_us]",
            Atom::Tesla                               => "T",
            Atom::TheNumberPi                         => "[pi]",
            Atom::TheNumberTenForArbitraryPowersCaret => "10^",
            Atom::TheNumberTenForArbitraryPowersStar  => "10*",
            Atom::Tonne                               => "t",
            Atom::Township                            => "[twp]",
            Atom::TropicalYear                        => "a_t",

            Atom::VelocityOfLight                     => "[c]",
            Atom::Volt                                => "V",
            Atom::Watt                                => "W",
            Atom::Weber                               => "Wb",
            Atom::Week                                => "wk",
            Atom::YardInternational                   => "[yd_i]",
            Atom::YardBR                              => "[yd_br]",
            Atom::YardUS                              => "[yd_us]",
            Atom::Year                                => "a",
        }
    }

    fn print_symbol(&self) -> Option<&'static str> {
        match *self {
            Atom::Candela       |
                Atom::Coulomb   |
                Atom::Gram      |
                Atom::Kelvin    |
                Atom::Meter     |
                Atom::Radian    |
                Atom::Second    |

                Atom::Ampere             |
                Atom::AstronomicUnit     |
                Atom::AtomicMassUnit     |
                Atom::Bar                |
                Atom::Becquerel          |
                Atom::Biot               |
                Atom::Curie              |

                Atom::Day                |
                Atom::DegreeMinute       |
                Atom::DegreeSecond       |
                Atom::Dyne               |
                Atom::ElectronVolt       |
                Atom::Erg                |
                Atom::Farad              |

                Atom::Gal                |
                Atom::Gilbert            |
                Atom::GramForce          |
                Atom::GramPercent        |
                Atom::Gray               |
                Atom::Henry              |
                Atom::Hertz              |
                Atom::Hour               |

                Atom::Joule              |
                Atom::Liter              |
                Atom::Lumen              |
                Atom::Lux                |

                Atom::Maxwell            |
                Atom::MeanGregorianMonth |
                Atom::MeanGregorianYear  |
                Atom::Minute             |
                Atom::Mole               |
                Atom::Month              |
                Atom::Oersted            |

                Atom::Parsec                 |
                Atom::Pascal                 |
                Atom::Percent                |
                Atom::Phot                   |
                Atom::Poise                  |
                Atom::RadiationAbsorbedDose  |
                Atom::RadiationEquivalentMan |
                Atom::Roentgen               |

                Atom::Siemens            |
                Atom::Sievert            |
                Atom::StandardAtmosphere |
                Atom::Steradian          |
                Atom::Stilb              |
                Atom::Stokes             |
                Atom::Tesla              |
                Atom::Tonne              |

                Atom::Volt               |
                Atom::Watt               |
                Atom::Weber              |
                Atom::Week               |
                Atom::Year                       => Some(self.primary_code()),
            Atom::Are                            => Some("a"),
            Atom::BoltzmannConstant              => Some("𝑘"),
            Atom::CircularMilInternational       => Some("circ.mil"),
            Atom::CubicYardInternational         => Some("cu.yd"),

            Atom::Degree                         => Some("°"),
            Atom::DegreeCelsius                  => Some("°C"),
            Atom::DegreeFahrenheit               => Some("°F"),
            Atom::DegreeReaumur                  => Some("°R"),
            Atom::ElectronMass                   => Some("𝑚ₑ"),
            Atom::ElementaryCharge               => Some("𝑒"),
            Atom::Equivalents                    => Some("eq"),
            Atom::FathomInternational            => Some("fth"),
            Atom::FluidOunceUS |
                Atom::MetricFluidOunce           => Some("oz fl"),
            Atom::FootInternational              => Some("ft"),
            Atom::FootUS                         => Some("ft (us)"),

            Atom::Gauss                          => Some("Gs"),
            Atom::Gon                            => Some("□"),
            Atom::HandInternational              => Some("hd"),

            Atom::Kayser                         => Some("K"),
            Atom::KnotInternational              => Some("knot"),
            Atom::Lambert                        => Some("L"),
            Atom::LightYear                      => Some("l.y."),

            Atom::MeanJulianMonth                => Some("moⱼ"),
            Atom::MeanJulianYear                 => Some("aⱼ"),
            Atom::MilInternational               => Some("mil"),
            Atom::MileInternational              => Some("mi"),
            Atom::NauticalMileInternational      => Some("n.mi"),
            Atom::Ohm                            => Some("Ω"),
            Atom::OunceAV                        => Some("oz"),

            Atom::PartsPerBillion                => Some("ppb"),
            Atom::PartsPerMillion                => Some("ppm"),
            Atom::PartsPerThousand               => Some("ppth"),
            Atom::PH                             => Some("pH"),
            Atom::PermeabilityOfVacuum           => Some("μ₀"),
            Atom::PermittivityOfVacuum           => Some("ε₀"),
            Atom::PlanckConstant                 => Some("ℎ"),
            Atom::PoundAV                        => Some("lb"),
            Atom::PoundForce                     => Some("lbf"),
            Atom::PrismDiopter                   => Some("PD"),
            Atom::ProteinNitrogenUnit            => Some("PNU"),
            Atom::ProtonMass                     => Some("𝑚ₚ"),

            Atom::StandardAccelerationOfFreeFall => Some("𝑔"),
            Atom::SynodalMonth                   => Some("moₛ"),
            Atom::TheNumberPi                    => Some("π"),
            Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => Some("10"),
            Atom::TropicalYear      => Some("aₜ"),

            Atom::VelocityOfLight   => Some("𝑐"),
            Atom::YardInternational => Some("yd"),
            _ => None,
        }
    }

    fn property(&self) -> Property {
        match *self {
            Atom::Gal |
                Atom::StandardAccelerationOfFreeFall        => Property::Acceleration,
            Atom::PH                                        => Property::Acidity,
            Atom::PlanckConstant                            => Property::Action,
            Atom::Equivalents |
                Atom::Mole                                  => Property::AmountOfSubstance,
            Atom::AcreBR                       |
                Atom::AcreUS                   |
                Atom::Are                      |
                Atom::CircularMilInternational |
                Atom::Section                  |
                Atom::SquareFootInternational  |
                Atom::SquareInchInternational  |
                Atom::SquareMileUS             |
                Atom::SquareRodUS              |
                Atom::SquareYardInternational  |
                Atom::Township                              => Property::Area,
            Atom::Lambert                                   => Property::Brightness,
            Atom::FathomInternational                       => Property::DepthOfWater,
            Atom::RadiationEquivalentMan |
                Atom::Sievert                               => Property::DoseEquivalent,
            Atom::BushelUS       |
                Atom::PeckUS     |
                Atom::DryPintUS  |
                Atom::DryQuartUS |
                Atom::HistoricalWinchesterGallon            => Property::DryVolume,
            Atom::Poise                                     => Property::DynamicViscosity,
            Atom::Coulomb  |
                Atom::ElementaryCharge                      => Property::ElectricCharge,
            Atom::Meter                         |
                Atom::AstronomicUnit            |
                Atom::FathomBR                  |
                Atom::FathomUS                  |
                Atom::FootInternational         |
                Atom::FootBR                    |
                Atom::FootUS                    |
                Atom::FurlongUS                 |
                Atom::GuntersChainBR            |
                Atom::GuntersChainUS            |
                Atom::InchInternational         |
                Atom::InchBR                    |
                Atom::InchUS                    |
                Atom::LightYear                 |
                Atom::LinkForGuntersChainBR     |
                Atom::LinkForGuntersChainUS     |
                Atom::LinkForRamdensChainUS     |
                Atom::MilInternational          |
                Atom::MilUS                     |
                Atom::MileInternational         |
                Atom::MileBR                    |
                Atom::MileUS                    |
                Atom::NauticalMileBR            |
                Atom::NauticalMileInternational |
                Atom::PaceBR                    |
                Atom::Parsec                    |
                Atom::RamdensChainUS            |
                Atom::RodBR                     |
                Atom::RodUS                     |
                Atom::YardInternational         |
                Atom::YardBR                    |
                Atom::YardUS                                => Property::Length,
            Atom::Farad                                     => Property::ElectricCapacitance,
            Atom::Siemens                                   => Property::ElectricConductance,
            Atom::Ampere |
                Atom::Biot                                  => Property::ElectricCurrent,
            Atom::PermittivityOfVacuum                      => Property::ElectricPermittivity,
            Atom::Volt                                      => Property::ElectricPotential,
            Atom::Ohm                                       => Property::ElectricResistance,
            Atom::Joule            |
                Atom::ElectronVolt |
                Atom::Erg                                   => Property::Energy,
            Atom::Gray |
                Atom::RadiationAbsorbedDose                 => Property::EnergyDose,
            Atom::BarrelUS             |
                Atom::CordUS           |
                Atom::FluidDramUS      |
                Atom::FluidOunceUS     |
                Atom::GillUS           |
                Atom::MetricFluidOunce |
                Atom::MinimUS          |
                Atom::PintUS           |
                Atom::QuartUS          |
                Atom::QueenAnnesWineGallon                  => Property::FluidVolume,
            Atom::Maxwell                                   => Property::FluxOfMagneticInduction,
            Atom::Newton        |
                Atom::Dyne      |
                Atom::GramForce |
                Atom::PoundForce                            => Property::Force,
            Atom::PartsPerBillion     |
                Atom::PartsPerMillion |
                Atom::PartsPerThousand                      => Property::Fraction,
            Atom::Hertz                                     => Property::Frequency,
            Atom::HandInternational                         => Property::HeightOfHorses,
            Atom::Lux |
                Atom::Phot                                  => Property::Illuminance,
            Atom::Henry                                     => Property::Inductance,
            Atom::Roentgen                                  => Property::IonDose,
            Atom::Stokes                                    => Property::KinematicViscosity,
            Atom::Kayser                                    => Property::LineicNumber,
            Atom::Lumen                                     => Property::LuminousFlux,
            Atom::Candela                                   => Property::LuminousIntensity,
            Atom::Stilb                                     => Property::LuminousIntensityDensity,
            Atom::Oersted                                   => Property::MagneticFieldIntensity,
            Atom::Weber                                     => Property::MagneticFlux,
            Atom::Gauss |
                Atom::Tesla                                 => Property::MagneticFluxDensity,
            Atom::PermeabilityOfVacuum                      => Property::MagneticPermeability,
            Atom::Gilbert                                   => Property::MagneticTension,
            Atom::AtomicMassUnit           |
                Atom::DramAV               |
                Atom::Grain                |
                Atom::Gram                 |
                Atom::ElectronMass         |
                Atom::LongHundredweightAV  |
                Atom::LongTonAV            |
                Atom::OunceAV              |
                Atom::OunceTR              |
                Atom::PennyweightTR        |
                Atom::PoundAV              |
                Atom::PoundTR              |
                Atom::ProtonMass           |
                Atom::ShortHundredweightAV |
                Atom::ShortTonAV           |
                Atom::StoneAV              |
                Atom::Tonne                                 => Property::Mass,
            Atom::GramPercent                               => Property::MassConcentration,
            Atom::Percent                                 |
                Atom::TheNumberPi                         |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar    => Property::Number,
            Atom::Radian           |
                Atom::Degree       |
                Atom::DegreeMinute |
                Atom::DegreeSecond |
                Atom::Gon                                   => Property::PlaneAngle,
            Atom::Horsepower |
                Atom::Watt                                  => Property::Power,
            Atom::Pascal  |
                Atom::Bar |
                Atom::StandardAtmosphere                    => Property::Pressure,
            Atom::ProteinNitrogenUnit                       => Property::ProcedureDefinedProtein,
            Atom::PrismDiopter                              => Property::RefractionOfPrism,
            Atom::Becquerel |
                Atom::Curie                                 => Property::Radioactivity,
            Atom::Steradian                                 => Property::SolidAngle,
            Atom::Kelvin               |
                Atom::DegreeCelsius    |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur                         => Property::Temperature,
            Atom::Second                 |
                Atom::Day                |
                Atom::Hour               |
                Atom::MeanGregorianMonth |
                Atom::MeanGregorianYear  |
                Atom::MeanJulianMonth    |
                Atom::MeanJulianYear     |
                Atom::Minute             |
                Atom::Month              |
                Atom::SynodalMonth       |
                Atom::TropicalYear       |
                Atom::Week               |
                Atom::Year                                  => Property::Time,
            Atom::TheUnity              |
                Atom::BoltzmannConstant |
                Atom::NewtonianConstantOfGravitation        => Property::Unclassified,
            Atom::KnotBR                |
                Atom::KnotInternational |
                Atom::VelocityOfLight                       => Property::Velocity,
            Atom::BoardFootInternational     |
                Atom::BushelBR               |
                Atom::CordInternational      |
                Atom::CubicFootInternational |
                Atom::CubicInchInternational |
                Atom::CubicYardInternational |
                Atom::CupUS                  |
                Atom::FluidDramBR            |
                Atom::FluidOunceBR           |
                Atom::GallonBR               |
                Atom::GillBR                 |
                Atom::Liter                  |
                Atom::MetricCup              |
                Atom::MetricTablespoon       |
                Atom::MetricTeaspoon         |
                Atom::MinimBR                |
                Atom::PeckBR                 |
                Atom::PintBR                 |
                Atom::QuartBR                |
                Atom::TablespoonUS           |
                Atom::TeaspoonUS                            => Property::Volume,
        }
    }

    fn secondary_code(&self) -> &'static str {
        match *self {
            Atom::TheUnity => "1",
            Atom::Candela  => "CD",
            Atom::Coulomb  => "C",
            Atom::Gram     => "G",
            Atom::Meter    => "M",
            Atom::Radian   => "RAD",
            Atom::Second   => "S",

            // Derived units
            Atom::AcreBR                         => "[ACR_BR]",
            Atom::AcreUS                         => "[ACR_US]",
            Atom::Are                            => "AR",
            Atom::AstronomicUnit                 => "AU",
            Atom::AtomicMassUnit                 => "AMU",
            Atom::Bar                            => "BAR",
            Atom::BarrelUS                       => "[BBL_US]",
            Atom::Becquerel                      => "BQ",
            Atom::Biot                           => "BI",
            Atom::BoardFootInternational         => "[BF_I]",
            Atom::BoltzmannConstant              => "[K]",
            Atom::BushelBR                       => "[BU_BR]",
            Atom::BushelUS                       => "[BU_US]",
            Atom::CircularMilInternational       => "[CML_I]",
            Atom::CordInternational              => "[CR_I]",
            Atom::CordUS                         => "[CRD_US]",
            Atom::CubicFootInternational         => "[CFT_I]",
            Atom::CubicInchInternational         => "[CIN_I]",
            Atom::CubicYardInternational         => "[CYD_I]",
            Atom::CupUS                          => "[CUP_US]",
            Atom::Curie                          => "CI",

            Atom::Day                            => "D",
            Atom::Degree                         => "DEG",
            Atom::DegreeCelsius                  => "CEL",
            Atom::DegreeFahrenheit               => "[DEGF]",
            Atom::DramAV                         => "[DR_AV]",
            Atom::DryPintUS                      => "[DPT_US]",
            Atom::DryQuartUS                     => "[DQT_US]",
            Atom::Dyne                           => "DYN",
            Atom::ElectronMass                   => "[M_E]",
            Atom::ElectronVolt                   => "EV",
            Atom::ElementaryCharge               => "[E]",
            Atom::Equivalents                    => "EQ",
            Atom::Erg                            => "ERG",
            Atom::FathomInternational            => "[FTH_I]",
            Atom::FathomBR                       => "[FTH_BR]",
            Atom::FathomUS                       => "[FTH_US]",
            Atom::FluidDramBR                    => "[FDR_BR]",
            Atom::FluidDramUS                    => "[FDR_US]",
            Atom::FluidOunceBR                   => "[FOZ_BR]",
            Atom::FluidOunceUS                   => "[FOZ_US]",
            Atom::FootInternational              => "[FT_I]",
            Atom::FootBR                         => "[FT_BR]",
            Atom::FootUS                         => "[FT_US]",
            Atom::FurlongUS                      => "[FUR_US]",

            Atom::Gal                            => "GL",
            Atom::GallonBR                       => "[GAL_BR]",
            Atom::Gauss                          => "GS",
            Atom::Gilbert                        => "GB",
            Atom::GillBR                         => "[GIL_BR]",
            Atom::GillUS                         => "[GIL_US]",
            Atom::Gon                            => "GON",
            Atom::GramForce                      => "GF",
            Atom::GramPercent                    => "G%",
            Atom::Grain                          => "[GR]",
            Atom::Gray                           => "GY",
            Atom::GuntersChainBR                 => "[CH_BR]",
            Atom::GuntersChainUS                 => "[CH_US]",
            Atom::HandInternational              => "[HD_I]",
            Atom::Hertz                          => "HZ",
            Atom::HistoricalWinchesterGallon     => "[GAL_WI]",
            Atom::Hour                           => "HR",
            Atom::InchInternational              => "[IN_I]",
            Atom::InchBR                         => "[IN_BR]",
            Atom::InchUS                         => "[IN_US]",

            Atom::Kayser                         => "KY",
            Atom::KnotBR                         => "[KN_BR]",
            Atom::KnotInternational              => "[KN_I]",
            Atom::Lambert                        => "LMB",
            Atom::LongHundredweightAV            => "[LCWT_AV]",
            Atom::LongTonAV                      => "[LTON_AV]",
            Atom::LightYear                      => "[LY]",
            Atom::LinkForGuntersChainBR          => "[LK_BR]",
            Atom::LinkForGuntersChainUS          => "[LK_US]",
            Atom::LinkForRamdensChainUS          => "[RLK_US]",
            Atom::Liter                          => "L",
            Atom::Lumen                          => "LM",
            Atom::Lux                            => "LX",

            Atom::Maxwell                        => "Mx",
            Atom::MeanGregorianMonth             => "MO_G",
            Atom::MeanGregorianYear              => "ANN_G",
            Atom::MeanJulianMonth                => "MO_J",
            Atom::MeanJulianYear                 => "ANN_J",
            Atom::MetricCup                      => "[CUP_M]",
            Atom::MetricFluidOunce               => "[FOZ_M]",
            Atom::MetricTablespoon               => "[TBS_M]",
            Atom::MetricTeaspoon                 => "[TSP_M]",
            Atom::MilInternational               => "[MIL_I]",
            Atom::MilUS                          => "[MIL_US]",
            Atom::MileInternational              => "[MI_I]",
            Atom::MileBR                         => "[MI_BR]",
            Atom::MileUS                         => "[MI_US]",
            Atom::MinimBR                        => "[MIN_BR]",
            Atom::MinimUS                        => "[MIN_US]",
            Atom::Minute                         => "MIN",
            Atom::Mole                           => "MOL",
            Atom::Month                          => "MO",
            Atom::NauticalMileBR                 => "[NMI_BR]",
            Atom::NauticalMileInternational      => "[NMI_I]",
            Atom::NewtonianConstantOfGravitation => "[GC]",
            Atom::Ohm                            => "OHM",
            Atom::Oersted                        => "OE",
            Atom::OunceAV                        => "[OZ_AV]",
            Atom::OunceTR                        => "[OZ_TR]",

            Atom::PH                             => "[PH]",
            Atom::PaceBR                         => "[PC_BR]",
            Atom::Parsec                         => "PRS",
            Atom::PartsPerBillion                => "[PPB]",
            Atom::PartsPerMillion                => "[PPM]",
            Atom::PartsPerThousand               => "[PPTH]",
            Atom::Pascal                         => "PAL",
            Atom::PermeabilityOfVacuum           => "[MU_0]",
            Atom::PermittivityOfVacuum           => "[EPS_0]",
            Atom::Phot                           => "PHT",
            Atom::PeckBR                         => "[PK_BR]",
            Atom::PeckUS                         => "[PK_US]",
            Atom::PennyweightTR                  => "[PWT_TR]",
            Atom::PintBR                         => "[PT_BR]",
            Atom::PintUS                         => "[PT_US]",
            Atom::PlanckConstant                 => "[H]",
            Atom::PoundAV                        => "[LB_AV]",
            Atom::PoundTR                        => "[LB_TR]",
            Atom::PoundForce                     => "[LBF_AV]",
            Atom::PrismDiopter                   => "[P'DIOP]",
            Atom::ProtonMass                     => "[M_P]",
            Atom::QuartBR                        => "[QT_BR]",
            Atom::QuartUS                        => "[QT_US]",
            Atom::QueenAnnesWineGallon           => "[GAL_US]",
            Atom::RadiationAbsorbedDose          => "[RAD]",
            Atom::RadiationEquivalentMan         => "[REM]",
            Atom::RamdensChainUS                 => "[RCH_US]",
            Atom::RodBR                          => "[RD_BR]",
            Atom::RodUS                          => "[RD_US]",
            Atom::Roentgen                       => "ROE",

            Atom::Section                        => "[SCT]",
            Atom::ShortHundredweightAV           => "[SCWT_AV]",
            Atom::ShortTonAV                     => "[STON_AV]",
            Atom::Siemens                        => "SIE",
            Atom::Sievert                        => "SV",
            Atom::SquareFootInternational        => "[SFT_I]",
            Atom::SquareInchInternational        => "[SIN_I]",
            Atom::SquareMileUS                   => "[SMI_US]",
            Atom::SquareRodUS                    => "[SRD_US]",
            Atom::SquareYardInternational        => "[SYD_I]",
            // NOTE: Primary code of NewtonianConstantOfGravitation is [G] so this will never
            // parse:
            Atom::StandardAccelerationOfFreeFall => "[G]",
            Atom::StandardAtmosphere             => "ATM",
            Atom::Steradian                      => "SR",
            Atom::Stilb                          => "SB",
            Atom::StoneAV                        => "[STONE_AV]",
            Atom::Stokes                         => "ST",
            Atom::SynodalMonth                   => "MO_S",
            Atom::TablespoonUS                   => "[TBS_US]",
            Atom::TeaspoonUS                     => "[TSP_US]",
            Atom::TheNumberPi                    => "[PI]",
            Atom::Tonne                          => "TNE",
            Atom::Township                       => "[TWP]",
            Atom::TropicalYear                   => "ANN_T",
            Atom::VelocityOfLight                => "[C]",
            Atom::Weber                          => "WB",
            Atom::Week                           => "WK",
            Atom::YardInternational              => "[YD_I]",
            Atom::YardBR                         => "[YD_BR]",
            Atom::YardUS                         => "[YD_US]",
            Atom::Year                           => "ANN",
            Atom::Ampere                                  |
                Atom::DegreeReaumur                       |
                Atom::DegreeMinute                        |
                Atom::DegreeSecond                        |
                Atom::Farad                               |
                Atom::Henry                               |
                Atom::Horsepower                          |
                Atom::Joule                               |
                Atom::Kelvin                              |
                Atom::Newton                              |
                Atom::Percent                             |
                Atom::Poise                               |
                Atom::ProteinNitrogenUnit                 |
                Atom::Tesla                               |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar  |
                Atom::Volt                                |
                Atom::Watt => self.primary_code(),
        }
    }

    fn scalar(&self) -> f64 {
        match *self {
            Atom::TheUnity => 1.0,
            _ => self.calculate_scalar(1.0),
        }
    }

    fn magnitude(&self) -> f64 {
        match *self {
            Atom::TheUnity => 1.0,
            _ => self.calculate_magnitude(self.scalar()),
        }
    }

    fn calculate_scalar(&self, value: f64) -> f64 {
        match *self {
            Atom::TheUnity         => 1.0,
            Atom::DegreeCelsius    => value + 273.15,
            Atom::DegreeFahrenheit => 5.0 / 9.0 * (value + 459.67),
            Atom::DegreeReaumur    => (value / 0.8) + 273.15,
            Atom::PH               => 10.0_f64.powf(-value),
            Atom::PrismDiopter     => value.tan() * 100.0,
            _                      => self.definition().calculate_scalar(value),
        }
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        match *self {
            Atom::DegreeCelsius    => value - 273.15,
            Atom::DegreeFahrenheit => 9.0 * value / 5.0 - 459.67,
            Atom::DegreeReaumur    => (value - 273.15) * 0.8,
            Atom::PH               => -value.log10(), // TODO: This seems wrong...
            Atom::PrismDiopter     => (value / 100.0).atan(),
            _                      => 1.0,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "{}", self.primary_code()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Atom;
    use classification::Classification;
    // use dimension::Dimension;
    use prefix::Prefix;
    use term::Term;
    use ucum_symbol::UcumSymbol;

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
            assert_eq!(atom.classification(), Classification::SI);
        }
    }

    #[test]
    fn validate_classification_us_lengths() {
        let atoms = vec![Atom::AcreUS, Atom::FootUS, Atom::RodUS];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::USLengths);
        }
    }

    #[test]
    fn validate_classification_iso1000() {
        let atoms = vec![Atom::Are, Atom::Degree, Atom::Liter];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::ISO1000);
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
            Atom::QueenAnnesWineGallon,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::USVolumes);
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
        let terms = vec![Term::new(Some(Atom::TheUnity), None)];

        for base_atom in base_atoms {
            assert_eq!(base_atom.definition().value, 1.0);
            assert_eq!(base_atom.definition().terms, terms);
        }
    }

    #[test]
    fn validate_definition_acre_us() {
        let atom = Atom::AcreUS;
        let mut term = Term::new(Some(Atom::RodUS), None);
        term.exponent = 2;

        assert_eq!(atom.definition().value, 160.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_are() {
        let atom = Atom::Are;
        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = 2;

        assert_eq!(atom.definition().value, 100.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_degree() {
        let atom = Atom::Degree;
        let term = Term::new(Some(Atom::TheNumberPi), None);
        let term2 = Term::new(Some(Atom::Radian), None);

        let mut term3 = Term::new(None, None);
        term3.factor = 360;
        term3.exponent = -1;

        assert_eq!(atom.definition().value, 2.0);
        assert_eq!(atom.definition().terms, vec![term, term2, term3]);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_degree_celsius() {
        let atom = Atom::DegreeCelsius;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_degree_fahrenheit() {
        let atom = Atom::DegreeFahrenheit;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_degree_reaumur() {
        let atom = Atom::DegreeReaumur;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_fluid_ounce_us() {
        let atom = Atom::FluidOunceUS;
        let term = Term::new(Some(Atom::GillUS), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 4;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term, term2]);
    }

    #[test]
    fn validate_definition_foot_international() {
        let atom = Atom::FootInternational;
        let term = Term::new(Some(Atom::InchInternational), None);

        assert_eq!(atom.definition().value, 12.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_foot_us() {
        let atom = Atom::FootUS;
        let term1 = Term::new(Some(Atom::Meter), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 3937;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1200.0);
        assert_eq!(atom.definition().terms, vec![term1, term2]);
    }

    #[test]
    fn validate_definition_gill_us() {
        let atom = Atom::GillUS;
        let term = Term::new(Some(Atom::PintUS), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 4;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term, term2]);
    }

    #[test]
    fn validate_definition_inch_international() {
        let atom = Atom::InchInternational;
        let term = Term::new(Some(Atom::Meter), Some(Prefix::Centi));

        assert_eq!(atom.definition().value, 254e-2);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_liter() {
        let atom = Atom::Liter;
        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Deci));
        term.exponent = 3;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_mole() {
        let atom = Atom::Mole;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = 23;

        assert_eq!(atom.definition().value, 6.022_136_7);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_parts_per_billion() {
        let atom = Atom::PartsPerBillion;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = -9;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_parts_per_million() {
        let atom = Atom::PartsPerMillion;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = -6;

        let terms = vec![term];

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, terms);
    }

    #[test]
    fn validate_definition_parts_per_thousand() {
        let atom = Atom::PartsPerThousand;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = -3;

        let terms = vec![term];

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, terms);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_ph() {
        let atom = Atom::PH;
        let term = Term::new(Some(Atom::FootUS), None);
        let terms = vec![term];

        assert_eq!(atom.definition().value, 16.6);
        assert_eq!(atom.definition().terms, terms);
    }

    #[test]
    fn validate_definition_pint_us() {
        let atom = Atom::PintUS;
        let term = Term::new(Some(Atom::QuartUS), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 2;
        term2.exponent = -1;

        let terms = vec![term, term2];

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, terms);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_prism_diopter() {
        let atom = Atom::PrismDiopter;
        let term = Term::new(Some(Atom::QuartUS), None);
        let terms = vec![term];

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, terms);
    }

    #[test]
    fn validate_definition_quart_us() {
        let atom = Atom::QuartUS;
        let term = Term::new(Some(Atom::QueenAnnesWineGallon), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 4;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().terms, vec![term, term2]);
    }

    #[test]
    fn validate_definition_queen_annes_wine_gallon_us() {
        let atom = Atom::QueenAnnesWineGallon;
        let mut term = Term::new(Some(Atom::InchInternational), None);
        term.exponent = 3;

        assert_eq!(atom.definition().value, 231.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_rod_us() {
        let atom = Atom::RodUS;
        let term = Term::new(Some(Atom::FootUS), None);

        assert_eq!(atom.definition().value, 16.5);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_the_number_pi() {
        let atom = Atom::TheNumberPi;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(
            atom.definition().value,
            3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3
        );
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_the_number_ten_for_arbitrary_powers_caret() {
        let atom = Atom::TheNumberTenForArbitraryPowersCaret;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 10.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_definition_the_number_ten_for_arbitrary_powers_star() {
        let atom = Atom::TheNumberTenForArbitraryPowersCaret;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 10.0);
        assert_eq!(atom.definition().terms, vec![term]);
    }

    #[test]
    fn validate_scalar_base_atoms() {
        let base_atoms = vec![
            Atom::TheUnity,
            Atom::Candela,
            Atom::Coulomb,
            Atom::Gram,
            Atom::Kelvin,
            Atom::Meter,
            Atom::Radian,
            Atom::Second,
        ];
        for base_atom in base_atoms {
            assert_eq!(base_atom.scalar(), 1.0);
        }
    }

    #[test]
    fn validate_scalar_acre_us() {
        let atom = Atom::AcreUS;
        assert_relative_eq!(atom.scalar(), 4046.872_609_874_252);
        assert_ulps_eq!(atom.scalar(), 4046.872_609_874_252);
    }

    #[test]
    fn validate_scalar_are() {
        let atom = Atom::Are;
        assert_relative_eq!(atom.scalar(), 100.0);
        assert_ulps_eq!(atom.scalar(), 100.0);
    }

    #[test]
    fn validate_scalar_degree() {
        let atom = Atom::Degree;
        assert_relative_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
        assert_ulps_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_degree_celsius() {
        let atom = Atom::DegreeCelsius;
        assert_relative_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
        assert_ulps_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_degree_fahrenheit() {
        let atom = Atom::DegreeFahrenheit;
        assert_relative_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
        assert_ulps_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_degree_reaumur() {
        let atom = Atom::DegreeReaumur;
        assert_relative_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
        assert_ulps_eq!(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    fn validate_scalar_fluid_ounce_us() {
        let atom = Atom::FluidOunceUS;
        assert_relative_eq!(atom.scalar(), 2.95735295625e-05);
        assert_ulps_eq!(atom.scalar(), 2.95735295625e-05);
    }

    #[test]
    fn validate_scalar_foot_international() {
        let atom = Atom::FootInternational;
        assert_relative_eq!(atom.scalar(), 0.3048);
        assert_ulps_eq!(atom.scalar(), 0.3048);
    }

    #[test]
    fn validate_scalar_foot_us() {
        let atom = Atom::FootUS;
        assert_relative_eq!(atom.scalar(), 0.304_800_609_601_219_2);
        assert_ulps_eq!(atom.scalar(), 0.304_800_609_601_219_2);
    }

    #[test]
    fn validate_scalar_gill_us() {
        let atom = Atom::GillUS;
        assert_relative_eq!(atom.scalar(), 0.000_118_294_118_25);
        assert_ulps_eq!(atom.scalar(), 0.000_118_294_118_25);
    }

    #[test]
    fn validate_scalar_inch_international() {
        let atom = Atom::InchInternational;
        assert_relative_eq!(atom.scalar(), 0.0254);
        assert_ulps_eq!(atom.scalar(), 0.0254);
    }

    #[test]
    fn validate_scalar_liter() {
        let atom = Atom::Liter;
        assert_relative_eq!(atom.scalar(), 0.001);
        assert_ulps_eq!(atom.scalar(), 0.001);
    }

    #[test]
    fn validate_scalar_mole() {
        let atom = Atom::Mole;
        assert_relative_eq!(atom.scalar(), 6.0221367e+23);
        assert_ulps_eq!(atom.scalar(), 6.0221367e+23);
    }

    #[test]
    fn validate_scalar_parts_per_billion() {
        let atom = Atom::PartsPerBillion;
        assert_relative_eq!(atom.scalar(), 1.0e-09);
        assert_ulps_eq!(atom.scalar(), 1.0e-09);
    }

    #[test]
    fn validate_scalar_parts_per_million() {
        let atom = Atom::PartsPerMillion;
        assert_relative_eq!(atom.scalar(), 1.0e-06);
        assert_ulps_eq!(atom.scalar(), 1.0e-06);
    }

    #[test]
    fn validate_scalar_parts_per_thousand() {
        let atom = Atom::PartsPerThousand;
        assert_relative_eq!(atom.scalar(), 1.0e-03);
        assert_ulps_eq!(atom.scalar(), 1.0e-03);
    }

    #[test]
    fn validate_scalar_percent() {
        let atom = Atom::Percent;
        assert_relative_eq!(atom.scalar(), 0.01);
        assert_ulps_eq!(atom.scalar(), 0.01);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_ph() {
        let atom = Atom::PH;
        assert_relative_eq!(atom.scalar(), 1.0e-09);
        assert_ulps_eq!(atom.scalar(), 1.0e-09);
    }

    #[test]
    fn validate_scalar_pint_us() {
        let atom = Atom::PintUS;
        assert_relative_eq!(atom.scalar(), 0.000_473_176_473);
        assert_ulps_eq!(atom.scalar(), 0.000_473_176_473);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_prism_diopter() {
        let atom = Atom::PrismDiopter;
        assert_relative_eq!(atom.scalar(), 0.000_473_176_473);
        assert_ulps_eq!(atom.scalar(), 0.000_473_176_473);
    }

    #[test]
    fn validate_scalar_quart_us() {
        let atom = Atom::QuartUS;
        assert_relative_eq!(atom.scalar(), 0.000_946_352_946);
        assert_ulps_eq!(atom.scalar(), 0.000_946_352_946);
    }

    #[test]
    fn validate_scalar_queen_annes_wine_gallon() {
        let atom = Atom::QueenAnnesWineGallon;
        assert_relative_eq!(atom.scalar(), 0.003_785_411_784);
        assert_ulps_eq!(atom.scalar(), 0.003_785_411_784);
    }

    #[test]
    fn validate_scalar_rod_us() {
        let atom = Atom::RodUS;
        assert_relative_eq!(atom.scalar(), 5.029_210_058_420_117);
        assert_ulps_eq!(atom.scalar(), 5.029_210_058_420_117);
    }

    #[test]
    fn validate_scalar_the_number_pi() {
        let atom = Atom::TheNumberPi;
        assert_relative_eq!(atom.scalar(), 3.141_592_653_589_793);
        assert_ulps_eq!(atom.scalar(), 3.141_592_653_589_793);
    }

    #[test]
    fn validate_magnitude_base_atoms() {
        let base_atoms = vec![
            Atom::TheUnity,
            Atom::Candela,
            Atom::Coulomb,
            Atom::Gram,
            Atom::Kelvin,
            Atom::Meter,
            Atom::Radian,
            Atom::Second,
        ];
        for base_atom in base_atoms {
            assert_eq!(base_atom.magnitude(), 1.0);
        }
    }

    #[test]
    fn validate_magnitude_acre_us() {
        let atom = Atom::AcreUS;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_are() {
        let atom = Atom::Are;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_degree() {
        let atom = Atom::Degree;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_degree_celsius() {
        let atom = Atom::DegreeCelsius;
        assert_relative_eq!(atom.magnitude(), 0.0174_532_925_199_432_95);
        assert_ulps_eq!(atom.magnitude(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_degree_fahrenheit() {
        let atom = Atom::DegreeFahrenheit;
        assert_relative_eq!(atom.magnitude(), 0.0174_532_925_199_432_95);
        assert_ulps_eq!(atom.magnitude(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_degree_reaumur() {
        let atom = Atom::DegreeReaumur;
        assert_relative_eq!(atom.magnitude(), 0.0174_532_925_199_432_95);
        assert_ulps_eq!(atom.magnitude(), 0.0174_532_925_199_432_95);
    }

    #[test]
    fn validate_magnitude_fluid_ounce_us() {
        let atom = Atom::FluidOunceUS;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_foot_international() {
        let atom = Atom::FootInternational;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_foot_us() {
        let atom = Atom::FootUS;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_gill_us() {
        let atom = Atom::GillUS;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_inch_international() {
        let atom = Atom::InchInternational;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_liter() {
        let atom = Atom::Liter;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_mole() {
        let atom = Atom::Mole;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_parts_per_billion() {
        let atom = Atom::PartsPerBillion;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_parts_per_million() {
        let atom = Atom::PartsPerMillion;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_parts_per_thousand() {
        let atom = Atom::PartsPerThousand;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_percent() {
        let atom = Atom::Percent;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_ph() {
        let atom = Atom::PH;
        assert_relative_eq!(atom.magnitude(), 1.0e-09);
        assert_ulps_eq!(atom.magnitude(), 1.0e-09);
    }

    #[test]
    fn validate_magnitude_pint_us() {
        let atom = Atom::PintUS;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_prism_diopter() {
        let atom = Atom::PrismDiopter;
        assert_relative_eq!(atom.magnitude(), 0.000_473_176_473);
        assert_ulps_eq!(atom.magnitude(), 0.000_473_176_473);
    }

    #[test]
    fn validate_magnitude_quart_us() {
        let atom = Atom::QuartUS;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_queen_annes_wine_gallon() {
        let atom = Atom::QueenAnnesWineGallon;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_rod_us() {
        let atom = Atom::RodUS;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_the_number_pi() {
        let atom = Atom::TheNumberPi;
        assert_relative_eq!(atom.magnitude(), 1.0);
        assert_ulps_eq!(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_display() {
        let atom = Atom::TheNumberPi;
        assert_eq!(&atom.to_string(), "[pi]")
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Atom;
        use serde_json;

        #[test]
        fn validate_serialization() {
            let j = serde_json::to_string(&Atom::BushelUS)
                .expect("Couldn't convert Atom to JSON String");

            assert_eq!("\"BushelUS\"", j);
        }

        #[test]
        fn validate_deserialization() {
            let k =
                serde_json::from_str("\"BushelUS\"").expect("Couldn't convert JSON String to Atom");

            assert_eq!(Atom::BushelUS, k);
        }
    }
}
