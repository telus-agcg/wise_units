use composition::Composition;
use classification::Classification;
use dimension::Dimension;
use measurement::Measurement;
use property::Property;
use unit_type::UnitType;

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
    Are,
    Degree,
    DegreeCelsius,
    DegreeFahrenheit,
    DegreeReaumur,
    FluidOunceUS,
    FootInternational,
    FootUS,
    GillUS,
    InchInternational,
    Liter,
    Mole,
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
    TheNumberPi,
    TheNumberTenForArbitraryPowersCaret,
    TheNumberTenForArbitraryPowersStar,
}

impl Atom {
    pub fn classification(&self) -> Classification {
        match *self {
            Atom::TheUnity |
                Atom::Candela |
                Atom::Coulomb |
                Atom::DegreeCelsius |
                Atom::Gram |
                Atom::Kelvin |
                Atom::Meter |
                Atom::Radian |
                Atom::Second |
                Atom::Mole => Classification::SI,
            Atom::AcreUS |
                Atom::FootUS |
                Atom::RodUS => Classification::USLengths,
            Atom::Are |
                Atom::Degree |
                Atom::Liter => Classification::ISO1000,
            Atom::DegreeFahrenheit |
                Atom::DegreeReaumur => Classification::Heat,
            Atom::FluidOunceUS |
                Atom::GillUS |
                Atom::PintUS |
                Atom::QuartUS |
                Atom::QueenAnnesWineGallon => Classification::USVolumes,
            Atom::InchInternational |
                Atom::FootInternational => Classification::Intcust,
            Atom::PartsPerBillion |
                Atom::PartsPerMillion |
                Atom::PartsPerThousand |
                Atom::Percent |
                Atom::TheNumberPi |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => Classification::Dimless,
            Atom::PH => Classification::Chemical,
            Atom::PrismDiopter => Classification::Clinical,
        }
    }

    pub fn definition(&self) -> Measurement {
        match *self {
            Atom::TheUnity |
                Atom::Candela |
                Atom::Coulomb |
                Atom::Gram |
                Atom::Kelvin |
                Atom::Meter |
                Atom::Radian |
                Atom::Second => Measurement::new(1.0, "1"),
            Atom::AcreUS => Measurement::new(160.0, "[rd_us]2"),
            Atom::Are => Measurement::new(100.0, "m2"),
            Atom::Degree => Measurement::new(2.0, "[pi].rad/360"),
            Atom::DegreeCelsius => Measurement::new(1.0, "cel(1.0 K)"),
            Atom::DegreeFahrenheit => Measurement::new(1.0, "degf(5.0 K/9)"),
            Atom::DegreeReaumur => Measurement::new(1.0, "degre(5.0 K/4)"),
            Atom::FluidOunceUS => Measurement::new(1.0, "[gil_us]/4"),
            Atom::FootInternational => Measurement::new(12.0, "[in_i]"),
            Atom::FootUS => Measurement::new(1200.0, "m/3937"),
            Atom::GillUS => Measurement::new(1.0, "[pt_us]/4"),
            Atom::InchInternational => Measurement::new(254e-2, "cm"),
            Atom::Liter => Measurement::new(1.0, "dm3"),
            Atom::Mole => Measurement::new(6.0221367, "10*23"),
            Atom::PartsPerBillion => Measurement::new(1.0, "10*-9"),
            Atom::PartsPerMillion => Measurement::new(1.0, "10*-6"),
            Atom::PartsPerThousand => Measurement::new(1.0, "10*-3"),
            Atom::Percent => Measurement::new(1.0, "10*-2"),
            Atom::PH => Measurement::new(1.0, "ph(1.0 mol/l)"),
            Atom::PintUS => Measurement::new(1.0, "[qt_us]/2"),
            Atom::PrismDiopter => Measurement::new(1.0, "100tan(1.0 rad)"),
            Atom::QuartUS => Measurement::new(1.0, "[gal_us]/4"),
            Atom::QueenAnnesWineGallon => Measurement::new(231.0, "[in_i]3"),
            Atom::RodUS => Measurement::new(16.6, "[ft_us]"),
            Atom::TheNumberPi => Measurement::new(
                3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3,
                "1",
             ),
            Atom::TheNumberTenForArbitraryPowersCaret => Measurement::new(10.0, "1"),
            Atom::TheNumberTenForArbitraryPowersStar => Measurement::new(10.0, "1"),
        }
    }

