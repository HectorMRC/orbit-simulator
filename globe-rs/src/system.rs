use std::time::Duration;

use alvidir::name::Name;
use serde::{Deserialize, Serialize};

use crate::{
    cartesian::{shape::Arc, Coords},
    orbit::Orbit,
    Distance, Frequency, Luminosity, Mass, Radiant,
};

/// An arbitrary spherical body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    /// The name of the body.
    pub name: Name<Self>,
    /// The radius of the body.
    pub radius: Distance,
    /// The frequency of rotation over its own axis.
    pub rotation: Frequency,
    /// The mass of the body.
    pub mass: Mass,
    /// The luminosity of the body.
    pub luminosity: Luminosity,
}

/// Describes the habitable zone around a body.
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

/// An orbital system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    /// The central body of the system.
    pub primary: Body,
    /// The distance between the surface of the primary body of this system and that of the one it
    /// orbits, if any.
    pub distance: Distance,
    /// The systems orbiting the primary body.
    pub secondary: Vec<System>,
}

impl System {
    /// Returns the state of the system in a given moment in time.
    pub fn state_at(&self, time: Duration) -> SystemState {
        SystemState::at(time, self, None)
    }

    /// Returns the radius of the system.
    pub fn radius(&self) -> Distance {
        let this_radius = self.distance + self.primary.radius;

        self.secondary
            .iter()
            .map(|system| system.radius() + this_radius)
            .max()
            .unwrap_or(this_radius)
    }

    /// Returns the frequency of the system.
    pub fn frequency(&self) -> Frequency {
        /// Returns the greatest common divisor of a and b.
        fn gcd(mut a: f64, mut b: f64) -> f64 {
            if a < b {
                (a, b) = (b, a);
            }

            while b != 0. {
                (a, b) = (b, a % b)
            }

            a
        }

        /// Returns the fraction that represents the given number.
        fn decimal_to_fraction(decimal: f64) -> (f64, f64) {
            let mut denominator = 1.;
            while (decimal * denominator).fract() != 0. {
                denominator *= 10.;
            }

            let numerator = decimal * denominator;
            let gcd = gcd(numerator, denominator);
            (numerator / gcd, denominator / gcd)
        }

        todo!()
    }
}

/// A description of an orbital system.
#[derive(Debug)]
pub struct SystemDescriptor {
    /// The time it takes for the system to orbit its parent.
    pub period: Duration,
    /// The descriptor of the systems orbiting in this one.
    pub secondary: Vec<SystemDescriptor>,
}

impl From<&System> for SystemDescriptor {
    fn from(system: &System) -> Self {
        SystemDescriptor::new(system, None)
    }
}

impl SystemDescriptor {
    fn new(system: &System, parent: Option<&System>) -> Self {
        Self {
            period: parent
                .map(|parent| {
                    let radius = system.distance + system.primary.radius + parent.primary.radius;
                    let start = Coords::default().with_y(radius.as_km());

                    let orbit = Arc::default().with_start(start);

                    1. / orbit.frequency(&parent.primary)
                })
                .unwrap_or_default(),
            secondary: system
                .secondary
                .iter()
                .map(|subsystem| SystemDescriptor::new(subsystem, Some(system)))
                .collect(),
        }
    }
}

/// An union of the [Body] type and its [Cartesian] position.
#[derive(Debug, Clone, Copy)]
struct BodyPosition<'a> {
    /// The body itself.
    body: &'a Body,
    /// The location of the body.
    position: Coords,
}

/// The configuration of a [System] in a specific moment in time.
#[derive(Debug, Default, Clone)]
pub struct SystemState {
    /// How much rotated is the primary body.
    pub rotation: Radiant,
    /// Where is located the center of the primary body.
    pub position: Coords,
    /// The state of the secondary bodies.
    pub secondary: Vec<SystemState>,
}

impl SystemState {
    fn rotation_at(time: Duration, body: &Body) -> Radiant {
        (Radiant::from(body.rotation).as_f64() * time.as_secs() as f64).into()
    }

    fn position_at(time: Duration, system: &System, parent: Option<BodyPosition>) -> Coords {
        let Some(parent) = parent else {
            return Default::default();
        };

        let radius = system.distance + system.primary.radius + parent.body.radius;
        let start = parent.position.with_y(parent.position.y() + radius.as_km());

        let orbit = Arc::default()
            .with_center(parent.position)
            .with_start(start)
            .with_axis([0., 0., 1.].into());

        let theta =
            (Radiant::from(orbit.frequency(parent.body)).as_f64() * time.as_secs() as f64).into();

        orbit.with_theta(theta).end()
    }

    fn at(time: Duration, system: &System, parent: Option<BodyPosition>) -> Self {
        let mut state = SystemState {
            rotation: Self::rotation_at(time, &system.primary),
            position: Self::position_at(time, system, parent),
            ..Default::default()
        };

        let parent = BodyPosition {
            body: &system.primary,
            position: state.position,
        };

        state.secondary = system
            .secondary
            .iter()
            .map(|system| Self::at(time, system, Some(parent)))
            .collect();

        state
    }
}

/// Iterates over time yielding the corresponding state for a given [System].  
pub struct SystemStateGenerator<'a> {
    /// The system being iterated.
    pub system: &'a System,
    /// The time-step between generations.
    pub step: Duration,
    /// The latest generation time.
    pub time: Duration,
}

impl<'a> From<&'a System> for SystemStateGenerator<'a> {
    fn from(system: &'a System) -> Self {
        Self {
            system,
            step: Duration::from_secs(1),
            time: Duration::ZERO,
        }
    }
}

impl<'a> Iterator for SystemStateGenerator<'a> {
    type Item = SystemState;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.system.state_at(self.time);
        self.time += self.step;
        Some(state)
    }
}

impl<'a> SystemStateGenerator<'a> {
    pub fn with_step(mut self, step: Duration) -> Self {
        self.step = step;
        self
    }
}
