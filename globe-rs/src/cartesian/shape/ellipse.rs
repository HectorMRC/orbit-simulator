use std::{f64::consts::PI, time::Duration};

use serde::{Deserialize, Serialize};

use crate::{cartesian::Coords, Body, Distance, Orbit, Radiant, Ratio, Velocity};

use super::{Sample, Shape};

/// An ellipse.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Ellipse {
    /// The semi-major axis of the ellipse.
    pub semi_major_axis: Distance,
    /// The eccentricity of the ellipse.
    pub eccentricity: Ratio,
}

impl Sample for Ellipse {
    fn sample(&self, segments: usize) -> super::Shape {
        Shape {
            points: (0..segments)
                .into_iter()
                .map(|vertex_index| Radiant::TWO_PI / segments as f64 * vertex_index as f64)
                .map(|theta| self.position(theta))
                .collect(),
        }
    }
}

impl Orbit for Ellipse {
    /// Assumes the central body is located on the right foci of the ellipse.
    fn velocity_at(&self, theta: Radiant, orbitee: &Body) -> Velocity {
        let radius = Coords::default()
            .with_x(self.linear_eccentricity().as_meters())
            .distance(&self.position(theta));

        Velocity::meters_sec(
            (2. * orbitee.gravitational_parameter()
                * ((1. / radius) - (1. / (2. * self.semi_major_axis.as_meters()))))
            .sqrt(),
        )
    }

    fn position_at(&self, mut time: Duration, orbitee: &Body) -> Coords {
        time = Duration::from_secs_f64(time.as_secs_f64() % self.period(orbitee).as_secs_f64());

        let mean_anomaly =
            Radiant::TWO_PI.as_f64() / self.period(orbitee).as_secs_f64() * time.as_secs_f64();

        let mut eccentric_anomaly = if self.eccentricity.as_f64() < 0.8 {
            mean_anomaly
        } else {
            PI
        };

        for _ in 0..100 {
            // Calculate f(E) = E - e*sin(E) - M and its derivative f'(E) = 1 - e*cos(E)
            let f = eccentric_anomaly
                - self.eccentricity.as_f64() * eccentric_anomaly.sin()
                - mean_anomaly;

            let f_prime = 1.0 - self.eccentricity.as_f64() * eccentric_anomaly.cos();
            eccentric_anomaly = eccentric_anomaly - f / f_prime;
        }

        let true_anomaly = 2.0
            * ((1.0 + self.eccentricity.as_f64()).sqrt() * (eccentric_anomaly / 2.0).sin())
                .atan2((1.0 - self.eccentricity.as_f64()).sqrt() * (eccentric_anomaly / 2.0).cos());

        self.position(true_anomaly.into())
    }

    fn period(&self, orbitee: &Body) -> Duration {
        Duration::from_secs_f64(
            Radiant::TWO_PI.as_f64()
                * (self.semi_major_axis.as_meters().powi(3) / orbitee.gravitational_parameter())
                    .sqrt(),
        )
    }

    fn perimeter(&self) -> Distance {
        let a = self.semi_major_axis;
        let b = self.semi_minor_axis();
        let h = (a.abs_diff(b).as_meters() / (a + b).as_meters()).powi(2);

        Distance::meters(
            PI * (a + b).as_meters()
                * (1.
                    + 3. * h / (10. + (4. - 3. * h).sqrt())
                    + ((4. / PI - 14. / 11.) * h.powi(12))),
        )
    }

    fn focus(&self) -> Coords {
        Coords::default().with_x(-self.linear_eccentricity().as_meters())
    }

    fn radius(&self) -> Distance {
        self.semi_major_axis + self.linear_eccentricity()
    }
}

impl Ellipse {
    pub fn with_semi_major_axis(mut self, semi_major_axis: Distance) -> Self {
        self.semi_major_axis = semi_major_axis;
        self
    }

    pub fn with_eccentricity(mut self, eccentricity: Ratio) -> Self {
        self.eccentricity = eccentricity;
        self
    }

    /// Returns the semi minor axis (aka. b) of the allipse.
    pub fn semi_minor_axis(&self) -> Distance {
        self.semi_major_axis * (1. - self.eccentricity.as_f64().powi(2)).sqrt()
    }

    /// Returns the distance from the center of the ellipse to one of its foci.
    pub fn linear_eccentricity(&self) -> Distance {
        self.semi_major_axis * self.eccentricity.as_f64()
    }

    /// Return the position (in meters) of the given theta.
    pub fn position(&self, theta: Radiant) -> Coords {
        Coords::default()
            .with_x(self.semi_major_axis.as_meters() * theta.as_f64().cos())
            .with_y(self.semi_minor_axis().as_meters() * theta.as_f64().sin())
    }
}
