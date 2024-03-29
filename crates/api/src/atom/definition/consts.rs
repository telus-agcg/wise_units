use std::borrow::Cow;

use crate::{atom::function_set::FunctionSet, Atom, Prefix, Term};

use super::Definition;

pub(in crate::atom) const ONE: Definition<f64> = Definition::Value(1.0);

pub(in crate::atom) const PARTS_PER_THOUSAND: Definition<f64> = Definition::ValueTerms {
    value: 1_f64,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::TheNumberTenForArbitraryPowersStar),
        exponent: Some(-3),
        annotation: None,
    }]),
};

pub(in crate::atom) const PARTS_PER_MILLION: Definition<f64> = Definition::ValueTerms {
    value: 1_f64,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::TheNumberTenForArbitraryPowersStar),
        exponent: Some(-6),
        annotation: None,
    }]),
};

pub(in crate::atom) const PARTS_PER_BILLION: Definition<f64> = Definition::ValueTerms {
    value: 1_f64,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::TheNumberTenForArbitraryPowersStar),
        exponent: Some(-9),
        annotation: None,
    }]),
};

pub(in crate::atom) const PARTS_PER_TRILLION: Definition<f64> = Definition::ValueTerms {
    value: 1_f64,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::TheNumberTenForArbitraryPowersStar),
        exponent: Some(-12),
        annotation: None,
    }]),
};

pub(in crate::atom) const MOLE: Definition<f64> = Definition::ValueTerms {
    value: 6.022_136_7_f64,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::TheNumberTenForArbitraryPowersStar),
        exponent: Some(23),
        annotation: None,
    }]),
};

pub(in crate::atom) const STERADIAN: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Radian),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const HERTZ: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Second),
        exponent: Some(-1),
        annotation: None,
    }]),
};

pub(in crate::atom) const SIEMENS: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Ohm),
        exponent: Some(-1),
        annotation: None,
    }]),
};

pub(in crate::atom) const DEGREE_CELSIUS: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Kelvin),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: |value: f64| value - 273.15,
        convert_to: |value: f64| value + 273.15,
    },
};

pub(in crate::atom) const BECQUEREL: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Second),
        exponent: Some(-1),
        annotation: None,
    }]),
};

pub(in crate::atom) const GON: Definition<f64> = Definition::ValueTerms {
    value: 0.9,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Degree),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const LITER: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Deci),
        atom: Some(Atom::Meter),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const ARE: Definition<f64> = Definition::ValueTerms {
    value: 100.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Meter),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const MINUTE: Definition<f64> = Definition::ValueTerms {
    value: 60.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Second),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const HOUR: Definition<f64> = Definition::ValueTerms {
    value: 60.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Minute),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const DAY: Definition<f64> = Definition::ValueTerms {
    value: 24.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Hour),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const TROPICAL_YEAR: Definition<f64> = Definition::ValueTerms {
    value: 365.242_19,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Day),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MEAN_JULIAN_YEAR: Definition<f64> = Definition::ValueTerms {
    value: 365.25,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Day),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MEAN_GREGORIAN_YEAR: Definition<f64> = Definition::ValueTerms {
    value: 365.2425,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Day),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const YEAR: Definition<f64> = MEAN_JULIAN_YEAR;

