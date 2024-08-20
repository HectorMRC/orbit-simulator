use crate::Radiant;

use super::CartesianPoint;

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

    pub fn with_theta(mut self, theta: Radiant) -> Self {
        self.theta = theta;
        self
    }
}