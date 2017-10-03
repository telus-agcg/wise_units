use classification::Classification;
use composition::Composition;
use definition::Definition;
use dimension::Dimension;
use property::Property;
use std::f64::consts::PI;
use std::fmt;

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

    AcreUS,
    Ampere,
    Are,
    AstronomicUnit,
    AtomicMassUnit,
    Bar,
    Becquerel,
    Day,
    Degree,
    DegreeCelsius,
    DegreeFahrenheit,
    DegreeReaumur,
    DegreeMinute,
    DegreeSecond,
    ElectronVolt,
    Farad,
    FluidOunceUS,
    FootInternational,
    FootUS,
    GillUS,
    Gon,
    Gray,
    Hertz,
    Henry,
    Hour,
    InchInternational,
    Joule,
    Liter,
    Lumen,
    Lux,
    MeanGregorianMonth,
    MeanGregorianYear,
    MeanJulianMonth,
    MeanJulianYear,
    Minute,
    Mole,
    Month,
    Newton,
    Ohm,
    Parsec,
    Pascal,
    PartsPerBillion,
    PartsPerMillion,
    PartsPerThousand,
    Percent,
    PH,
    PintUS,
    PrismDiopter,
    QuartUS,
    QueenAnnesWineGallon,
    RodUS,
    Siemens,
    Sievert,
    Steradian,
    SynodalMonth,
    Tesla,
    TheNumberPi,
    TheNumberTenForArbitraryPowersCaret,
    TheNumberTenForArbitraryPowersStar,
    Tonne,
    TropicalYear,
    Volt,
    Watt,
    Weber,
    Week,
    Year,
}

impl Atom {
    pub fn classification(&self) -> Classification {
        match *self {
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
                Atom::Weber => Classification::SI,
            Atom::AcreUS     |
                Atom::FootUS |
                Atom::RodUS => Classification::USLengths,
            Atom::Are                    |
                Atom::AstronomicUnit     |
                Atom::AtomicMassUnit     |
                Atom::Bar                |
                Atom::Day                |
                Atom::Degree             |
                Atom::DegreeMinute       |
                Atom::DegreeSecond       |
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
                Atom::Year => Classification::ISO1000,
            Atom::DegreeFahrenheit |
                Atom::DegreeReaumur => Classification::Heat,
            Atom::FluidOunceUS |
                Atom::GillUS   |
                Atom::PintUS   |
                Atom::QuartUS  |
                Atom::QueenAnnesWineGallon => Classification::USVolumes,
            Atom::InchInternational |
                Atom::FootInternational => Classification::Intcust,
            Atom::PartsPerBillion                         |
                Atom::PartsPerMillion                     |
                Atom::PartsPerThousand                    |
                Atom::Percent                             |
                Atom::TheNumberPi                         |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => Classification::Dimless,
            Atom::PH => Classification::Chemical,
            Atom::PrismDiopter => Classification::Clinical,
        }
    }

