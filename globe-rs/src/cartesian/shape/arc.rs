use crate::{
    cartesian::{
        transform::{Rotation, Translation},
        Coords,
    },
    orbit::{Orbit, GRAVITATIONAL_CONSTANT},
    system::Body,
    Distance, Frequency, Radiant, Velocity,
};

use super::{Sample, Shape};

/// An arc shape, which is simply a segment or portion of a circle's circumference.
#[derive(Default)]
pub struct Arc {
    /// The center of the circumference of the arc.
    pub center: Coords,
    /// The starting point of the arc.
    pub start: Coords,
    /// The axis about which the arc is made.
    pub axis: Coords,
    /// The angle of the arc.
    pub theta: Radiant,
}

impl Sample for Arc {
    fn sample(&self, segments: usize) -> super::Shape {
        let theta = (self.theta.as_f64() / segments as f64).into();

        let translation = Translation::default().with_vector(-self.center);
        let rotation = Rotation::default().with_axis(self.axis).with_theta(theta);

        let mut points = Vec::with_capacity(segments + 1);
        points.push(self.start);

        for index in 1..segments {
            points.push(
                points[index - 1]
                    .transform(translation)
                    .transform(rotation)
                    .transform(-translation),
            );
        }

        Shape { points }
    }
}

/// An orbit in which the orbiting body moves in a perfect circle around the central body.
impl Orbit for Arc {
    fn velocity(&self, central_body: &Body) -> Velocity {
        Velocity::meters_sec((GRAVITATIONAL_CONSTANT * central_body.mass.as_kg() / self.radius().as_meters()).sqrt())
    }

    fn frequency(&self, central_body: &Body) -> Frequency {
        Frequency::hz(self.velocity(central_body).as_meters_sec() / self.perimeter().as_meters())
    }
}

impl Arc {
    pub fn with_center(mut self, center: Coords) -> Self {
        self.center = center;
        self
    }

    pub fn with_start(mut self, start: Coords) -> Self {
        self.start = start;
        self
    }

    pub fn with_axis(mut self, axis: Coords) -> Self {
        self.axis = axis;
        self
    }

    pub fn with_theta(mut self, theta: Radiant) -> Self {
        self.theta = theta;
        self
    }

    /// Returns the length of the arc.
    pub fn length(&self) -> Distance {
        Distance::km(self.center.distance(&self.start) * self.theta.as_f64())
    }

    /// Returns the perimeter of the arc's circumference.
    pub fn perimeter(&self) -> Distance {
        Distance::km(self.center.distance(&self.start) * Radiant::TWO_PI.as_f64())
    }

    /// Returns the radius of the arc.
    pub fn radius(&self) -> Distance {
        Distance::km(self.center.distance(&self.start))
    }

    /// Returns the latest [Cartesian] of the shape.
    pub fn end(&self) -> Coords {
        let translation = Translation::default().with_vector(-self.center);
        let rotation = Rotation::default()
            .with_axis(self.axis)
            .with_theta(self.theta);

        self.start
            .transform(translation)
            .transform(rotation)
            .transform(-translation)
    }
}
