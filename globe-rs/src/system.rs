use std::time::Duration;

use alvidir::name::Name;
use serde::{Deserialize, Serialize};

use crate::{
    cartesian::{transform::Translation, Coords}, Distance, Frequency, Luminosity, Mass, Orbit, Radiant, Velocity, GRAVITATIONAL_CONSTANT
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

impl Body {
    /// Returns the standard gravitational parameter of the body.
    pub fn gravitational_parameter(&self) -> f64 {
        GRAVITATIONAL_CONSTANT * self.mass.as_kg()
    }

    /// Returns true if, and only if, the body has a luminousity other than zero.
    pub fn is_luminous(&self) -> bool {
        self.luminosity != Luminosity::ZERO
    }
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
pub struct System<O: Orbit> {
    /// The central body of the system.
    pub primary: Body,
    /// The orbit the system.
    #[serde(default)]
    pub orbit: Option<O>,
    /// The systems orbiting the primary body.
    pub secondary: Vec<System<O>>,
}

impl<O: Orbit> System<O> {
    /// Returns the state of the system in a given moment in time.
    pub fn state_at(&self, time: Duration) -> SystemState {
        SystemState::at::<O>(time, self, None)
    }

    /// Returns the radius of the system.
    pub fn radius(&self) -> Distance {
        let radius = self.orbit.map(|orbit| orbit.radius()).unwrap_or_default();

        self.secondary
            .iter()
            .map(|system| system.radius() + radius)
            .max()
            .unwrap_or(radius)
    }

    /// Returns the orbital frequency of the system, which corresponds to the time it takes to the system to recover the same
    /// orbital state.
    pub fn state_frequency(&self) -> Frequency {
        // fn orbit_frequency(system: &System, central_body: &Body) -> Frequency {
        //     let radius = system.distance + system.primary.radius + central_body.radius;
        //     let start = Coords::default().with_y(radius.as_km());

        //     Arc::default().with_start(start).frequency(&central_body)
        // }

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

// impl From<&System> for SystemDescriptor {
//     fn from(system: &System) -> Self {
//         SystemDescriptor::new(system, None)
//     }
// }

// impl SystemDescriptor {
//     fn new(system: &System, parent: Option<&System>) -> Self {
//         Self {
//             period: parent
//                 .map(|parent| {
//                     let radius = system.distance + system.primary.radius + parent.primary.radius;
//                     let start = Coords::default().with_y(radius.as_km());

//                     let orbit = Arc::default().with_start(start);

//                     1. / orbit.frequency(&parent.primary)
//                 })
//                 .unwrap_or_default(),
//             secondary: system
//                 .secondary
//                 .iter()
//                 .map(|subsystem| SystemDescriptor::new(subsystem, Some(system)))
//                 .collect(),
//         }
//     }
// }

/// An union of the [Body] type and its [Cartesian] position.
#[derive(Debug, Clone, Copy)]
struct BodyPosition<'a> {
    /// The body itself.
    pub body: &'a Body,
    /// The location of the body.
    pub position: Coords,
}

/// The configuration of a [System] in a specific moment in time.
#[derive(Debug, Default, Clone)]
pub struct SystemState {
    /// How much rotated is the primary body.
    pub rotation: Radiant,
    /// Where is located the center of the primary body.
    pub position: Coords,
    /// At which velocity is the system moving.
    pub velocity: Velocity,
    /// The state of the secondary bodies.
    pub secondary: Vec<SystemState>,
}

impl SystemState {
    fn spin_at(mut time: Duration, body: &Body) -> Radiant {
        time = Duration::from_secs_f64(time.as_secs_f64() % (1. / body.rotation).as_secs_f64());

        (Radiant::from(body.rotation).as_f64() * time.as_secs() as f64).into()
    }

    fn position_at<O: Orbit>(
        time: Duration,
        system: &System<O>,
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

    fn velocity_at<O: Orbit>(
        time: Duration,
        system: &System<O>,
        parent: Option<BodyPosition>,
    ) -> Velocity {
        let (Some(parent), Some(orbit)) = (parent, system.orbit) else {
            return Default::default();
        };

        orbit.velocity_at(time, parent.body)
    }

    fn at<O: Orbit>(time: Duration, system: &System<O>, parent: Option<BodyPosition>) -> Self {
        let mut state = SystemState {
            rotation: Self::spin_at(time, &system.primary),
            position: Self::position_at::<O>(time, system, parent),
            velocity: Self::velocity_at::<O>(time, system, parent),
            ..Default::default()
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
}

/// Iterates over time yielding the corresponding state for a given [System].  
pub struct SystemStateGenerator<'a, O: Orbit> {
    /// The system being iterated.
    pub system: &'a System<O>,
    /// The time-step between generations.
    pub step: Duration,
    /// The latest generation time.
    pub time: Duration,
}

impl<'a, O: Orbit> From<&'a System<O>> for SystemStateGenerator<'a, O> {
    fn from(system: &'a System<O>) -> Self {
        Self {
            system,
            step: Duration::from_secs(1),
            time: Duration::ZERO,
        }
    }
}

impl<'a, O: Orbit> Iterator for SystemStateGenerator<'a, O> {
    type Item = SystemState;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.system.state_at(self.time);
        self.time += self.step;
        Some(state)
    }
}

impl<'a, O: Orbit> SystemStateGenerator<'a, O> {
    pub fn with_step(mut self, step: Duration) -> Self {
        self.step = step;
        self
    }
}
