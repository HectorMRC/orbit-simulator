use crate::{Distance, Luminosity};

use super::Body;

/// Describes the habitable zone around a body.
#[derive(Debug, Default)]
pub struct HabitableZone {
    pub inner_edge: Distance,
    pub outer_edge: Distance,
}

impl From<&Body> for HabitableZone {
    fn from(body: &Body) -> Self {
        let sun_relative = body.luminosity / Luminosity::SUN;

        Self {
            inner_edge: Distance::ASTRONOMICAL_UNIT * (sun_relative.as_watts() / 1.1).sqrt(),
            outer_edge: Distance::ASTRONOMICAL_UNIT * (sun_relative.as_watts() / 0.53).sqrt(),
        }
    }
}
