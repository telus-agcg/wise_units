use super::ValueFunction;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UnitValue<'atoms> {
    pub(crate) value: f64,
    pub(crate) unit: &'atoms str,
    pub(crate) function: Option<ValueFunction<'atoms>>,
}

impl<'atoms> UnitValue<'atoms> {
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub const fn unit(&self) -> &'atoms str {
        self.unit
    }

    #[must_use]
    pub const fn function(&self) -> &Option<ValueFunction<'atoms>> {
        &self.function
    }
}
