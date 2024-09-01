use std::{collections::VecDeque, time::Duration};

use serde::{Deserialize, Serialize};
use alvidir::name::Name;

use crate::{
    cartesian::{shape::Arc, Coords},
    orbit::Orbit,
    Distance, Frequency, Mass, Radiant,
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
}

/// An orbital system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    /// The central body of the system.
    pub primary: Body,
    /// The distance between the center of the primary body of this system and that of the one it
    /// orbits, if any.
    pub distance: Distance,
    /// The systems orbiting the primary body.
    pub secondary: Vec<System>,
}

impl<'a> IntoIterator for &'a System {
    type Item = &'a System;
    type IntoIter = SystemIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SystemIter {
            next: Some(self),
            ..Default::default()
        }
    }
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
}

/// Iterates over all the systems conforming the given system.
#[derive(Debug, Default)]
pub struct SystemIter<'a> {
    next: Option<&'a System>,
    pending: VecDeque<&'a System>,
}

impl<'a> Iterator for SystemIter<'a> {
    type Item = &'a System;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;
        self.pending.extend(next.secondary.iter());
        self.next = self.pending.pop_front();

        Some(next)
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

impl<'a> IntoIterator for &'a SystemState {
    type Item = &'a SystemState;
    type IntoIter = SystemStateIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SystemStateIter {
            next: Some(self),
            ..Default::default()
        }
    }
}

impl SystemState {
    fn rotation_at(time: Duration, body: &Body) -> Radiant {
        (Radiant::from(body.rotation).as_f64() * time.as_secs() as f64).into()
    }

    fn position_at(time: Duration, system: &System, parent: Option<BodyPosition>) -> Coords {
        let Some(parent) = parent else {
            return Default::default();
        };

        let orbit = Arc::default()
            .with_center(parent.position)
            .with_start([0., system.distance.as_km(), 0.].into())
            .with_axis([0., 0., 1.].into());

        let theta =
            (Radiant::from(orbit.frequency(parent.body)).as_f64() * time.as_secs() as f64).into();

        orbit.with_theta(theta).end()
    }

    fn at(time: Duration, system: &System, parent: Option<BodyPosition>) -> Self {
        let mut state = SystemState::default();
        state.rotation = Self::rotation_at(time, &system.primary);
        state.position = Self::position_at(time, system, parent);

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

/// Iterates over all the system states conforming the given state.
#[derive(Debug, Default)]
pub struct SystemStateIter<'a> {
    next: Option<&'a SystemState>,
    pending: VecDeque<&'a SystemState>,
}

impl<'a> Iterator for SystemStateIter<'a> {
    type Item = &'a SystemState;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;
        self.pending.extend(next.secondary.iter());
        self.next = self.pending.pop_front();

        Some(next)
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
