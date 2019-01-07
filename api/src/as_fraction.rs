/// Mainly intended for `Unit`s, allows for defining how a type should be split into numerator
/// and denominator. Can be useful for dealing with each differently.
///
pub trait AsFraction {
    type Denominator;
    type Numerator;

    fn as_fraction(&self) -> (Option<Self::Numerator>, Option<Self::Denominator>) {
        (self.numerator(), self.denominator())
    }

    fn numerator(&self) -> Option<Self::Numerator>;
    fn denominator(&self) -> Option<Self::Denominator>;
}
