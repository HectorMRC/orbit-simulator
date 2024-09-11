use crate::{cartesian::Coords, Radiant};

/// An elliptic shape.
#[derive(Default)]
pub struct Ellipse {
    /// The center of the ellipse.
    pub center: Coords,
    /// The starting point of the ellipse.
    pub start: Coords,
    /// The ellipse focus points.
    pub foci: [Coords; 2],
}

impl Ellipse {
    pub fn with_center(mut self, center: Coords) -> Self {
        self.center = center;
        self
    }

    pub fn with_start(mut self, start: Coords) -> Self {
        self.start = start;
        self
    }

    pub fn with_foci(mut self, f1: Coords, f2: Coords) -> Self {
        self.foci = [f1, f2];
        self
    }

    pub fn semi_major_axis(&self) -> f64 {
        let d1 = self.start.distance(&self.foci[0]);
        let d2 = self.start.distance(&self.foci[1]);
        (d1 + d2) / 2.
    }

    pub fn semi_minor_axis(&self) -> f64 {
        let semi_major = self.semi_major_axis();
        let center = self.linear_eccentricity();

        (semi_major.powi(2) - center.powi(2)).sqrt()
    }

    pub fn center(&self) -> Coords {
        (self.foci[0] + self.foci[1]) / 2.
    }

    pub fn linear_eccentricity(&self) -> f64 {
        self.foci[0].distance(&self.foci[1]) / 2.
    }

    pub fn perimeter(&self) -> f64 {
        Radiant::TWO_PI.as_f64()
            * ((self.semi_major_axis().powi(2) + self.semi_minor_axis().powi(2)) / 2.).sqrt()
    }
}
