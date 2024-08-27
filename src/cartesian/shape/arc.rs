use crate::{
    cartesian::{
        transform::{Rotation, Translation},
        CartesianPoint,
    }, Distance, Radiant, TWO_PI
};

use super::{Sample, Shape};

/// An arc shape, which is simply a segment or portion of a circle's circumference.
#[derive(Default)]
pub struct Arc {
    /// The center of the circumference of the arc.
    pub center: CartesianPoint,
    /// The starting point of the arc.
    pub start: CartesianPoint,
    /// The axis about which the arc is made.
    pub axis: CartesianPoint,
    /// The angle of the arc.
    pub theta: Radiant,
}

impl Sample for Arc {
    fn sample(&self, segments: usize) -> super::Shape {
        let theta = (f64::from(self.theta) / segments as f64).into();

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

impl Arc {
    pub fn with_center(mut self, center: CartesianPoint) -> Self {
        self.center = center;
        self
    }

    pub fn with_starting_point(mut self, start: CartesianPoint) -> Self {
        self.start = start;
        self
    }

    pub fn with_axisaxis(mut self, axis: CartesianPoint) -> Self {
        self.axis = axis;
        self
    }

    pub fn with_theta(mut self, theta: Radiant) -> Self {
        self.theta = theta;
        self
    }

    /// Returns the length of the arc.
    pub fn length(&self) -> Distance {
        Distance::km(
            self.center.distance(&self.start) * f64::from(self.theta)
        )
    }

    /// Returns the perimeter of the arc's circumference.
    pub fn perimeter(&self) -> Distance {
        Distance::km(
            self.center.distance(&self.start) * TWO_PI
        )
    }

    /// Returns the radius of the arc.
    pub fn radius(&self) -> Distance {
        Distance::km(
            self.center.distance(&self.start)
        )
    }
}
