use classification::Classification;
use definition::Definition;
use property::Property;

pub trait UcumSymbol: Send + Sync + Copy + Clone {
    fn classification(&self) -> Classification;
    fn definition(&self) -> Definition;
    fn primary_code(&self) -> &'static str;
    fn print_symbol(&self) -> Option<&'static str>;
    fn property(&self) -> Property;
    fn names(&self) -> Vec<&'static str>;
    fn secondary_code(&self) -> &'static str;

    fn is_arbitrary(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_special(&self) -> bool;

    fn scalar(&self) -> f64;
    fn magnitude(&self) -> f64;
    fn calculate_scalar(&self, magnitude: f64) -> f64;
    fn calculate_magnitude(&self, scalar: f64) -> f64;
}
