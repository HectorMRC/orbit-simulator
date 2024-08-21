use crate::{Radiant, Rotation};

use super::{CartesianPoint, Scaling, Translation};

/// A succession of [CartesianPoint]s representing an arbitrary shape.
#[derive(Default)]
pub struct Shape {
    pub points: Vec<CartesianPoint>,
}

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
    pub fn length(&self) -> f64 {
        self.center.distance(&self.start) * f64::from(self.theta)
    }

    /// Given the number of segments the [Shape] must be made of, returns the discrete
    /// representation of the arc.
    pub fn as_shape(&self, segments: usize) -> Shape {
        let theta = (f64::from(self.theta) / segments as f64).into();
        let radius = self.center.distance(&self.start);

        let mut points = Vec::with_capacity(segments + 1);
        points.push(self.start);

        for index in 1..segments {
            let mut point = Rotation::default()
                .with_axis(self.axis)
                .with_theta(theta)
                .rotate(points[index - 1]);

            if point.magnitude() != radius {
                // ensures all points are equidistant to the center point

                point = Translation::default()
                    .with_vector(-self.center)
                    .translate(point);

                point = Scaling::default()
                    .with_factor(radius / point.magnitude())
                    .scale(point);

                point = Translation::default()
                    .with_vector(self.center)
                    .translate(point);
            }

            points.push(point);
        }

        Shape { points }
    }
}
