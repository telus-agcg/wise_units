pub use classification::Classification;
pub use property::Property;

#[derive(Clone, Debug, PartialEq)]
pub enum AtomType {
    Base,
    Derived,
}

pub trait TAtom {
    fn classification() -> Classification;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    pub atom_type: AtomType,
    pub classification: Classification,
    pub names: &'static [&'static str],
    pub primary_code: &'static str,
    pub print_symbol: Option<&'static str>,
    pub property: Property,
    pub scale: f64,
    pub secondary_code: &'static str,
    arbitrary: bool,
    // TODO: making public for testing
    pub dim: &'static str,
    metric: bool,
    special: bool,
}

impl Atom {
    pub fn dim(&self) -> String {
        // Commenting to save from implemented composition module
        // if self.is_terminal() {
            self.property.to_string()
        // } else {
        //     self.composition_string()
        // }
    }

    pub fn is_arbitrary(&self) -> bool {
        self.arbitrary
    }

    pub fn is_base(&self) -> bool {
        match self.atom_type {
            AtomType::Base => true,
            AtomType::Derived => false
        }
    }

    pub fn is_derived(&self) -> bool {
        match self.atom_type {
            AtomType::Base => false,
            AtomType::Derived => true
        }
    }

    pub fn is_metric(&self) -> bool {
        self.metric
    }
}


pub static ATOMS: [Atom; 7] = [
    Atom {
        atom_type: AtomType::Base,
        classification: Classification::SI,
        names: &["meter"],
        primary_code: "m",
        print_symbol: Some("m"),
        property: Property::Length,
        scale: 1.0,
        secondary_code: "M",
        arbitrary: false,
        dim: "L",
        metric: true,
        special: false,
    },
    Atom {
        atom_type: AtomType::Base,
        classification: Classification::SI,
        names: &["second"],
        primary_code: "s",
        print_symbol: Some("s"),
        property: Property::Time,
        scale: 1.0,
        secondary_code: "S",
        arbitrary: false,
        dim: "T",
        metric: true,
        special: false,
    },
    Atom {
        atom_type: AtomType::Base,
        classification: Classification::SI,
        names: &["gram"],
        primary_code: "g",
        print_symbol: Some("g"),
        property: Property::Mass,
        scale: 1.0,
        secondary_code: "G",
        arbitrary: false,
        dim: "M",
        metric: true,
        special: false,
    },
    Atom {
        atom_type: AtomType::Base,
        classification: Classification::SI,
        names: &["radian"],
        primary_code: "rad",
        print_symbol: Some("rad"),
        property: Property::PlaneAngle,
        scale: 1.0,
        secondary_code: "RAD",
        arbitrary: false,
        dim: "A",
        metric: true,
        special: false,
    },
    Atom {
        atom_type: AtomType::Base,
        classification: Classification::SI,
        names: &["Kelvin"],
        primary_code: "K",
        print_symbol: Some("K"),
        property: Property::Temperature,
        scale: 1.0,
        secondary_code: "K",
        arbitrary: false,
        dim: "C",
        metric: true,
        special: false,
    },
    Atom {
        atom_type: AtomType::Base,
        classification: Classification::SI,
        names: &["Coulomb"],
        primary_code: "C",
        print_symbol: Some("C"),
        property: Property::ElectricCharge,
        scale: 1.0,
        secondary_code: "C",
        arbitrary: false,
        dim: "Q",
        metric: true,
        special: false,
    },
    Atom {
        atom_type: AtomType::Base,
        classification: Classification::SI,
        names: &["candela"],
        primary_code: "cd",
        print_symbol: Some("cd"),
        property: Property::LuminousIntensity,
        scale: 1.0,
        secondary_code: "CD",
        arbitrary: false,
        dim: "F",
        metric: true,
        special: false,
    },
    // Atom::Derived(DerivedAtom {
    //     names: &["the number ten for arbitrary powers"],
    //     print_symbol: Some("10"),
    //     primary_code: "10*",
    //     secondary_code: Some("10*"),
    //     scale: Scale::Scalar(ScalarScale {
    //         value: 10.0,
    //         unit: Unit {
    //             expression: Cow::Borrowed("1"),
    //             mode: UnitDisplayMode::PrimaryKey,
    //             // not sure if this is right...
    //             terms: None,
    //         }
    //     }),
    //     classification: Classification::Dimless,
    //     property: Property::Number,
    //     metric: false,
    //     special: false,
    //     arbitrary: false,
    // })
];
