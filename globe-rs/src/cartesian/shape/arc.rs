use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    cartesian::{transform::Rotation, Coords},
    orbit::{Orbit, GRAVITATIONAL_CONSTANT},
    system::Body,
    Distance, Radian, Velocity,
};

use super::{Sample, Shape};

/// A circumference.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Circle {
    /// The radius of the circle.
    pub radius: Distance,
    /// The initial radiant of the circle.
    pub initial_theta: Radian,
    /// The direction of the circle.
    pub clockwise: bool,
    /// The total radiants of the circle to sample.
    pub theta: Radian,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: Default::default(),
            initial_theta: Default::default(),
            clockwise: Default::default(),
            theta: Radian::TWO_PI,
        }
    }
}

impl Sample for Circle {
    fn with_initial_theta(mut self, theta: Radian) -> Self {
        self.initial_theta = theta;
        self
    }

    fn sample(&self, segments: usize) -> super::Shape {
        Shape {
            points: (0..segments)
                .map(|vertex_index| self.theta / segments as f64 * vertex_index as f64)
                .map(|theta| {
                    if self.clockwise {
                        self.initial_theta - theta
                    } else {
                        self.initial_theta + theta
                    }
                })
                .map(|theta| {
                    Rotation::default()
                        .with_axis(Coords::default().with_z(1.))
                        .with_theta(theta)
                })
                .map(|rotation| {
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
    fn min_velocity(&self, orbitee: &Body) -> Velocity {
        Velocity::meters_sec(
            (GRAVITATIONAL_CONSTANT * orbitee.mass.as_kg() / self.radius.as_meters()).sqrt(),
        )
    }

    fn max_velocity(&self, orbitee: &Body) -> Velocity {
        self.min_velocity(orbitee)
    }

    fn velocity_at(&self, _: Duration, orbitee: &Body) -> Velocity {
        self.min_velocity(orbitee)
    }

    fn position_at(&self, time: Duration, orbitee: &Body) -> Coords {
        let theta = self.theta_at(time, orbitee);
        let rotation = Rotation::default()
            .with_axis(Coords::default().with_z(1.))
            .with_theta(theta);

        Coords::default()
            .with_x(self.radius.as_meters())
            .transform(rotation)
    }

    fn theta_at(&self, mut time: Duration, orbitee: &Body) -> Radian {
        let period = self.period(orbitee);
        time = Duration::from_secs_f64(time.as_secs_f64() % period.as_secs_f64());

        Radian::TWO_PI / period.as_secs_f64() * time.as_secs_f64()
    }

    fn period(&self, orbitee: &Body) -> Duration {
        Duration::from_secs_f64(
            Radian::TWO_PI.as_f64()
                * (self.radius.as_meters().powi(3) / orbitee.gravitational_parameter()).sqrt(),
        )
    }

    fn perimeter(&self) -> Distance {
        self.radius * Radian::TWO_PI.as_f64()
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
        self.radius * Radian::TWO_PI.as_f64()
    }
}
