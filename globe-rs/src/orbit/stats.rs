use std::time::Duration;

use alvidir::name::Name;

use crate::{Distance, Velocity};

use super::{Body, HabitableZone, Orbit, OrbitalSystem};

/// The time it takes for a body to complete a "solar day" relative to another body.
#[derive(Debug)]
pub struct SynodicPeriod {
    pub relative: Name<Body>,
    pub period: Duration,
}

/// Constant stats of an orbital system.
#[derive(Debug)]
pub struct SystemStats {
    /// The name of the ruling body.
    pub body: Name<Body>,
    /// The distance from the center of the orbit to its outer most boundary.
    pub radius: Distance,
    /// The perimeter of the orbit.
    pub perimeter: Distance,
    /// The time it takes for the system to complete its orbit.
    pub orbital_period: Duration,
    /// The synodic periods of the system relative to its major systems.
    pub synodic_periods: Vec<SynodicPeriod>,
    /// The minimum velocity at which the system orbits.
    pub min_velocity: Velocity,
    /// The maximum velocity at which the system orbits.
    pub max_velocity: Velocity,
    /// The habitable zone of the system, if any.
    pub habitable_zone: HabitableZone,
    /// The descriptor of the systems orbiting in this one.
    pub secondary: Vec<SystemStats>,
}

impl<O: Orbit> From<&OrbitalSystem<O>> for SystemStats {
    fn from(system: &OrbitalSystem<O>) -> Self {
        SystemStats::new(system, None)
    }
}

impl SystemStats {
    fn new<O: Orbit>(system: &OrbitalSystem<O>, orbitee: Option<&OrbitalSystem<O>>) -> Self {
        Self {
            body: system.primary.name.clone(),
            radius: system.orbit.map(|orbit| orbit.radius()).unwrap_or_default(),
            perimeter: system
                .orbit
                .map(|orbit| orbit.perimeter())
                .unwrap_or_default(),
            orbital_period: orbitee
                .zip(system.orbit)
                .map(|(orbitee, orbit)| orbit.period(&orbitee.primary))
                .unwrap_or_default(),
            synodic_periods: Default::default(),
            min_velocity: orbitee
                .zip(system.orbit)
                .map(|(orbitee, orbit)| orbit.min_velocity(&orbitee.primary))
                .unwrap_or_default(),
            max_velocity: orbitee
                .zip(system.orbit)
                .map(|(orbitee, orbit)| orbit.max_velocity(&orbitee.primary))
                .unwrap_or_default(),
            habitable_zone: HabitableZone::from(&system.primary),
            secondary: system
                .secondary
                .iter()
                .map(|subsystem| SystemStats::new(subsystem, Some(system)))
                .collect(),
        }
    }

    /// Returns the stats in the system stats corresponding to the body with the given name.
    pub fn stats<'a>(&'a self, name: &Name<Body>) -> Option<&'a SystemStats> {
        if &self.body == name {
            return Some(self);
        }

        self.secondary.iter().find_map(|system| system.stats(name))
    }
}
