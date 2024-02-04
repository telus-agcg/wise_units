use num_rational::Ratio;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ValueFunction<'atoms> {
    pub(crate) name: &'atoms str,
    pub(crate) value: Ratio<u128>,
    pub(crate) unit: &'atoms str,
}

impl<'atoms> ValueFunction<'atoms> {
    #[must_use]
    pub const fn name(&self) -> &'atoms str {
        self.name
    }

    #[must_use]
    pub const fn value(&self) -> &Ratio<u128> {
        &self.value
    }

    #[must_use]
    pub const fn unit(&self) -> &'atoms str {
        self.unit
    }
}
