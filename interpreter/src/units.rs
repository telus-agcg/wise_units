use crate::{dimension::Dimension, unit::*, Number};

// Base unit defs
pub const BU_METER: BaseUnit = BaseUnit {
    primary_code: "m",
    dimension: Dimension::Length,
};

pub const BU_GRAM: BaseUnit = BaseUnit {
    primary_code: "g",
    dimension: Dimension::Mass,
};

// Derive unit defs
pub const DU_TEN_ARBITRARY_POWERS_STAR: Unit = Unit {
    primary_code: "10*",
    value: Value {
        value: Number::Integer(10),
        unit: None,
    },
};

// Unit defs
// TODO: Pre-sort these so we can binary search?
pub static BASE_UNITS: [UnitDefinition; 2] = [METER, GRAM];

pub const METER: UnitDefinition = UnitDefinition::Base(BU_METER);
pub const GRAM: UnitDefinition = UnitDefinition::Base(BU_GRAM);

pub static DERIVED_UNITS: [UnitDefinition; 1] = [TEN_ARBITRARY_POWERS_STAR];

pub const TEN_ARBITRARY_POWERS_STAR: UnitDefinition =
    UnitDefinition::Unit(DU_TEN_ARBITRARY_POWERS_STAR);