pub(in crate::atom) const WEEK: Definition<f64> = Definition::ValueTerms {
    value: 7.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Day),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const SYNODAL_MONTH: Definition<f64> = Definition::ValueTerms {
    value: 29.530_59,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Day),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MONTH: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::MeanJulianMonth),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const TONNE: Definition<f64> = Definition::ValueTerms {
    value: 1000.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Gram),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const BAR: Definition<f64> = Definition::ValueTerms {
    value: 100_000.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Pascal),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const UNIFIED_ATOMIC_MASS_UNIT: Definition<f64> = Definition::ValueTerms {
    value: 0.000_000_000_000_000_000_000_001_660_540_2,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Gram),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const ASTRONOMIC_UNIT: Definition<f64> = Definition::ValueTerms {
    value: 149_597.870_691,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Mega),
        atom: Some(Atom::Meter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PARSEC: Definition<f64> = Definition::ValueTerms {
    value: 30_856_780_000_000_000.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Meter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const ELEMENTARY_CHARGE: Definition<f64> = Definition::ValueTerms {
    value: 0.000_000_000_000_000_000_160_217_733,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Coulomb),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const ELECTRON_MASS: Definition<f64> = Definition::ValueTerms {
    value: 0.000_000_000_000_000_000_000_000_000_910_938_97,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Gram),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PROTON_MASS: Definition<f64> = Definition::ValueTerms {
    value: 0.000_000_000_000_000_000_000_001_672_623_1,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Gram),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const STANDARD_ATMOSPHERE: Definition<f64> = Definition::ValueTerms {
    value: 101_325.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Pascal),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const KAYSER: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Centi),
        atom: Some(Atom::Meter),
        exponent: Some(-1),
        annotation: None,
    }]),
};

