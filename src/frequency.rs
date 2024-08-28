use crate::PositiveFloat;

/// The frequency at which an specific event occurs per unit of time.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Frequency(PositiveFloat);

impl Frequency {
    /// Returns a new frequency of hz hertz, which is the number of ocurrences per second.
    pub fn hz(hz: f64) -> Self {
        Self(hz.into())
    }

    /// Returns an [f64] representing the frequency in hertz.
    pub fn as_hz(&self) -> f64 {
        self.0.into()
    }
}
