use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    cartesian::{transform::Rotation, Coords},
    orbit::{Orbit, GRAVITATIONAL_CONSTANT},
    system::Body,
    Distance, Radiant, Velocity,
};

use super::{Sample, Shape};

/// A circumference.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Circle {
    /// The radius of the circle.
    pub radius: Distance,
}

impl Sample for Circle {
    fn sample(&self, segments: usize) -> super::Shape {
        Shape {
            points: (0..segments)
                .into_iter()
                .map(|index| {
                    let theta = (Radiant::TWO_PI.as_f64() * index as f64 / segments as f64).into();
                    let rotation = Rotation::default()
                        .with_axis(Coords::default().with_z(1.))
                        .with_theta(theta);
                    Coords::default()
                        .with_x(self.radius.as_meters())
                        .transform(rotation)
                })
                .collect(),
        }
    }
}

/// An orbit in which the orbiting body moves in a perfect circle around the central body.
impl Orbit for Circle {
    fn velocity_at(&self, _: Radiant, orbitee: &Body) -> Velocity {
        Velocity::meters_sec(
            (GRAVITATIONAL_CONSTANT * orbitee.mass.as_kg() / self.radius.as_meters()).sqrt(),
        )
    }

    fn position_at(&self, mut time: Duration, orbitee: &Body) -> Coords {
        let period = self.period(orbitee);
        time = Duration::from_secs_f64(time.as_secs_f64() % period.as_secs_f64());

        let theta = Radiant::TWO_PI / period.as_secs_f64() * time.as_secs_f64();
        let rotation = Rotation::default()
            .with_axis(Coords::default().with_z(1.))
            .with_theta(theta);

        Coords::default()
            .with_x(self.radius.as_meters())
            .transform(rotation)
    }

    fn period(&self, orbitee: &Body) -> Duration {
        Duration::from_secs_f64(
            Radiant::TWO_PI.as_f64()
                * (self.radius.as_meters().powi(3) / orbitee.gravitational_parameter()).sqrt(),
        )
    }

    fn perimeter(&self) -> Distance {
        self.radius * Radiant::TWO_PI.as_f64()
    }

    fn focus(&self) -> Coords {
        Coords::default()
    }

    fn radius(&self) -> Distance {
        self.radius
    }
}

impl Circle {
    pub fn with_radius(mut self, radius: Distance) -> Self {
        self.radius = radius;
        self
    }

    /// Returns the length of the arc.
    pub fn length(&self) -> Distance {
        self.radius * Radiant::TWO_PI.as_f64()
    }
}