    pub fn definition(&self) -> Definition {
        match *self {
            Atom::TheUnity    |
                Atom::Candela |
                Atom::Coulomb |
                Atom::Gram    |
                Atom::Kelvin  |
                Atom::Meter   |
                Atom::Radian  |
                Atom::Second           => Definition::new(1.0, "1"),
            Atom::AcreUS               => Definition::new(160.0, "[rd_us]2"),
            Atom::Ampere               => Definition::new(1.0, "C/s"),
            Atom::Are                  => Definition::new(100.0, "m2"),
            Atom::AstronomicUnit       => Definition::new(149_597.870_691, "Mm"),
            Atom::AtomicMassUnit       => Definition::new(1.660_540_2e-24, "g"),
            Atom::Becquerel            => Definition::new(1.0, "s-1"),
            Atom::Bar                  => Definition::new(1.0e5, "Pa"),
            Atom::Day                  => Definition::new(24.0, "h"),
            Atom::Degree               => Definition::new(2.0, "[pi].rad/360"),
            Atom::DegreeCelsius        => Definition::new(1.0, "cel(1.0 K)"),
            Atom::DegreeFahrenheit     => Definition::new(1.0, "degf(5.0 K/9)"),
            Atom::DegreeReaumur        => Definition::new(1.0, "degre(5.0 K/4)"),
            Atom::DegreeMinute         => Definition::new(1.0, "deg/60"),
            Atom::DegreeSecond         => Definition::new(1.0, "/60"),
            Atom::ElectronVolt         => Definition::new(1.0, "[e].V"),
            Atom::Farad                => Definition::new(1.0, "C/V"),
            Atom::FluidOunceUS         => Definition::new(1.0, "[gil_us]/4"),
            Atom::FootInternational    => Definition::new(12.0, "[in_i]"),
            Atom::FootUS               => Definition::new(1200.0, "m/3937"),
            Atom::GillUS               => Definition::new(1.0, "[pt_us]/4"),
            Atom::Gon                  => Definition::new(0.9, "deg"),
            Atom::Gray                 => Definition::new(1.0, "J/kg"),
            Atom::Henry                => Definition::new(1.0, "Wb/A"),
            Atom::Hertz                => Definition::new(1.0, "s-1"),
            Atom::Hour                 => Definition::new(60.0, "min"),
            Atom::InchInternational    => Definition::new(254.0e-2, "cm"),
            Atom::Joule                => Definition::new(1.0, "N.m"),
            Atom::Liter                => Definition::new(1.0, "dm3"),
            Atom::Lumen                => Definition::new(1.0, "cd.sr"),
            Atom::Lux                  => Definition::new(1.0, "lm/m2"),
            Atom::MeanGregorianMonth   => Definition::new(1.0, "a_g/12"),
            Atom::MeanGregorianYear    => Definition::new(365.2425, "d"),
            Atom::MeanJulianMonth      => Definition::new(1.0, "a_j/12"),
            Atom::MeanJulianYear       => Definition::new(365.25, "d"),
            Atom::Minute               => Definition::new(60.0, "s"),
            Atom::Mole                 => Definition::new(6.022_136_7, "10*23"),
            Atom::Month                => Definition::new(1.0, "mo_j"),
            Atom::Newton               => Definition::new(1.0, "kg.m/s2"),
            Atom::Ohm                  => Definition::new(1.0, "V/A"),
            Atom::Pascal               => Definition::new(1.0, "N/m2"),
            Atom::Parsec               => Definition::new(3.085_678e16, "m"),
            Atom::PartsPerBillion      => Definition::new(1.0, "10*-9"),
            Atom::PartsPerMillion      => Definition::new(1.0, "10*-6"),
            Atom::PartsPerThousand     => Definition::new(1.0, "10*-3"),
            Atom::Percent              => Definition::new(1.0, "10*-2"),
            Atom::PH                   => Definition::new(1.0, "ph(1.0 mol/l)"),
            Atom::PintUS               => Definition::new(1.0, "[qt_us]/2"),
            Atom::PrismDiopter         => Definition::new(1.0, "100tan(1.0 rad)"),
            Atom::QuartUS              => Definition::new(1.0, "[gal_us]/4"),
            Atom::QueenAnnesWineGallon => Definition::new(231.0, "[in_i]3"),
            Atom::RodUS                => Definition::new(16.5, "[ft_us]"),
            Atom::Siemens              => Definition::new(1.0, "Ohm-1"),
            Atom::Sievert              => Definition::new(1.0, "J/kg"),
            Atom::Steradian            => Definition::new(1.0, "rad2"),
            Atom::SynodalMonth         => Definition::new(29.530_59, "d"),
            Atom::Tesla                => Definition::new(1.0, "Wb/m2"),
            Atom::TheNumberPi          => Definition::new(PI, "1"),
            Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => Definition::new(10.0, "1"),
            Atom::TropicalYear                           => Definition::new(365.242_19, "d"),
            Atom::Tonne                                  => Definition::new(1.0e3, "kg"),
            Atom::Volt                                   => Definition::new(1.0, "J/C"),
            Atom::Watt                                   => Definition::new(1.0, "J/s"),
            Atom::Weber                                  => Definition::new(1.0, "V.s"),
            Atom::Week                                   => Definition::new(7.0, "d"),
            Atom::Year                                   => Definition::new(1.0, "a_j"),
        }
    }

    pub fn composition(&self) -> Option<Composition> {
        match *self {
            Atom::TheUnity => None,
            Atom::Candela => Some(Composition::new(Dimension::LuminousIntensity, 1)),
            Atom::Coulomb => Some(Composition::new(Dimension::ElectricCharge, 1)),
            Atom::Gram => Some(Composition::new(Dimension::Mass, 1)),
            Atom::Kelvin => Some(Composition::new(Dimension::Temperature, 1)),
            Atom::Meter => Some(Composition::new(Dimension::Length, 1)),
            Atom::Radian => Some(Composition::new(Dimension::PlaneAngle, 1)),
            Atom::Second => Some(Composition::new(Dimension::Time, 1)),
            _ => self.definition().unit.composition(),
        }
    }

    pub fn is_arbitrary(&self) -> bool {
        match *self {
            _ => false,
        }
    }

