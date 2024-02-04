pub trait UcumUnit {
    fn dimension(&self) -> crate::tokens::Dimension;

    fn is_metric(&self) -> bool;
    fn is_special(&self) -> bool;
    fn names(&self) -> (&'static str, Option<&'static str>);
    fn primary_code(&self) -> &'static str;
    fn print_symbol(&self) -> Option<&'static str>;

    fn scalar(&self) -> num_rational::Ratio<u128>;

    fn value(&self) -> Option<crate::tokens::UnitValue<'static>>;
}