pub(in crate::atom) const BIOT: Definition<f64> = Definition::ValueTerms {
    value: 10.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Ampere),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MAXWELL: Definition<f64> = Definition::ValueTerms {
    value: 0.000_000_01,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Weber),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const GAUSS: Definition<f64> = Definition::ValueTerms {
    value: 0.000_1,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Tesla),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PHOT: Definition<f64> = Definition::ValueTerms {
    value: 0.000_1,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Lux),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const CURIE: Definition<f64> = Definition::ValueTerms {
    value: 37_000_000_000.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Becquerel),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const INCH_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 2.54,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Centi),
        atom: Some(Atom::Meter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const FOOT_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 12.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const YARD_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 3.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MILE_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 5280.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const FATHOM_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 6.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const NAUTICAL_MILE_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 1852.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Meter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const SQUARE_INCH_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const SQUARE_FOOT_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootInternational),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const SQUARE_YARD_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::YardInternational),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const CUBIC_INCH_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const CUBIC_FOOT_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const CUBIC_YARD_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::YardInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const BOARD_FOOT_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 144.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const CORD_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 128.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const MIL_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 0.001,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const HAND_INTERNATIONAL: Definition<f64> = Definition::ValueTerms {
    value: 4.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const YARD_US: Definition<f64> = Definition::ValueTerms {
    value: 3.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const ROD_US: Definition<f64> = Definition::ValueTerms {
    value: 16.5,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const GUNTERS_CHAIN_US: Definition<f64> = Definition::ValueTerms {
    value: 4.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::RodUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const RAMDENS_CHAIN_US: Definition<f64> = Definition::ValueTerms {
    value: 100.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const FATHOM_US: Definition<f64> = Definition::ValueTerms {
    value: 6.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const FURLONG_US: Definition<f64> = Definition::ValueTerms {
    value: 40.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::RodUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MILE_US: Definition<f64> = Definition::ValueTerms {
    value: 8.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FurlongUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const ACRE_US: Definition<f64> = Definition::ValueTerms {
    value: 160.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::RodUS),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const SQUARE_ROD_US: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::RodUS),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const SQUARE_MILE_US: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::MileUS),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const SECTION: Definition<f64> = SQUARE_MILE_US;

pub(in crate::atom) const TOWNSHIP: Definition<f64> = Definition::ValueTerms {
    value: 36.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Section),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MIL_US: Definition<f64> = Definition::ValueTerms {
    value: 0.001,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const INCH_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 2.539_998,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Centi),
        atom: Some(Atom::Meter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const FOOT_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 12.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const ROD_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 16.5,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const GUNTERS_CHAIN_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 4.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::RodBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const FATHOM_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 6.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PACE_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 2.5,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const YARD_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 3.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MILE_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 5280.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const NAUTICLE_MILE_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 6080.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const ACRE_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 4840.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::YardBritish),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const QUEEN_ANNES_WINE_GALLON_US: Definition<f64> = Definition::ValueTerms {
    value: 231.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const BARREL_US: Definition<f64> = Definition::ValueTerms {
    value: 42.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::QueenAnnesWineGallonUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const CORD_US: Definition<f64> = Definition::ValueTerms {
    value: 128.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::FootInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const BUSHEL_US: Definition<f64> = Definition::ValueTerms {
    value: 2150.42,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const CUP_US: Definition<f64> = Definition::ValueTerms {
    value: 16.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::TablespoonUS),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METRIC_FLUID_OUNCE: Definition<f64> = Definition::ValueTerms {
    value: 30.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Milli),
        atom: Some(Atom::Liter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METRIC_CUP: Definition<f64> = Definition::ValueTerms {
    value: 240.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Milli),
        atom: Some(Atom::Liter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METRIC_TEASPOON: Definition<f64> = Definition::ValueTerms {
    value: 5.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Milli),
        atom: Some(Atom::Liter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METRIC_TABLESPOON: Definition<f64> = Definition::ValueTerms {
    value: 15.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Milli),
        atom: Some(Atom::Liter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const GALLON_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 4.54609,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Liter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PECK_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 2.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::GallonBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const BUSHEL_BRITISH: Definition<f64> = Definition::ValueTerms {
    value: 4.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::PeckBritish),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const GRAIN: Definition<f64> = Definition::ValueTerms {
    value: 64.798_91,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Milli),
        atom: Some(Atom::Gram),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const POUND_AVOIRDUPOIS: Definition<f64> = Definition::ValueTerms {
    value: 7000.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Grain),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const SHORT_HUNDREDWEIGHT_AVOIRDUPOIS: Definition<f64> =
    Definition::ValueTerms {
        value: 100.0,
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: None,
            atom: Some(Atom::PoundAvoirdupois),
            exponent: None,
            annotation: None,
        }]),
    };

pub(in crate::atom) const LONG_HUNDREDWEIGHT_AVOIRDUPOIS: Definition<f64> =
    Definition::ValueTerms {
        value: 112.0,
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: None,
            atom: Some(Atom::PoundAvoirdupois),
            exponent: None,
            annotation: None,
        }]),
    };

pub(in crate::atom) const SHORT_TON_AVOIRDUPOIS: Definition<f64> = Definition::ValueTerms {
    value: 20.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::ShortHundredweightAvoirdupois),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const LONG_TON_AVOIRDUPOIS: Definition<f64> = Definition::ValueTerms {
    value: 20.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::LongHunderdweightAvoirdupois),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const STONE_AVOIRDUPOIS: Definition<f64> = Definition::ValueTerms {
    value: 14.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::PoundAvoirdupois),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PENNYWEIGHT_TROY: Definition<f64> = Definition::ValueTerms {
    value: 24.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Grain),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const OUNCE_TROY: Definition<f64> = Definition::ValueTerms {
    value: 20.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::PennyweightTroy),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const POUND_TROY: Definition<f64> = Definition::ValueTerms {
    value: 12.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::OunceTroy),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const SCUPLE_APOTHECARIES: Definition<f64> = Definition::ValueTerms {
    value: 20.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Grain),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const DRAM_APOTHECARIES: Definition<f64> = Definition::ValueTerms {
    value: 3.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::ScrupleApothecaries),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const OUNCE_APOTHECARIES: Definition<f64> = Definition::ValueTerms {
    value: 8.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::DramApothecaries),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const POUND_APOTHECARIES: Definition<f64> = Definition::ValueTerms {
    value: 12.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::OunceApothecaries),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METRIC_OUNCE: Definition<f64> = Definition::ValueTerms {
    value: 28.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Gram),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PICA: Definition<f64> = Definition::ValueTerms {
    value: 12.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Point),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PRINTERS_POINT: Definition<f64> = Definition::ValueTerms {
    value: 0.013_837,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PRINTERS_PICA: Definition<f64> = Definition::ValueTerms {
    value: 12.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::PrintersPoint),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PIED: Definition<f64> = Definition::ValueTerms {
    value: 32.48,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Centi),
        atom: Some(Atom::Meter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const CICERO: Definition<f64> = Definition::ValueTerms {
    value: 12.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Didot),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const CALORIE_AT_15C: Definition<f64> = Definition::ValueTerms {
    value: 4.1858,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const CALORIE_AT_20C: Definition<f64> = Definition::ValueTerms {
    value: 4.1819,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MEAN_CALORIE: Definition<f64> = Definition::ValueTerms {
    value: 4.190_02,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const INTERNATIONAL_TABLE_CALORIE: Definition<f64> = Definition::ValueTerms {
    value: 4.1868,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const THERMOCHEMICAL_CALORIE: Definition<f64> = Definition::ValueTerms {
    value: 4.184,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const CALORIE: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::ThermochemicalCalorie),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const NUTRITION_LABEL_CALORIES: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::ThermochemicalCalorie),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const BRITISH_THERMAL_UNIT_AT_39F: Definition<f64> = Definition::ValueTerms {
    value: 1.059_67,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const BRITISH_THERMAL_UNIT_AT_59F: Definition<f64> = Definition::ValueTerms {
    value: 1.0548,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const BRITISH_THERMAL_UNIT_AT_60F: Definition<f64> = Definition::ValueTerms {
    value: 1.05468,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const MEAN_BRITISH_THERMAL_UNIT: Definition<f64> = Definition::ValueTerms {
    value: 1.055_87,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Joule),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const INTERNATIONAL_TABLE_BRITISH_THERMAL_UNIT: Definition<f64> =
    Definition::ValueTerms {
        value: 1.055_055_852_62,
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: Some(Prefix::Kilo),
            atom: Some(Atom::Joule),
            exponent: None,
            annotation: None,
        }]),
    };

pub(in crate::atom) const THERMOCHEMICAL_BRITISH_THERMAL_UNIT: Definition<f64> =
    Definition::ValueTerms {
        value: 1.054_35,
        terms: Cow::Borrowed(&[Term {
            factor: None,
            prefix: Some(Prefix::Kilo),
            atom: Some(Atom::Joule),
            exponent: None,
            annotation: None,
        }]),
    };

pub(in crate::atom) const BRITISH_THERMAL_UNIT: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::ThermochemicalBritishThermalUnit),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METER_OF_WATER_COLUMN: Definition<f64> = Definition::ValueTerms {
    value: 9.806_65,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Pascal),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METER_OF_MERCURY_COLUMN: Definition<f64> = Definition::ValueTerms {
    value: 133.322,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Pascal),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const PRISM_DIOPTER: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Radian),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: |value: f64| (value / 100.0).atan(),
        convert_to: |value: f64| value.tan() * 100.0,
    },
};

pub(in crate::atom) const PERCENT_OF_SLOPE: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Degree),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: |value: f64| (value / 100.0).atan(),
        convert_to: |value: f64| value.tan() * 100.0,
    },
};

pub(in crate::atom) const HOMEOPATHIC_POTENCY_OF_DECIMAL_SERIES_RETIRED: Definition<f64> =
    Definition::ValueSpecial {
        value: 1f64,
        function_set: FunctionSet {
            convert_from: |value: f64| -value.log10(),
            convert_to: |value: f64| 10_f64.powf(-value),
        },
    };

pub(in crate::atom) const HOMEOPATHIC_POTENCY_OF_CENTESIMAL_SERIES_RETIRED: Definition<f64> =
    Definition::ValueSpecial {
        value: 1f64,
        function_set: FunctionSet {
            convert_from: |value: f64| -value.ln() / 100_f64.ln(),
            convert_to: |value: f64| 100_f64.powf(-value),
        },
    };

pub(in crate::atom) const HOMEOPATHIC_POTENCY_OF_MILLESIMAL_SERIES_RETIRED: Definition<f64> =
    Definition::ValueSpecial {
        value: 1f64,
        function_set: FunctionSet {
            convert_from: |value: f64| -value.ln() / 1_000_f64.ln(),
            convert_to: |value: f64| 1_000_f64.powf(-value),
        },
    };

pub(in crate::atom) const HOMEOPATHIC_POTENCY_OF_QUINTMILLESIMAL_SERIES_RETIRED: Definition<f64> =
    Definition::ValueSpecial {
        value: 1f64,
        function_set: FunctionSet {
            convert_from: |value: f64| -value.ln() / 50_000_f64.ln(),
            convert_to: |value: f64| 50_000_f64.powf(-value),
        },
    };

pub(in crate::atom) const EQUIVALENTS: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Mole),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const OSMOLE: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Mole),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const INTERNATIONAL_UNIT_SECONDARY: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InternationalUnit),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const NEPER: Definition<f64> = Definition::ValueSpecial {
    value: 1.0,
    function_set: FunctionSet {
        convert_from: f64::ln,
        convert_to: f64::exp,
    },
};

pub(in crate::atom) const BEL: Definition<f64> = Definition::ValueSpecial {
    value: 1.0,
    function_set: FunctionSet {
        convert_from: f64::log10,
        convert_to: |value: f64| 10_f64.powf(value),
    },
};

pub(in crate::atom) const BEL_VOLT: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Volt),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: |value: f64| 2.0 * value.log10(),
        convert_to: |value: f64| 10_f64.powf(value / 2.0),
    },
};

