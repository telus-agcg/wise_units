use crate::v2::behavior_traits::convert::ToScalar;

pub trait StaticPrefix<V>: ToScalar<V> {
    fn primary_code(&self) -> &'static str;
    fn secondary_code(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn print_symbol(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prototyping_test() {
        struct Kilo;

        impl Kilo {
            fn primary_code(&self) -> &'static str {
                "k"
            }

            fn secondary_code(&self) -> &'static str {
                "K"
            }

            fn name(&self) -> &'static str {
                "kilo"
            }

            fn print_symbol(&self) -> &'static str {
                "k"
            }
        }

        impl StaticPrefix<f64> for Kilo {
            fn primary_code(&self) -> &'static str {
                Self::primary_code(self)
            }

            fn secondary_code(&self) -> &'static str {
                Self::secondary_code(self)
            }

            fn name(&self) -> &'static str {
                Self::name(self)
            }

            fn print_symbol(&self) -> &'static str {
                Self::print_symbol(self)
            }
        }

        impl ToScalar<f64> for Kilo {
            fn to_scalar(&self) -> f64 {
                1000.0
            }
        }
    }
}
