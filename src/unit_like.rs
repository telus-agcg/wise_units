use composition::Composition;

pub trait UnitLike {
    fn is_special(&self) -> bool;
    fn is_unity(&self) -> bool;

    fn is_compatible_with(&self, other_unit: &Self) -> bool {
        let me = self.composition();
        let other_comp = other_unit.composition();

        me == other_comp
    }


    fn scalar(&self) -> f64;
    fn magnitude(&self) -> f64;

    fn composition(&self) -> Option<Composition>;
    fn expression(&self) -> String;
    fn expression_reduced(&self) -> String;

    fn div_u32(&self, other_factor: u32) -> Self;
    fn mul_u32(&self, other_factor: u32) -> Self;
}