pub(in crate::atom) const BEL_MILLIVOLT: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Milli),
        atom: Some(Atom::Volt),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: |value: f64| 2.0 * value.log10(),
        convert_to: |value: f64| 10_f64.powf(value / 2.0),
    },
};

pub(in crate::atom) const BEL_MICROVOLT: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Micro),
        atom: Some(Atom::Volt),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: |value: f64| 2.0 * value.log10(),
        convert_to: |value: f64| 10_f64.powf(value / 2.0),
    },
};

pub(in crate::atom) const BEL_10_NANOVOLT: Definition<f64> = Definition::ValueTermsSpecial {
    value: 10.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Nano),
        atom: Some(Atom::Volt),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: |value: f64| 2.0 * value.log10(),
        convert_to: |value: f64| 10_f64.powf(value / 2.0),
    },
};

pub(in crate::atom) const BEL_WATT: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Watt),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: f64::log10,
        convert_to: |value: f64| 10_f64.powf(value),
    },
};

pub(in crate::atom) const BEL_KILOWATT: Definition<f64> = Definition::ValueTermsSpecial {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Kilo),
        atom: Some(Atom::Watt),
        exponent: None,
        annotation: None,
    }]),
    function_set: FunctionSet {
        convert_from: f64::log10,
        convert_to: |value: f64| 10_f64.powf(value),
    },
};

