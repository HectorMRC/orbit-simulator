use std::time::Duration;

use alvidir::name::Name;
use serde::{Deserialize, Serialize};

use crate::{Distance, Orbit};

use super::{Body, OrbitalSystemState};

/// An orbital system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalSystem<O> {
    /// The central body of the system.
    pub primary: Body,
    /// The orbit the system.
    #[serde(default)]
    pub orbit: Option<O>,
    /// The systems orbiting the primary body.
    pub secondary: Vec<OrbitalSystem<O>>,
}

impl<O: Orbit> OrbitalSystem<O> {
    /// Returns the state of the system in a given moment in time.
    pub fn state_at(&self, time: Duration) -> OrbitalSystemState {
        OrbitalSystemState::at::<O>(time, self, None)
    }

    /// Returns the radius of the system.
    pub fn radius(&self) -> Distance {
        let radius =
            self.primary.radius + self.orbit.map(|orbit| orbit.radius()).unwrap_or_default();

        self.secondary
            .iter()
            .map(|system| system.radius() + radius)
            .max()
            .unwrap_or(radius)
    }

    /// Returns the system in the system which primary body has the given name.
    pub fn system<'a>(&'a self, name: &Name<Body>) -> Option<&'a OrbitalSystem<O>> {
        if &self.primary.name == name {
            return Some(self);
        }

        self.secondary.iter().find_map(|system| system.system(name))
    }
}
