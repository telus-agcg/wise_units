use num_rational::Ratio;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PrefixSymbol {
    pub(crate) name: &'static str,
    pub(crate) primary_code: &'static str,
    pub(crate) value: Ratio<u128>,
}

impl PrefixSymbol {
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }

    #[must_use]
    pub const fn primary_code(&self) -> &str {
        self.primary_code
    }

    #[must_use]
    pub const fn value(&self) -> &Ratio<u128> {
        &self.value
    }
}