pub(in crate::atom) const STERE: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Meter),
        exponent: Some(3),
        annotation: None,
    }]),
};

pub(in crate::atom) const ANGSTROM: Definition<f64> = Definition::ValueTerms {
    value: 0.1,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Nano),
        atom: Some(Atom::Meter),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const BARN: Definition<f64> = Definition::ValueTerms {
    value: 100.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: Some(Prefix::Femto),
        atom: Some(Atom::Meter),
        exponent: Some(2),
        annotation: None,
    }]),
};

pub(in crate::atom) const MHO: Definition<f64> = Definition::ValueTerms {
    value: 1.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Siemens),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const METRIC_CARAT: Definition<f64> = Definition::ValueTerms {
    value: 0.2,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Gram),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const SMOOT: Definition<f64> = Definition::ValueTerms {
    value: 67.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::InchInternational),
        exponent: None,
        annotation: None,
    }]),
};

pub(in crate::atom) const BIT_LOGARITHMUS_DUALIS: Definition<f64> = Definition::ValueSpecial {
    value: 1.0,
    function_set: FunctionSet {
        convert_from: f64::log2,
        convert_to: f64::exp2,
    },
};

pub(in crate::atom) const BYTE: Definition<f64> = Definition::ValueTerms {
    value: 8.0,
    terms: Cow::Borrowed(&[Term {
        factor: None,
        prefix: None,
        atom: Some(Atom::Bit),
        exponent: None,
        annotation: None,
    }]),
};
