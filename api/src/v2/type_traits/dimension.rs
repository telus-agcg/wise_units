use std::ops::Mul;

pub trait Dimension: PartialEq + Mul<i32, Output = Self> {}
