use std::time::Duration;

use crate::{
    cartesian::{shape::Arc, Cartesian},
    Distance, Frequency, Mass, Radiant, Velocity,
};

/// The gravitational constant as N⋅m^2⋅kg^−2.
pub const G: f64 = 6.67430e-11;

/// The orbit of an object around a central body.
pub trait Orbit {
    /// The orbital velocity of the object.
    fn velocity(&self, central_body: Body) -> Velocity;
    /// The orbit's frequency.
    fn frequency(&self, central_body: Body) -> Frequency;
}

/// An orbit in which the orbiting body moves in a perfect circle around the central body.
impl Orbit for Arc {
    fn velocity(&self, central_body: Body) -> Velocity {
        Velocity::meters_sec((G * central_body.mass.as_kg() / self.radius().as_meters()).sqrt())
    }

    fn frequency(&self, central_body: Body) -> Frequency {
        Frequency::hz(self.velocity(central_body).as_meters_sec() / self.perimeter().as_meters())
    }
}

/// An arbitrary spherical body.
#[derive(Debug, Default, Clone, Copy)]
pub struct Body {
    /// The radius of the body.
    pub radius: Distance,
    /// The frequency of rotation over its own axis.
    pub rotation: Frequency,
    /// The mass of the body.
    pub mass: Mass,
}

/// An orbital system.
#[derive(Debug, Default, Clone)]
pub struct System {
    /// The central body of the system.
    pub primary: Body,
    /// The distance between the surface of the primary body of this system and that of the one it
    /// orbits, if any.
    pub distance: Distance,
    /// The systems orbiting the primary body.
    pub secondary: Vec<System>,
}

/// An union of the [Body] type and its [Cartesian] position.
#[derive(Debug, Default, Clone, Copy)]
struct BodyPosition {
    /// The body itself.
    body: Body,
    /// The location of the body.
    position: Cartesian,
}

/// The configuration of a [System] in a specific moment in time.
#[derive(Debug, Default, Clone)]
pub struct SystemState {
    /// How much rotated is the primary body.
    pub rotation: Radiant,
    /// Where is located the center of the primary body.
    pub position: Cartesian,
    /// The state of the secondary bodies.
    pub secondary: Vec<SystemState>,
}

impl SystemState {
    pub fn with_rotation(mut self, rotation: Radiant) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_position(mut self, position: Cartesian) -> Self {
        self.position = position;
        self
    }

    pub fn with_secondary(mut self, secondary: Vec<SystemState>) -> Self {
        self.secondary = secondary;
        self
    }

    fn rotation_at(time: Duration, body: Body) -> Radiant {
        (Radiant::from(body.rotation).as_f64() * time.as_secs() as f64).into()
    }

    fn position_at(time: Duration, system: &System, parent: Option<BodyPosition>) -> Cartesian {
        let Some(parent) = parent else {
            return Default::default();
        };

        let radius =
            system.distance.as_km() + system.primary.radius.as_km() + parent.body.radius.as_km();

        let orbit = Arc::default()
            .with_center(parent.position)
            .with_start([0., radius, 0.].into())
            .with_axis([0., 0., 1.].into());

        let theta =
            (Radiant::from(orbit.frequency(parent.body)).as_f64() * time.as_secs() as f64).into();

        orbit.with_theta(theta).end()
    }

    fn state_at(time: Duration, system: &System, parent: Option<BodyPosition>) -> Self {
        let state = SystemState::default()
            .with_rotation(Self::rotation_at(time, system.primary))
            .with_position(Self::position_at(time, system, parent));

        let parent = BodyPosition {
            body: system.primary,
            position: state.position,
        };

        state.with_secondary(
            system
                .secondary
                .iter()
                .map(|system| SystemState::state_at(time, system, Some(parent)))
                .collect(),
        )
    }
}

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
        let state = SystemState::state_at(self.time, self.system, None);
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