    pub fn is_metric(&self) -> bool {
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

                Atom::Ampere         |
                Atom::Are            |
                Atom::AtomicMassUnit |
                Atom::Bar            |
                Atom::Becquerel      |
                Atom::ElectronVolt   |
                Atom::Farad          |
                Atom::Gray           |
                Atom::Henry          |
                Atom::Hertz          |
                Atom::Joule          |
                Atom::Liter          |
                Atom::Lumen          |
                Atom::Lux            |
                Atom::Mole           |
                Atom::Newton         |
                Atom::Ohm            |
                Atom::Parsec         |
                Atom::Pascal         |
                Atom::Steradian      |
                Atom::Siemens        |
                Atom::Sievert        |
                Atom::Tesla          |
                Atom::Tonne          |
                Atom::Volt           |
                Atom::Watt           |
                Atom::Weber => true,
            _ => false,
        }
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Atom::DegreeCelsius        |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur    |
                Atom::PH               |
                Atom::PrismDiopter => true,
            _ => false,
        }
    }

    // TODO: can return slice instead of Vec?
    pub fn names(&self) -> Vec<&'static str> {
        match *self {
            Atom::TheUnity => vec!["the unity"],
            Atom::Candela  => vec!["candela"],
            Atom::Coulomb  => vec!["Coulomb"],
            Atom::Gram     => vec!["gram"],
            Atom::Kelvin   => vec!["Kelvin"],
            Atom::Meter    => vec!["meter"],
            Atom::Radian   => vec!["radian"],
            Atom::Second   => vec!["second"],

            // Derived units
            Atom::AcreUS           => vec!["acre"],
            Atom::Ampere           => vec!["ampère"],
            Atom::Are              => vec!["are"],
            Atom::AstronomicUnit   => vec!["astronomic unit"],
            Atom::AtomicMassUnit   => vec!["unified atomic mass unit"],
            Atom::Bar              => vec!["bar"],
            Atom::Becquerel        => vec!["becquerel"],
            Atom::Day              => vec!["day"],
            Atom::Degree           => vec!["degree"],
            Atom::DegreeCelsius    => vec!["degree Celsius"],
            Atom::DegreeFahrenheit => vec!["degree Fahrenheit"],
            Atom::DegreeReaumur    => vec!["degree Réaumur"],
            Atom::DegreeMinute     => vec!["minute"],
            Atom::DegreeSecond     => vec!["second"],
            Atom::ElectronVolt     => vec!["electronvolt"],
            Atom::Farad            => vec!["farad"],
            Atom::FluidOunceUS     => vec!["fluid once"],
            Atom::FootInternational |
                Atom::FootUS           => vec!["foot"],
            Atom::GillUS               => vec!["gill"],
            Atom::Gon                  => vec!["gon", "grade"],
            Atom::Gray                 => vec!["gray"],
            Atom::Henry                => vec!["henry"],
            Atom::Hertz                => vec!["hertz"],
            Atom::Hour                 => vec!["hour"],
            Atom::InchInternational    => vec!["inch"],
            Atom::Joule                => vec!["joule"],
            Atom::Liter                => vec!["liter"],
            Atom::Lumen                => vec!["lumen"],
            Atom::Lux                  => vec!["lux"],
            Atom::MeanGregorianMonth   => vec!["mean Gregorian month"],
            Atom::MeanGregorianYear    => vec!["mean Gregorian year"],
            Atom::MeanJulianMonth      => vec!["mean Julian month"],
            Atom::MeanJulianYear       => vec!["mean Julian year"],
            Atom::Minute               => vec!["minute"],
            Atom::Mole                 => vec!["mole"],
            Atom::Month                => vec!["month"],
            Atom::Newton               => vec!["newton"],
            Atom::Ohm                  => vec!["ohm"],
            Atom::Parsec               => vec!["parsec"],
            Atom::Pascal               => vec!["pascal"],
            Atom::PartsPerBillion      => vec!["parts per billion"],
            Atom::PartsPerMillion      => vec!["parts per million"],
            Atom::PartsPerThousand     => vec!["parts per thousand"],
            Atom::Percent              => vec!["percent"],
            Atom::PH                   => vec!["pH"],
            Atom::PintUS               => vec!["pint"],
            Atom::PrismDiopter         => vec!["prism diopter"],
            Atom::QuartUS              => vec!["quart"],
            Atom::QueenAnnesWineGallon => vec!["Queen Ann's wine gallon"],
            Atom::RodUS                => vec!["rod"],
            Atom::Steradian            => vec!["steradian"],
            Atom::Siemens              => vec!["siemens"],
            Atom::Sievert              => vec!["sievert"],
            Atom::SynodalMonth         => vec!["synodal month"],
            Atom::Tesla                => vec!["tesla"],
            Atom::TheNumberPi          => vec!["the number pi"],
            Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => {
                vec!["the number ten for arbitrary powers"]
            },
            Atom::Tonne        => vec!["tonne"],
            Atom::TropicalYear => vec!["tropical year"],
            Atom::Volt         => vec!["volt"],
            Atom::Watt         => vec!["watt"],
            Atom::Weber        => vec!["weber"],
            Atom::Week         => vec!["week"],
            Atom::Year         => vec!["year"],
        }
    }

    pub fn primary_code(&self) -> &'static str {
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
            Atom::AcreUS                              => "[acr_us]",
            Atom::Ampere                              => "A",
            Atom::Are                                 => "ar",
            Atom::AstronomicUnit                      => "AU",
            Atom::AtomicMassUnit                      => "u",
            Atom::Bar                                 => "bar",
            Atom::Becquerel                           => "Bq",
            Atom::Day                                 => "d",
            Atom::Degree                              => "deg",
            Atom::DegreeCelsius                       => "Cel",
            Atom::DegreeFahrenheit                    => "[degF]",
            Atom::DegreeReaumur                       => "[degRe]",
            Atom::DegreeMinute                        => "'",
            Atom::DegreeSecond                        => "''",
            Atom::ElectronVolt                        => "eV",
            Atom::FluidOunceUS                        => "[foz_us]",
            Atom::Farad                               => "F",
            Atom::FootInternational                   => "[ft_i]",
            Atom::FootUS                              => "[ft_us]",
            Atom::GillUS                              => "[gil_us]",
            Atom::Gon                                 => "gon",
            Atom::Gray                                => "Gy",
            Atom::Henry                               => "H",
            Atom::Hertz                               => "Hz",
            Atom::Hour                                => "h",
            Atom::InchInternational                   => "[in_i]",
            Atom::Joule                               => "J",
            Atom::Liter                               => "l",
            Atom::Lumen                               => "lm",
            Atom::Lux                                 => "lx",
            Atom::MeanGregorianMonth                  => "mo_g",
            Atom::MeanGregorianYear                   => "a_g",
            Atom::MeanJulianMonth                     => "mo_j",
            Atom::MeanJulianYear                      => "a_j",
            Atom::Minute                              => "min",
            Atom::Mole                                => "mol",
            Atom::Month                               => "mo",
            Atom::Newton                              => "N",
            Atom::Ohm                                 => "Ohm",
            Atom::Parsec                              => "pc",
            Atom::Pascal                              => "Pa",
            Atom::PartsPerBillion                     => "[ppb]",
            Atom::PartsPerMillion                     => "[ppm]",
            Atom::PartsPerThousand                    => "[ppth]",
            Atom::Percent                             => "%",
            Atom::PH                                  => "[pH]",
            Atom::PintUS                              => "[pt_us]",
            Atom::PrismDiopter                        => "[p'diop]",
            Atom::QuartUS                             => "[qt_us]",
            Atom::QueenAnnesWineGallon                => "[gal_us]",
            Atom::RodUS                               => "[rd_us]",
            Atom::Siemens                             => "S",
            Atom::Sievert                             => "Sv",
            Atom::Steradian                           => "sr",
            Atom::SynodalMonth                        => "mo_s",
            Atom::Tesla                               => "T",
            Atom::TheNumberPi                         => "[pi]",
            Atom::TheNumberTenForArbitraryPowersCaret => "10^",
            Atom::TheNumberTenForArbitraryPowersStar  => "10*",
            Atom::Tonne                               => "t",
            Atom::TropicalYear                        => "a_t",
            Atom::Volt                                => "V",
            Atom::Watt                                => "W",
            Atom::Weber                               => "Wb",
            Atom::Week                                => "wk",
            Atom::Year                                => "a",
        }
    }

    pub fn print_symbol(&self) -> Option<&'static str> {
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
                Atom::Day                |
                Atom::DegreeMinute       |
                Atom::DegreeSecond       |
                Atom::ElectronVolt       |
                Atom::Farad              |
                Atom::Gray               |
                Atom::Hertz              |
                Atom::Henry              |
                Atom::Hour               |
                Atom::Joule              |
                Atom::Liter              |
                Atom::Lumen              |
                Atom::Lux                |
                Atom::MeanGregorianMonth |
                Atom::MeanGregorianYear  |
                Atom::Minute             |
                Atom::Mole               |
                Atom::Month              |
                Atom::Parsec             |
                Atom::Pascal             |
                Atom::Percent            |
                Atom::Siemens            |
                Atom::Sievert            |
                Atom::Steradian          |
                Atom::Tesla              |
                Atom::Tonne              |
                Atom::Volt               |
                Atom::Watt               |
                Atom::Weber              |
                Atom::Week               |
                Atom::Year => Some(self.primary_code()),
            Atom::Are               => Some("a"),
            Atom::Degree            => Some("°"),
            Atom::DegreeCelsius     => Some("°C"),
            Atom::DegreeFahrenheit  => Some("°F"),
            Atom::DegreeReaumur     => Some("°R"),
            Atom::FluidOunceUS      => Some("oz fl"),
            Atom::FootInternational => Some("ft"),
            Atom::FootUS            => Some("ft (us)"),
            Atom::Gon               => Some("□"),
            Atom::MeanJulianMonth   => Some("moⱼ"),
            Atom::MeanJulianYear    => Some("aⱼ"),
            Atom::Ohm               => Some("Ω"),
            Atom::PartsPerBillion   => Some("ppb"),
            Atom::PartsPerMillion   => Some("ppm"),
            Atom::PartsPerThousand  => Some("ppth"),
            Atom::PH                => Some("pH"),
            Atom::PrismDiopter      => Some("PD"),
            Atom::SynodalMonth      => Some("moₛ"),
            Atom::TheNumberPi       => Some("π"),
            Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => Some("10"),
            Atom::TropicalYear      => Some("aₜ"),
            _ => None,
        }
    }

    pub fn property(&self) -> Property {
        match *self {
            Atom::TheUnity => Property::Unclassified,
            Atom::Candela  => Property::LuminousIntensity,
            Atom::Coulomb  => Property::ElectricCharge,
            Atom::Gram               |
                Atom::AtomicMassUnit |
                Atom::Tonne => Property::Mass,
            Atom::Kelvin               |
                Atom::DegreeCelsius    |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur => Property::Temperature,
            Atom::Meter                 |
                Atom::AstronomicUnit    |
                Atom::FootInternational |
                Atom::FootUS            |
                Atom::InchInternational |
                Atom::Parsec            |
                Atom::RodUS => Property::Length,
            Atom::Radian           |
                Atom::Degree       |
                Atom::DegreeMinute |
                Atom::DegreeSecond |
                Atom::Gon => Property::PlaneAngle,
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
                Atom::Year => Property::Time,
            Atom::AcreUS |
                Atom::Are => Property::Area,
            Atom::FluidOunceUS |
                Atom::GillUS   |
                Atom::PintUS   |
                Atom::QuartUS  |
                Atom::QueenAnnesWineGallon => Property::FluidVolume,
            Atom::Liter                    => Property::Volume,
            Atom::Mole                     => Property::AmountOfSubstance,
            Atom::PartsPerBillion     |
                Atom::PartsPerMillion |
                Atom::PartsPerThousand => Property::Fraction,
            Atom::Percent                                 |
                Atom::TheNumberPi                         |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => Property::Number,
            Atom::PH                                     => Property::Acidity,
            Atom::PrismDiopter                           => Property::RefractionOfPrism,
            Atom::Steradian                              => Property::SolidAngle,
            Atom::Hertz                                  => Property::Frequency,
            Atom::Newton                                 => Property::Force,
            Atom::Pascal                                 |
                Atom::Bar   => Property::Pressure,
            Atom::Joule     |
                Atom::ElectronVolt => Property::Energy,
            Atom::Watt             => Property::Power,
            Atom::Ampere           => Property::ElectricCurrent,
            Atom::Volt             => Property::ElectricPotential,
            Atom::Farad            => Property::ElectricCapacitance,
            Atom::Ohm              => Property::ElectricResistance,
            Atom::Siemens          => Property::ElectricConductance,
            Atom::Weber            => Property::MagneticFlux,
            Atom::Tesla            => Property::MagneticFluxDensity,
            Atom::Henry            => Property::Inductance,
            Atom::Lumen            => Property::LuminousFlux,
            Atom::Lux              => Property::Illuminance,
            Atom::Becquerel        => Property::Radioactivity,
            Atom::Gray             => Property::EnergyDose,
            Atom::Sievert          => Property::DoseEquivalent,
        }
    }

    pub fn secondary_code(&self) -> &'static str {
        match *self {
            Atom::TheUnity => "1",
            Atom::Candela  => "CD",
            Atom::Coulomb  => "C",
            Atom::Gram     => "G",
            Atom::Meter    => "M",
            Atom::Radian   => "RAD",
            Atom::Second   => "S",

            // Derived units
            Atom::AcreUS               => "[ACR_US]",
            Atom::Are                  => "AR",
            Atom::AstronomicUnit       => "AMU",
            Atom::AtomicMassUnit       => "AMU",
            Atom::Bar                  => "BAR",
            Atom::Becquerel            => "BQ",
            Atom::Day                  => "D",
            Atom::Degree               => "DEG",
            Atom::DegreeCelsius        => "CEL",
            Atom::DegreeFahrenheit     => "[DEGF]",
            Atom::ElectronVolt         => "EV",
            Atom::FluidOunceUS         => "[FOZ_US]",
            Atom::FootInternational    => "[FT_I]",
            Atom::FootUS               => "[FT_US]",
            Atom::GillUS               => "[GIL_US]",
            Atom::Gon                  => "GON",
            Atom::Gray                 => "GY",
            Atom::Hertz                => "HZ",
            Atom::Hour                 => "HR",
            Atom::InchInternational    => "[IN_I]",
            Atom::Liter                => "L",
            Atom::Lumen                => "LM",
            Atom::Lux                  => "LX",
            Atom::MeanGregorianMonth   => "MO_G",
            Atom::MeanGregorianYear    => "ANN_G",
            Atom::MeanJulianMonth      => "MO_J",
            Atom::MeanJulianYear       => "ANN_J",
            Atom::Minute               => "MIN",
            Atom::Mole                 => "MOL",
            Atom::Month                => "MO",
            Atom::Ohm                  => "OHM",
            Atom::Parsec               => "PRS",
            Atom::Pascal               => "PAL",
            Atom::PartsPerBillion      => "[PPB]",
            Atom::PartsPerMillion      => "[PPM]",
            Atom::PartsPerThousand     => "[PPTH]",
            Atom::PH                   => "[PH]",
            Atom::PintUS               => "[PT_US]",
            Atom::PrismDiopter         => "[P'DIOP]",
            Atom::QuartUS              => "[QT_US]",
            Atom::QueenAnnesWineGallon => "[GAL_US]",
            Atom::RodUS                => "[RD_US]",
            Atom::Siemens              => "SIE",
            Atom::Sievert              => "SV",
            Atom::Steradian            => "SR",
            Atom::SynodalMonth         => "MO_S",
            Atom::TheNumberPi          => "[PI]",
            Atom::Tonne                => "TNE",
            Atom::TropicalYear         => "ANN_T",
            Atom::Weber                => "WB",
            Atom::Week                 => "WK",
            Atom::Year                 => "ANN",
            Atom::Ampere                                  |
                Atom::DegreeReaumur                       |
                Atom::DegreeMinute                        |
                Atom::DegreeSecond                        |
                Atom::Farad                               |
                Atom::Henry                               |
                Atom::Joule                               |
                Atom::Kelvin                              |
                Atom::Newton                              |
                Atom::Percent                             |
                Atom::Tesla                               |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar  |
                Atom::Volt                                |
                Atom::Watt => self.primary_code(),
        }
    }

    pub fn scalar(&self) -> f64 {
        match *self {
            Atom::TheUnity => 1.0,
            _ => self.calculate_scalar(1.0),
        }
    }

    pub fn magnitude(&self) -> f64 {
        match *self {
            Atom::TheUnity => 1.0,
            _ => self.calculate_magnitude(self.scalar()),
        }
    }

    pub fn calculate_scalar(&self, value: f64) -> f64 {
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

    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        match *self {
            Atom::DegreeCelsius    => value - 273.15,
            Atom::DegreeFahrenheit => 9.0 * value / 5.0 - 459.67,
            Atom::DegreeReaumur    => (value - 273.15) * 0.8,
            Atom::PH               => -value.log10(),
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
    use composition::Composition;
    use dimension::Dimension;
    use prefix::Prefix;
    use term::Term;

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
            assert_eq!(base_atom.definition().unit.terms, terms);
        }
    }

    #[test]
    fn validate_definition_acre_us() {
        let atom = Atom::AcreUS;
        let mut term = Term::new(Some(Atom::RodUS), None);
        term.exponent = 2;

        assert_eq!(atom.definition().value, 160.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_are() {
        let atom = Atom::Are;
        let mut term = Term::new(Some(Atom::Meter), None);
        term.exponent = 2;

        assert_eq!(atom.definition().value, 100.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
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
        assert_eq!(atom.definition().unit.terms, vec![term, term2, term3]);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_degree_celsius() {
        let atom = Atom::DegreeCelsius;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_degree_fahrenheit() {
        let atom = Atom::DegreeFahrenheit;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_degree_reaumur() {
        let atom = Atom::DegreeReaumur;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_fluid_ounce_us() {
        let atom = Atom::FluidOunceUS;
        let term = Term::new(Some(Atom::GillUS), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 4;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term, term2]);
    }

    #[test]
    fn validate_definition_foot_international() {
        let atom = Atom::FootInternational;
        let term = Term::new(Some(Atom::InchInternational), None);

        assert_eq!(atom.definition().value, 12.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_foot_us() {
        let atom = Atom::FootUS;
        let term1 = Term::new(Some(Atom::Meter), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 3937;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1200.0);
        assert_eq!(atom.definition().unit.terms, vec![term1, term2]);
    }

    #[test]
    fn validate_definition_gill_us() {
        let atom = Atom::GillUS;
        let term = Term::new(Some(Atom::PintUS), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 4;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term, term2]);
    }

    #[test]
    fn validate_definition_inch_international() {
        let atom = Atom::InchInternational;
        let term = Term::new(Some(Atom::Meter), Some(Prefix::Centi));

        assert_eq!(atom.definition().value, 254e-2);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_liter() {
        let atom = Atom::Liter;
        let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Deci));
        term.exponent = 3;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_mole() {
        let atom = Atom::Mole;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = 23;

        assert_eq!(atom.definition().value, 6.022_136_7);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_parts_per_billion() {
        let atom = Atom::PartsPerBillion;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = -9;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_parts_per_million() {
        let atom = Atom::PartsPerMillion;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = -6;

        let terms = vec![term];

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, terms);
    }

    #[test]
    fn validate_definition_parts_per_thousand() {
        let atom = Atom::PartsPerThousand;
        let mut term = Term::new(Some(Atom::TheNumberTenForArbitraryPowersStar), None);
        term.exponent = -3;

        let terms = vec![term];

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, terms);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_ph() {
        let atom = Atom::PH;
        let term = Term::new(Some(Atom::FootUS), None);
        let terms = vec![term];

        assert_eq!(atom.definition().value, 16.6);
        assert_eq!(atom.definition().unit.terms, terms);
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
        assert_eq!(atom.definition().unit.terms, terms);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_definition_prism_diopter() {
        let atom = Atom::PrismDiopter;
        let term = Term::new(Some(Atom::QuartUS), None);
        let terms = vec![term];

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, terms);
    }

    #[test]
    fn validate_definition_quart_us() {
        let atom = Atom::QuartUS;
        let term = Term::new(Some(Atom::QueenAnnesWineGallon), None);

        let mut term2 = Term::new(None, None);
        term2.factor = 4;
        term2.exponent = -1;

        assert_eq!(atom.definition().value, 1.0);
        assert_eq!(atom.definition().unit.terms, vec![term, term2]);
    }

    #[test]
    fn validate_definition_queen_annes_wine_gallon_us() {
        let atom = Atom::QueenAnnesWineGallon;
        let mut term = Term::new(Some(Atom::InchInternational), None);
        term.exponent = 3;

        assert_eq!(atom.definition().value, 231.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_rod_us() {
        let atom = Atom::RodUS;
        let term = Term::new(Some(Atom::FootUS), None);

        assert_eq!(atom.definition().value, 16.5);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_the_number_pi() {
        let atom = Atom::TheNumberPi;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(
            atom.definition().value,
            3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3
        );
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_the_number_ten_for_arbitrary_powers_caret() {
        let atom = Atom::TheNumberTenForArbitraryPowersCaret;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 10.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_the_number_ten_for_arbitrary_powers_star() {
        let atom = Atom::TheNumberTenForArbitraryPowersCaret;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value, 10.0);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_composition() {
        let atom = Atom::Candela;
        let composition = Composition::new(Dimension::LuminousIntensity, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Coulomb;
        let composition = Composition::new(Dimension::ElectricCharge, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Gram;
        let composition = Composition::new(Dimension::Mass, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Kelvin;
        let composition = Composition::new(Dimension::Temperature, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Meter;
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Radian;
        let composition = Composition::new(Dimension::PlaneAngle, 1);
        assert_eq!(atom.composition().unwrap(), composition);

        let atom = Atom::Second;
        let composition = Composition::new(Dimension::Time, 1);
        assert_eq!(atom.composition().unwrap(), composition);
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
        assert_floats_eq(atom.scalar(), 4046.872_609_874_252);
    }

    #[test]
    fn validate_scalar_are() {
        let atom = Atom::Are;
        assert_floats_eq(atom.scalar(), 100.0);
    }

    #[test]
    fn validate_scalar_degree() {
        let atom = Atom::Degree;
        assert_floats_eq(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_degree_celsius() {
        let atom = Atom::DegreeCelsius;
        assert_floats_eq(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_degree_fahrenheit() {
        let atom = Atom::DegreeFahrenheit;
        assert_floats_eq(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_degree_reaumur() {
        let atom = Atom::DegreeReaumur;
        assert_floats_eq(atom.scalar(), 0.0174_532_925_199_432_95);
    }

    #[test]
    fn validate_scalar_fluid_ounce_us() {
        let atom = Atom::FluidOunceUS;
        assert_floats_eq(atom.scalar(), 2.95735295625e-05);
    }

    #[test]
    fn validate_scalar_foot_international() {
        let atom = Atom::FootInternational;
        assert_floats_eq(atom.scalar(), 0.3048);
    }

    #[test]
    fn validate_scalar_foot_us() {
        let atom = Atom::FootUS;
        assert_floats_eq(atom.scalar(), 0.304_800_609_601_219_2);
    }

    #[test]
    fn validate_scalar_gill_us() {
        let atom = Atom::GillUS;
        assert_floats_eq(atom.scalar(), 0.000_118_294_118_25);
    }

    #[test]
    fn validate_scalar_inch_international() {
        let atom = Atom::InchInternational;
        assert_floats_eq(atom.scalar(), 0.0254);
    }

    #[test]
    fn validate_scalar_liter() {
        let atom = Atom::Liter;
        assert_floats_eq(atom.scalar(), 0.001);
    }

    #[test]
    fn validate_scalar_mole() {
        let atom = Atom::Mole;
        assert_floats_eq(atom.scalar(), 6.0221367e+23);
    }

    #[test]
    fn validate_scalar_parts_per_billion() {
        let atom = Atom::PartsPerBillion;
        assert_floats_eq(atom.scalar(), 1.0e-09);
    }

    #[test]
    fn validate_scalar_parts_per_million() {
        let atom = Atom::PartsPerMillion;
        assert_floats_eq(atom.scalar(), 1.0e-06);
    }

    #[test]
    fn validate_scalar_parts_per_thousand() {
        let atom = Atom::PartsPerThousand;
        assert_floats_eq(atom.scalar(), 1.0e-03);
    }

    #[test]
    fn validate_scalar_percent() {
        let atom = Atom::Percent;
        assert_floats_eq(atom.scalar(), 0.01);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_ph() {
        let atom = Atom::PH;
        assert_floats_eq(atom.scalar(), 1.0e-09);
    }

    #[test]
    fn validate_scalar_pint_us() {
        let atom = Atom::PintUS;
        assert_floats_eq(atom.scalar(), 0.000_473_176_473);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_scalar_prism_diopter() {
        let atom = Atom::PrismDiopter;
        assert_floats_eq(atom.scalar(), 0.000_473_176_473);
    }

    #[test]
    fn validate_scalar_quart_us() {
        let atom = Atom::QuartUS;
        assert_floats_eq(atom.scalar(), 0.000_946_352_946);
    }

    #[test]
    fn validate_scalar_queen_annes_wine_gallon() {
        let atom = Atom::QueenAnnesWineGallon;
        assert_floats_eq(atom.scalar(), 0.003_785_411_784);
    }

    #[test]
    fn validate_scalar_rod_us() {
        let atom = Atom::RodUS;
        assert_floats_eq(atom.scalar(), 5.029_210_058_420_117);
    }

    #[test]
    fn validate_scalar_the_number_pi() {
        let atom = Atom::TheNumberPi;
        assert_floats_eq(atom.scalar(), 3.141_592_653_589_793);
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
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_are() {
        let atom = Atom::Are;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_degree() {
        let atom = Atom::Degree;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_degree_celsius() {
        let atom = Atom::DegreeCelsius;
        assert_floats_eq(atom.magnitude(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_degree_fahrenheit() {
        let atom = Atom::DegreeFahrenheit;
        assert_floats_eq(atom.magnitude(), 0.0174_532_925_199_432_95);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_degree_reaumur() {
        let atom = Atom::DegreeReaumur;
        assert_floats_eq(atom.magnitude(), 0.0174_532_925_199_432_95);
    }

    #[test]
    fn validate_magnitude_fluid_ounce_us() {
        let atom = Atom::FluidOunceUS;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_foot_international() {
        let atom = Atom::FootInternational;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_foot_us() {
        let atom = Atom::FootUS;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_gill_us() {
        let atom = Atom::GillUS;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_inch_international() {
        let atom = Atom::InchInternational;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_liter() {
        let atom = Atom::Liter;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_mole() {
        let atom = Atom::Mole;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_parts_per_billion() {
        let atom = Atom::PartsPerBillion;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_parts_per_million() {
        let atom = Atom::PartsPerMillion;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_parts_per_thousand() {
        let atom = Atom::PartsPerThousand;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_percent() {
        let atom = Atom::Percent;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_ph() {
        let atom = Atom::PH;
        assert_floats_eq(atom.magnitude(), 1.0e-09);
    }

    #[test]
    fn validate_magnitude_pint_us() {
        let atom = Atom::PintUS;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    #[ignore(reason = "Special Units")]
    fn validate_magnitude_prism_diopter() {
        let atom = Atom::PrismDiopter;
        assert_floats_eq(atom.magnitude(), 0.000_473_176_473);
    }

    #[test]
    fn validate_magnitude_quart_us() {
        let atom = Atom::QuartUS;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_queen_annes_wine_gallon() {
        let atom = Atom::QueenAnnesWineGallon;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_rod_us() {
        let atom = Atom::RodUS;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_magnitude_the_number_pi() {
        let atom = Atom::TheNumberPi;
        assert_floats_eq(atom.magnitude(), 1.0);
    }

    #[test]
    fn validate_display() {
        let atom = Atom::TheNumberPi;
        assert_eq!(&atom.to_string(), "[pi]")
    }

    // Because the precision of floats can vary, using assert_eq! with float values
    // is not recommended; clippy's recommendation is to calculate the absolute
    // value of the difference and make sure that it's under some acceptable
    // threshold.
    fn assert_floats_eq(actual: f64, expected: f64) {
        let error_threshold = ::std::f32::EPSILON as f64;
        let difference = actual - expected;

        assert!(
            difference.abs() < error_threshold,
            "Actual: {}, Expected: {}, Diff: {}",
            actual,
            expected,
            difference
        );
    }
}
