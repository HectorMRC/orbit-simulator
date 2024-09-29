use std::time::Duration;

use alvidir::name::Name;

use crate::{
    cartesian::{transform::Translation, Coords},
    Radian, Velocity,
};

use super::{Body, Orbit, OrbitalSystem};

/// An union of the [Body] type and its [Cartesian] position.
#[derive(Debug, Clone, Copy)]
pub struct BodyPosition<'a> {
    /// The body itself.
    pub body: &'a Body,
    /// The location of the body.
    pub position: Coords,
}

/// The configuration of a [System] in a specific moment in time.
#[derive(Debug, Clone)]
pub struct OrbitalSystemState {
    /// The name of the ruling body.
    pub body: Name<Body>,
    /// How much rotated is the primary body.
    pub rotation: Radian,
    /// Where is located the center of the primary body.
    pub position: Coords,
    /// At which radiant of its orbit is localed the system.
    pub theta: Radian,
    /// At which velocity is the system moving.
    pub velocity: Velocity,
    /// The state of the secondary bodies.
    pub secondary: Vec<OrbitalSystemState>,
}

impl OrbitalSystemState {
    fn spin_at(mut time: Duration, body: &Body) -> Radian {
        time = Duration::from_secs_f64(time.as_secs_f64() % body.spin.period.as_secs_f64());

        (Radian::from(body.spin.period).as_f64() * time.as_secs() as f64).into()
    }

    fn position_at<O: Orbit>(
        time: Duration,
        system: &OrbitalSystem<O>,
        parent: Option<BodyPosition>,
    ) -> Coords {
        let (Some(parent), Some(orbit)) = (parent, system.orbit) else {
            return Default::default();
        };

        orbit
            .position_at(time, parent.body)
            .transform(Translation::default().with_vector(parent.position))
            .transform(Translation::default().with_vector(orbit.focus()))
    }

    fn theta_at<O: Orbit>(
        time: Duration,
        system: &OrbitalSystem<O>,
        parent: Option<BodyPosition>,
    ) -> Radian {
        let (Some(parent), Some(orbit)) = (parent, system.orbit) else {
            return Default::default();
        };

        orbit.theta_at(time, parent.body)
    }

    fn velocity_at<O: Orbit>(
        time: Duration,
        system: &OrbitalSystem<O>,
        parent: Option<BodyPosition>,
    ) -> Velocity {
        let (Some(parent), Some(orbit)) = (parent, system.orbit) else {
            return Default::default();
        };

        orbit.velocity_at(time, parent.body)
    }

    pub fn at<O: Orbit>(
        time: Duration,
        system: &OrbitalSystem<O>,
        parent: Option<BodyPosition>,
    ) -> Self {
        let mut state = OrbitalSystemState {
            body: system.primary.name.clone(),
            rotation: Self::spin_at(time, &system.primary),
            position: Self::position_at::<O>(time, system, parent),
            theta: Self::theta_at::<O>(time, system, parent),
            velocity: Self::velocity_at::<O>(time, system, parent),
            secondary: Default::default(),
        };

        let parent = BodyPosition {
            body: &system.primary,
            position: state.position,
        };

        state.secondary = system
            .secondary
            .iter()
            .map(|system| Self::at::<O>(time, system, Some(parent)))
            .collect();

        state
    }

    /// Returns the state of the system for which the primary body has the given name.
    pub fn state<'a>(&'a self, name: &Name<Body>) -> Option<&'a OrbitalSystemState> {
        if &self.body == name {
            return Some(self);
        }

        self.secondary
            .iter()
            .find_map(|state: &OrbitalSystemState| state.state(name))
    }
}

/// Iterates over time yielding the corresponding state for a given [System].  
pub struct OrbitalSystemStateGenerator<'a, O: Orbit> {
    /// The system being iterated.
    pub system: &'a OrbitalSystem<O>,
    /// The time-step between generations.
    pub step: Duration,
    /// The latest generation time.
    pub time: Duration,
}

impl<'a, O: Orbit> From<&'a OrbitalSystem<O>> for OrbitalSystemStateGenerator<'a, O> {
    fn from(system: &'a OrbitalSystem<O>) -> Self {
        Self {
            system,
            step: Duration::from_secs(1),
            time: Duration::ZERO,
        }
    }
}

impl<'a, O: Orbit> Iterator for OrbitalSystemStateGenerator<'a, O> {
    type Item = OrbitalSystemState;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.system.state_at(self.time);
        self.time += self.step;
        Some(state)
    }
}

impl<'a, O: Orbit> OrbitalSystemStateGenerator<'a, O> {
    pub fn with_step(mut self, step: Duration) -> Self {
        self.step = step;
        self
    }
}