    pub fn dimension(&self) -> Composition {
        match *self {
            Atom::TheUnity |
            Atom::Candela |
            Atom::Coulomb |
            Atom::Gram |
            Atom::Kelvin |
            Atom::Meter |
            Atom::Radian |
            Atom::Second => self.base_unit_composition(),
            _ => self.definition().unit.composition()
        }
    }

    fn base_unit_composition(&self) -> Composition {
        let mut composition = Composition::new();

        match *self {
            Atom::TheUnity => composition.insert(Dimension::None, 0),
            Atom::Candela => composition.insert(Dimension::LuminousIntensity, 1),
            Atom::Coulomb => composition.insert(Dimension::ElectricCharge, 1),
            Atom::Gram => composition.insert(Dimension::Mass, 1),
            Atom::Kelvin => composition.insert(Dimension::Temperature, 1),
            Atom::Meter => composition.insert(Dimension::Length, 1),
            Atom::Radian => composition.insert(Dimension::PlaneAngle, 1),
            Atom::Second => composition.insert(Dimension::Time, 1),
            _ => (),
        }

        composition
    }

    pub fn is_arbitrary(&self) -> bool {
        match *self {
            _ => false
        }
    }

    pub fn is_metric(&self) -> bool {
        match *self {
            Atom::TheUnity |
                Atom::Candela |
                Atom::Coulomb |
                Atom::DegreeCelsius |
                Atom::Gram |
                Atom::Kelvin |
                Atom::Meter |
                Atom::Radian |
                Atom::Second |
                Atom::Are |
                Atom::Liter |
                Atom::Mole => true,
            _ => false
        }
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Atom::DegreeCelsius |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur |
                Atom::PH |
                Atom::PrismDiopter => true,
            _ => false
        }
    }

    pub fn names(&self) -> Vec<String> {
        match *self {
            Atom::TheUnity => vec!["the unity".to_string()],
            Atom::Candela => vec!["candela".to_string()],
            Atom::Coulomb => vec!["Coulomb".to_string()],
            Atom::Gram => vec!["gram".to_string()],
            Atom::Kelvin => vec!["Kelvin".to_string()],
            Atom::Meter => vec!["meter".to_string()],
            Atom::Radian => vec!["radian".to_string()],
            Atom::Second => vec!["second".to_string()],

            // Derived units
            Atom::AcreUS => vec!["acre".to_string()],
            Atom::Are => vec!["are".to_string()],
            Atom::Degree => vec!["degree".to_string()],
            Atom::DegreeCelsius => vec!["degree Celsius".to_string()],
            Atom::DegreeFahrenheit => vec!["degree Fahrenheit".to_string()],
            Atom::DegreeReaumur => vec!["degree Réaumur".to_string()],
            Atom::FluidOunceUS => vec!["fluid once".to_string()],
            Atom::FootInternational => vec!["foot".to_string()],
            Atom::FootUS => vec!["foot".to_string()],
            Atom::GillUS => vec!["gill".to_string()],
            Atom::InchInternational => vec!["inch".to_string()],
            Atom::Liter => vec!["liter".to_string()],
            Atom::Mole => vec!["mole".to_string()],
            Atom::PartsPerBillion => vec!["parts per billion".to_string()],
            Atom::PartsPerMillion => vec!["parts per million".to_string()],
            Atom::PartsPerThousand => vec!["parts per thousand".to_string()],
            Atom::Percent => vec!["percent".to_string()],
            Atom::PH => vec!["pH".to_string()],
            Atom::PintUS => vec!["pint".to_string()],
            Atom::PrismDiopter => vec!["prism diopter".to_string()],
            Atom::QuartUS => vec!["quart".to_string()],
            Atom::QueenAnnesWineGallon => vec!["Queen Ann's wine gallon".to_string()],
            Atom::RodUS => vec!["rod".to_string()],
            Atom::TheNumberPi => vec!["the number pi".to_string()],
            Atom::TheNumberTenForArbitraryPowersCaret => vec!["the number ten for arbitrary powers".to_string()],
            Atom::TheNumberTenForArbitraryPowersStar => vec!["the number ten for arbitrary powers".to_string()],
        }
    }

    pub fn primary_code(&self) -> String {
        match *self {
            Atom::TheUnity => "1".to_string(),
            Atom::Candela => "cd".to_string(),
            Atom::Coulomb => "C".to_string(),
            Atom::Gram => "g".to_string(),
            Atom::Kelvin => "K".to_string(),
            Atom::Meter => "m".to_string(),
            Atom::Radian => "rad".to_string(),
            Atom::Second => "s".to_string(),

            // Derived units
            Atom::AcreUS => "[acr_us]".to_string(),
            Atom::Are => "ar".to_string(),
            Atom::Degree => "deg".to_string(),
            Atom::DegreeCelsius => "Cel".to_string(),
            Atom::DegreeFahrenheit => "[degF]".to_string(),
            Atom::DegreeReaumur => "[degRe]".to_string(),
            Atom::FluidOunceUS => "[foz_us]".to_string(),
            Atom::FootInternational => "[ft_i]".to_string(),
            Atom::FootUS => "[ft_us]".to_string(),
            Atom::GillUS => "[gil_us]".to_string(),
            Atom::InchInternational => "[in_i]".to_string(),
            Atom::Liter => "l".to_string(),
            Atom::Mole => "mol".to_string(),
            Atom::PartsPerBillion => "[ppb]".to_string(),
            Atom::PartsPerMillion => "[ppm]".to_string(),
            Atom::PartsPerThousand => "[ppth]".to_string(),
            Atom::Percent => "%".to_string(),
            Atom::PH => "[pH]".to_string(),
            Atom::PintUS => "[pt_us]".to_string(),
            Atom::PrismDiopter => "[p'diop]".to_string(),
            Atom::QuartUS => "[qt_us]".to_string(),
            Atom::QueenAnnesWineGallon => "[gal_us]".to_string(),
            Atom::RodUS => "[rd_us]".to_string(),
            Atom::TheNumberPi => "[pi]".to_string(),
            Atom::TheNumberTenForArbitraryPowersCaret => "10^".to_string(),
            Atom::TheNumberTenForArbitraryPowersStar => "10*".to_string(),
        }
    }

    pub fn print_symbol(&self) -> Option<String> {
        match *self {
            Atom::Candela |
                Atom::Coulomb |
                Atom::Gram |
                Atom::Kelvin |
                Atom::Meter |
                Atom::Radian |
                Atom::Second |
                Atom::Liter |
                Atom::Mole |
                Atom::Percent => Some(self.primary_code()),
            Atom::Are => Some("a".to_string()),
            Atom::Degree => Some("°".to_string()),
            Atom::DegreeCelsius => Some("°C".to_string()),
            Atom::DegreeFahrenheit => Some("°F".to_string()),
            Atom::DegreeReaumur => Some("°R".to_string()),
            Atom::FluidOunceUS => Some("oz fl".to_string()),
            Atom::FootInternational => Some("ft".to_string()),
            Atom::FootUS => Some("ft (us)".to_string()),
            Atom::PartsPerBillion => Some("ppb".to_string()),
            Atom::PartsPerMillion => Some("ppm".to_string()),
            Atom::PartsPerThousand => Some("ppth".to_string()),
            Atom::PH => Some("pH".to_string()),
            Atom::PrismDiopter => Some("PD".to_string()),
            Atom::TheNumberPi => Some("π".to_string()),
            Atom::TheNumberTenForArbitraryPowersCaret => Some("10".to_string()),
            Atom::TheNumberTenForArbitraryPowersStar => Some("10".to_string()),
            _ => None
        }
    }

    pub fn property(&self) -> Property {
        match *self {
            Atom::TheUnity => Property::Unclassified,
            Atom::Candela => Property::LuminousIntensity,
            Atom::Coulomb => Property::ElectricCharge,
            Atom::Gram => Property::Mass,
            Atom::Kelvin |
                Atom::DegreeCelsius |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur => Property::Temperature,
            Atom::Meter |
                Atom::FootInternational |
                Atom::FootUS |
                Atom::InchInternational |
                Atom::RodUS => Property::Length,
            Atom::Radian |
                Atom::Degree => Property::PlaneAngle,
            Atom::Second => Property::Time,
            Atom::AcreUS |
                Atom::Are => Property::Area,
            Atom::FluidOunceUS |
                Atom::GillUS |
                Atom::PintUS |
                Atom::QuartUS |
                Atom::QueenAnnesWineGallon => Property::FluidVolume,
            Atom::Liter => Property::Volume,
            Atom::Mole => Property::AmountOfSubstance,
            Atom::PartsPerBillion |
                Atom::PartsPerMillion |
                Atom::PartsPerThousand => Property::Fraction,
            Atom::Percent |
                Atom::TheNumberPi |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar=> Property::Number,
            Atom::PH => Property::Acidity,
            Atom::PrismDiopter => Property::RefractionOfPrism,
        }
    }

    pub fn secondary_code(&self) -> String {
        match *self {
            Atom::TheUnity => "1".to_string(),
            Atom::Candela => "CD".to_string(),
            Atom::Coulomb => "C".to_string(),
            Atom::Gram => "G".to_string(),
            Atom::Kelvin => self.primary_code(),
            Atom::Meter => "M".to_string(),
            Atom::Radian => "RAD".to_string(),
            Atom::Second => "S".to_string(),

            // Derived units
            Atom::AcreUS => "[ACR_US]".to_string(),
            Atom::Are => "AR".to_string(),
            Atom::Degree => "DEG".to_string(),
            Atom::DegreeCelsius => "CEL".to_string(),
            Atom::DegreeFahrenheit => "[DEGF]".to_string(),
            Atom::FluidOunceUS => "[FOZ_US]".to_string(),
            Atom::FootInternational => "[FT_I]".to_string(),
            Atom::FootUS => "[FT_US]".to_string(),
            Atom::GillUS => "[GIL_US]".to_string(),
            Atom::InchInternational => "[IN_I]".to_string(),
            Atom::Liter => "L".to_string(),
            Atom::Mole => "MOL".to_string(),
            Atom::PartsPerBillion => "[PPB]".to_string(),
            Atom::PartsPerMillion => "[PPM]".to_string(),
            Atom::PartsPerThousand => "[PPTH]".to_string(),
            Atom::PH => "[PH]".to_string(),
            Atom::PintUS => "[PT_US]".to_string(),
            Atom::PrismDiopter => "[P'DIOP]".to_string(),
            Atom::QuartUS => "[QT_US]".to_string(),
            Atom::QueenAnnesWineGallon => "[GAL_US]".to_string(),
            Atom::RodUS => "[RD_US]".to_string(),
            Atom::TheNumberPi => "[PI]".to_string(),
            Atom::DegreeReaumur |
                Atom::Percent |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => self.primary_code(),
        }
    }

    pub fn unit_type(&self) -> UnitType {
        match *self {
            Atom::TheUnity |
                Atom::Candela |
                Atom::Coulomb |
                Atom::Gram |
                Atom::Kelvin |
                Atom::Meter |
                Atom::Radian |
                Atom::Second => UnitType::Base,
            _ => UnitType::Derived,
        }
    }

    pub fn scalar(&self) -> f64 {
        match *self {
            Atom::TheUnity => 1.0,
            _ => self.definition().scalar(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        match *self {
            Atom::TheUnity => 1.0,
            _ => self.definition().magnitude(),
        }
    }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        match *self {
            Atom::DegreeCelsius => magnitude + 273.15,
            Atom::DegreeFahrenheit => 5.0 / 9.0 * (magnitude + 459.67),
            Atom::DegreeReaumur => (magnitude / 0.8) + 273.15,
            Atom::PH => 10.0_f64.powf(-magnitude),
            Atom::PrismDiopter => magnitude.tan() * 100.0,
            _ => self.definition().calculate_scalar(magnitude)
        }
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        match *self {
            Atom::DegreeCelsius => scalar - 273.15,
            Atom::DegreeFahrenheit => 9.0 * scalar / 5.0 - 459.67,
            Atom::DegreeReaumur => (scalar - 273.15) * 0.8,
            Atom::PH => -scalar.log10(),
            Atom::PrismDiopter => (scalar / 100.0).atan(),
            _ => self.definition().calculate_magnitude(scalar)
        }
    }
}
