/// Defines a interface for reducing `Unit`s and `Measurements`, where "reducing" is on cancelling
/// out `Unit`s that have the same dimension (ex. "[acr_us]" and "har" both have the dimension
/// "L2", so they can effectively be canceled out).
///
pub trait IntoReduced {
    type Output;

    fn into_reduced(&self) -> Self::Output;
}
