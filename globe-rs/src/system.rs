use std::time::Duration;

use alvidir::name::Name;
use serde::{Deserialize, Serialize};

use crate::{
    cartesian::{transform::Translation, Coords},
    Distance, Luminosity, Mass, Orbit, Radian, Velocity, GRAVITATIONAL_CONSTANT,
};

/// The period and direction of a rotation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rotation {
    pub period: Duration,
    pub clockwise: bool,
}

/// An arbitrary spherical body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    /// The name of the body.
    pub name: Name<Self>,
    /// The radius of the body.
    pub radius: Distance,
    /// The rotation of the body over its own axis.
    pub spin: Rotation,
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

    /// The time it takes to the body to complete a rotation.
    pub fn sideral_period(&self) -> Duration {
        self.spin.period
    }
}

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

/// An orbital system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System<O> {
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
        let radius =
            self.primary.radius + self.orbit.map(|orbit| orbit.radius()).unwrap_or_default();

        self.secondary
            .iter()
            .map(|system| system.radius() + radius)
            .max()
            .unwrap_or(radius)
    }

    /// Returns the system in the system which primary body has the given name.
    pub fn system<'a>(&'a self, name: &Name<Body>) -> Option<&'a System<O>> {
        if &self.primary.name == name {
            return Some(self);
        }

        self.secondary.iter().find_map(|system| system.system(name))
    }
}

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

impl<O: Orbit> From<&System<O>> for SystemStats {
    fn from(system: &System<O>) -> Self {
        SystemStats::new(system, None)
    }
}

impl SystemStats {
    fn new<O: Orbit>(system: &System<O>, orbitee: Option<&System<O>>) -> Self {
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
    pub rotation: Radian,
    /// Where is located the center of the primary body.
    pub position: Coords,
    /// At which radiant of its orbit is localed the system.
    pub theta: Radian,
    /// At which velocity is the system moving.
    pub velocity: Velocity,
    /// The state of the secondary bodies.
    pub secondary: Vec<SystemState>,
}

impl SystemState {
    fn spin_at(mut time: Duration, body: &Body) -> Radian {
        time = Duration::from_secs_f64(time.as_secs_f64() % body.spin.period.as_secs_f64());

        (Radian::from(body.spin.period).as_f64() * time.as_secs() as f64).into()
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

    fn theta_at<O: Orbit>(
        time: Duration,
        system: &System<O>,
        parent: Option<BodyPosition>,
    ) -> Radian {
        let (Some(parent), Some(orbit)) = (parent, system.orbit) else {
            return Default::default();
        };

        orbit.theta_at(time, parent.body)
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
            theta: Self::theta_at::<O>(time, system, parent),
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
