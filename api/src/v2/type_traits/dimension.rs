pub trait Dimension {
    type Output: PartialEq + std::ops::Mul<i32, Output = Self::Output>;

    fn dimension(&self) -> Self::Output;
}
