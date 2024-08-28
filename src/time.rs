/// The time elapsed since a particular instance, which is always a positive number.
pub struct Time(usize);

impl Time {
    /// Returns a new time of secs seconds.
    pub fn secs(secs: usize) -> Self {
        Self(secs)
    }

    /// Returns an [usize] representing the time in seconds.
    pub fn as_secs(&self) -> usize {
        self.0
    }
}