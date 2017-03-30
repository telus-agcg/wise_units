mod atom_type;
mod candela;
mod coulomb;
mod gram;
mod kelvin;
mod meter;
mod radian;
mod second;

pub use atom::atom_type::AtomType;
pub use atom::candela::Candela;
pub use atom::coulomb::Coulomb;
pub use atom::gram::Gram;
pub use atom::kelvin::Kelvin;
pub use atom::meter::Meter;
pub use atom::radian::Radian;
pub use atom::second::Second;
pub use classification::Classification;
pub use dimension::Dimension;
pub use property::Property;

pub trait Atom {
    fn arbitrary(&self) -> bool;
    fn atom_type(&self) -> AtomType;
    fn classification(&self) -> Classification;
    fn names(&self) -> Vec<String>;
    fn primary_code(&self) -> String;
    fn print_symbol(&self) -> Option<String>;
    fn property(&self) -> Property;
    fn scale(&self) -> f64;
    fn secondary_code(&self) -> String;
    fn special(&self) -> bool;
    fn dim(&self) -> Dimension;
    fn metric(&self) -> bool;
}

// impl Atom {
//     pub fn dim(&self) -> String {
//         // Commenting to save from implemented composition module
//         // if self.is_terminal() {
//             self.property.to_string()
//         // } else {
//         //     self.composition_string()
//         // }
//     }

//     pub fn is_base(&self) -> bool {
//         match self.atom_type {
//             AtomType::Base => true,
//             AtomType::Derived => false
//         }
//     }

//     pub fn is_derived(&self) -> bool {
//         match self.atom_type {
//             AtomType::Base => false,
//             AtomType::Derived => true
//         }
//     }

//     pub fn is_metric(&self) -> bool {
//         self.metric
//     }
// }


// pub static ATOMS: [Atom; 7] = [
//     // Atom::Derived(DerivedAtom {
//     //     names: &["the number ten for arbitrary powers"],
//     //     print_symbol: Some("10"),
//     //     primary_code: "10*",
//     //     secondary_code: Some("10*"),
//     //     scale: Scale::Scalar(ScalarScale {
//     //         value: 10.0,
//     //         unit: Unit {
//     //             expression: Cow::Borrowed("1"),
//     //             mode: UnitDisplayMode::PrimaryKey,
//     //             // not sure if this is right...
//     //             terms: None,
//     //         }
//     //     }),
//     //     classification: Classification::Dimless,
//     //     property: Property::Number,
//     //     metric: false,
//     //     special: false,
//     //     arbitrary: false,
//     // })
// ];
