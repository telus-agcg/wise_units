use composition::Composition;
use classification::Classification;
use dimension::Dimension;
use measurable::Measurable;
use definition::Definition;
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
            Atom::TheUnity          |
                Atom::Candela       |
                Atom::Coulomb       |
                Atom::DegreeCelsius |
                Atom::Gram          |
                Atom::Kelvin        |
                Atom::Meter         |
                Atom::Radian        |
                Atom::Second        |
                Atom::Mole => Classification::SI,
            Atom::AcreUS     |
                Atom::FootUS |
                Atom::RodUS => Classification::USLengths,
            Atom::Are        |
                Atom::Degree |
                Atom::Liter => Classification::ISO1000,
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
                Atom::Second => Definition::new(1.0, "1"),
            Atom::AcreUS => Definition::new(160.0, "[rd_us]2"),
            Atom::Are => Definition::new(100.0, "m2"),
            Atom::Degree => Definition::new(2.0, "[pi].rad/360"),
            Atom::DegreeCelsius => Definition::new(1.0, "cel(1.0 K)"),
            Atom::DegreeFahrenheit => Definition::new(1.0, "degf(5.0 K/9)"),
            Atom::DegreeReaumur => Definition::new(1.0, "degre(5.0 K/4)"),
            Atom::FluidOunceUS => Definition::new(1.0, "[gil_us]/4"),
            Atom::FootInternational => Definition::new(12.0, "[in_i]"),
            Atom::FootUS => Definition::new(1200.0, "m/3937"),
            Atom::GillUS => Definition::new(1.0, "[pt_us]/4"),
            Atom::InchInternational => Definition::new(254.0e-2, "cm"),
            Atom::Liter => Definition::new(1.0, "dm3"),
            Atom::Mole => Definition::new(6.0221367, "10*23"),
            Atom::PartsPerBillion => Definition::new(1.0, "10*-9"),
            Atom::PartsPerMillion => Definition::new(1.0, "10*-6"),
            Atom::PartsPerThousand => Definition::new(1.0, "10*-3"),
            Atom::Percent => Definition::new(1.0, "10*-2"),
            Atom::PH => Definition::new(1.0, "ph(1.0 mol/l)"),
            Atom::PintUS => Definition::new(1.0, "[qt_us]/2"),
            Atom::PrismDiopter => Definition::new(1.0, "100tan(1.0 rad)"),
            Atom::QuartUS => Definition::new(1.0, "[gal_us]/4"),
            Atom::QueenAnnesWineGallon => Definition::new(231.0, "[in_i]3"),
            Atom::RodUS => Definition::new(16.6, "[ft_us]"),
            Atom::TheNumberPi => Definition::new(
                3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3,
                "1",
             ),
            Atom::TheNumberTenForArbitraryPowersCaret => Definition::new(10.0, "1"),
            Atom::TheNumberTenForArbitraryPowersStar => Definition::new(10.0, "1"),
        }
    }

    pub fn composition(&self) -> Composition {
        match *self {
            Atom::TheUnity => Composition::new_unity(),
            Atom::Candela => Composition::new_from_values(Dimension::LuminousIntensity, 1),
            Atom::Coulomb => Composition::new_from_values(Dimension::ElectricCharge, 1),
            Atom::Gram => Composition::new_from_values(Dimension::Mass, 1),
            Atom::Kelvin => Composition::new_from_values(Dimension::Temperature, 1),
            Atom::Meter => Composition::new_from_values(Dimension::Length, 1),
            Atom::Radian => Composition::new_from_values(Dimension::PlaneAngle, 1),
            Atom::Second => Composition::new_from_values(Dimension::Time, 1),
            _ => self.definition().unit.composition()
        }
    }

    pub fn is_arbitrary(&self) -> bool {
        match *self {
            _ => false
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
                Atom::Are           |
                Atom::Liter         |
                Atom::Mole => true,
            _ => false
        }
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Atom::DegreeCelsius        |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur    |
                Atom::PH               |
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
            Atom::Candela     |
                Atom::Coulomb |
                Atom::Gram    |
                Atom::Kelvin  |
                Atom::Meter   |
                Atom::Radian  |
                Atom::Second  |
                Atom::Liter   |
                Atom::Mole    |
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
            Atom::Kelvin               |
                Atom::DegreeCelsius    |
                Atom::DegreeFahrenheit |
                Atom::DegreeReaumur => Property::Temperature,
            Atom::Meter                 |
                Atom::FootInternational |
                Atom::FootUS            |
                Atom::InchInternational |
                Atom::RodUS => Property::Length,
            Atom::Radian |
                Atom::Degree => Property::PlaneAngle,
            Atom::Second => Property::Time,
            Atom::AcreUS |
                Atom::Are => Property::Area,
            Atom::FluidOunceUS |
                Atom::GillUS   |
                Atom::PintUS   |
                Atom::QuartUS  |
                Atom::QueenAnnesWineGallon => Property::FluidVolume,
            Atom::Liter => Property::Volume,
            Atom::Mole => Property::AmountOfSubstance,
            Atom::PartsPerBillion     |
                Atom::PartsPerMillion |
                Atom::PartsPerThousand => Property::Fraction,
            Atom::Percent                                 |
                Atom::TheNumberPi                         |
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
            Atom::DegreeReaumur                           |
                Atom::Percent                             |
                Atom::TheNumberTenForArbitraryPowersCaret |
                Atom::TheNumberTenForArbitraryPowersStar => self.primary_code(),
        }
    }

    // TODO: is this necessary?
    pub fn unit_type(&self) -> UnitType {
        match *self {
            Atom::TheUnity    |
                Atom::Candela |
                Atom::Coulomb |
                Atom::Gram    |
                Atom::Kelvin  |
                Atom::Meter   |
                Atom::Radian  |
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

    pub fn calculate_scalar(&self, value: f64) -> f64 {
        match *self {
            Atom::DegreeCelsius => value + 273.15,
            Atom::DegreeFahrenheit => 5.0 / 9.0 * (value + 459.67),
            Atom::DegreeReaumur => (value / 0.8) + 273.15,
            Atom::PH => 10.0_f64.powf(-value),
            Atom::PrismDiopter => value.tan() * 100.0,
            _ => self.definition().calculate_scalar(value)
        }
    }

    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        match *self {
            Atom::DegreeCelsius => value - 273.15,
            Atom::DegreeFahrenheit => 9.0 * value / 5.0 - 459.67,
            Atom::DegreeReaumur => (value - 273.15) * 0.8,
            Atom::PH => -value.log10(),
            Atom::PrismDiopter => (value / 100.0).atan(),
            _ => self.definition().calculate_magnitude(value)
        }
    }
}
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
            Atom::Mole
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::SI);
        }
    }

    #[test]
    fn validate_classification_us_lengths() {
        let atoms = vec![
            Atom::AcreUS,
            Atom::FootUS,
            Atom::RodUS,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::USLengths);
        }
    }

    #[test]
    fn validate_classification_iso1000() {
        let atoms = vec![
            Atom::Are,
            Atom::Degree,
            Atom::Liter,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::ISO1000);
        }
    }

    #[test]
    fn validate_classification_heat() {
        let atoms = vec![
            Atom::DegreeFahrenheit,
            Atom::DegreeReaumur,
        ];

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
        let atoms = vec![
            Atom::InchInternational,
            Atom::FootInternational,
        ];

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
        let atoms = vec![
            Atom::PH,
        ];

        for atom in atoms {
            assert_eq!(atom.classification(), Classification::Chemical);
        }
    }

    #[test]
    fn validate_classification_clinical() {
        let atoms = vec![
            Atom::PrismDiopter,
        ];

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
            Atom::Second
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

        assert_eq!(atom.definition().value, 16.6);
        assert_eq!(atom.definition().unit.terms, vec![term]);
    }

    #[test]
    fn validate_definition_the_number_pi() {
        let atom = Atom::TheNumberPi;
        let term = Term::new(Some(Atom::TheUnity), None);

        assert_eq!(atom.definition().value,
            3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3);
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
        let composition = Composition::new_from_values(Dimension::LuminousIntensity, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Coulomb;
        let composition = Composition::new_from_values(Dimension::ElectricCharge, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Gram;
        let composition = Composition::new_from_values(Dimension::Mass, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Kelvin;
        let composition = Composition::new_from_values(Dimension::Temperature, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Meter;
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Radian;
        let composition = Composition::new_from_values(Dimension::PlaneAngle, 1);
        assert_eq!(atom.composition(), composition);

        let atom = Atom::Second;
        let composition = Composition::new_from_values(Dimension::Time, 1);
        assert_eq!(atom.composition(), composition);
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

    // Because the precision of floats can vary, using assert_eq! with float values
    // is not recommended; clippy's recommendation is to calculate the absolute
    // value of the difference and make sure that it's under some acceptable
    // threshold.
    fn assert_floats_eq(actual: f64, expected: f64) {
        let error_threshold = ::std::f32::EPSILON as f64;
        let difference = actual - expected;

        assert!(difference.abs() < error_threshold, "Actual: {}, Expected: {}, Diff: {}", actual, expected, difference);
    }
}
