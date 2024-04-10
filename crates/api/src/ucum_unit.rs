pub trait UcumUnit {
    fn is_special(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_arbitrary(&self) -> bool;

    fn scalar(&self) -> f64;
    fn magnitude(&self) -> f64;
}
