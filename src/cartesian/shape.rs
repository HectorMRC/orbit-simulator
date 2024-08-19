use super::CartesianPoint;

/// A succession of [CartesianPoint]s representing an arbitrary shape.
#[derive(Default)]
pub struct Shape {
    pub points: Vec<CartesianPoint>,        
}

impl From<Arc> for Shape {
    fn from(value: Arc) -> Self {
        todo!()
    }
}

/// An arc shape, which is simply a segment or portion of a circle's circumference.
#[derive(Default)]
pub struct Arc {
    /// The center of the circumference of the arc.
    pub center: CartesianPoint,
    /// The starting point of the arc.
    pub start: CartesianPoint,
    /// The ending point of the arc.
    pub end: CartesianPoint,
}

impl Arc {
    pub fn with_center(mut self, center: CartesianPoint) -> Self {
        self.center = center;
        self
    }

    pub fn with_start(mut self, start: CartesianPoint) -> Self {
        self.start = start;
        self
    }

    pub fn with_end(mut self, end: CartesianPoint) -> Self {
        self.end = end;
        self
    }
}