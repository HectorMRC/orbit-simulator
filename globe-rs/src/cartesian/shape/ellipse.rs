use std::{f64::consts::PI, time::Duration};

use crate::{
    cartesian::{
        transform::{Rotation, Transform, Translation},
        Coords,
    },
    Body, Orbit, Radiant, Velocity,
};

use super::{Sample, Scale, Shape};

/// An elliptic shape.
#[derive(Clone, Copy)]
pub struct Ellipse {
    /// The starting point of the ellipse.
    pub start: Coords,
    /// The ellipse focus points.
    pub foci: [Coords; 2],
    /// The angle of the ellipse to be sampled.
    pub theta: Radiant,
}

impl Sample for Ellipse {
    fn sample(&self, segments: usize) -> super::Shape {
        let translation = Translation::default().with_vector(self.center());
        let mut ellipse = self.transform(-translation);

        let rotation_z = Rotation::default()
            .with_axis(Coords::default().with_z(1.))
            .with_theta(ellipse.theta_z());

        ellipse = ellipse.transform(-rotation_z);

        let rotation_y = Rotation::default()
            .with_axis(Coords::default().with_y(1.))
            .with_theta(ellipse.theta_y());

        ellipse = ellipse.transform(-rotation_y);

        let rotation_x = Rotation::default()
            .with_axis(Coords::default().with_x(1.))
            .with_theta(ellipse.theta_x());

        ellipse = ellipse.transform(-rotation_x);

        let initial_theta = ellipse.start.y().atan2(ellipse.start.x());
        let a = ellipse.semi_major_axis();
        let b = ellipse.semi_minor_axis();

        Shape {
            points: (0..segments)
                .into_iter()
                .map(|vertex_index| self.theta.as_f64() * vertex_index as f64 / segments as f64)
                .map(|vertex_theta| initial_theta + vertex_theta)
                .map(|theta| {
                    Coords::default()
                        .with_x(a * theta.cos())
                        .with_y(b * theta.sin())
                })
                .map(|point| {
                    point
                        .transform(rotation_x)
                        .transform(rotation_y)
                        .transform(rotation_z)
                        .transform(translation)
                })
                .collect(),
        }
    }
}

impl Orbit for Ellipse {
    fn velocity<S: Scale>(&self, central_body: &Body) -> Velocity {
        let radius = S::distance(self.start.distance(&self.foci[0]));
        let a = S::distance(self.semi_major_axis());

        Velocity::meters_sec(
            (2. * central_body.gravitational_parameter()
                * ((1. / radius.as_meters()) - (1. / (2. * a.as_meters()))))
            .sqrt(),
        )
    }

    fn period<S: Scale>(&self, central_body: &Body) -> Duration {
        Duration::from_secs_f64(
            (Radiant::TWO_PI.as_f64() / central_body.gravitational_parameter().sqrt())
                * self.semi_major_axis().powf(3. / 2.),
        )
    }

    fn orbit<S: Scale>(&self, mut time: Duration, central_body: &Body) -> Coords {
        time = Duration::from_secs_f64(
            time.as_secs_f64() % self.period::<S>(central_body).as_secs_f64(),
        );

        let translation = Translation::default().with_vector(self.center());
        let mut ellipse = self.transform(-translation);

        let rotation_z = Rotation::default()
            .with_axis(Coords::default().with_z(1.))
            .with_theta(ellipse.theta_z());

        ellipse = ellipse.transform(-rotation_z);

        let rotation_y = Rotation::default()
            .with_axis(Coords::default().with_y(1.))
            .with_theta(ellipse.theta_y());

        ellipse = ellipse.transform(-rotation_y);

        let rotation_x = Rotation::default()
            .with_axis(Coords::default().with_x(1.))
            .with_theta(ellipse.theta_x());

        ellipse = ellipse.transform(-rotation_x);

        let initial_theta = ellipse.start.y().atan2(ellipse.start.x());
        let a = self.semi_major_axis();
        let b = self.semi_minor_axis();

        let meters = ellipse.velocity::<S>(central_body).as_meters_sec() * time.as_secs_f64();
        let perimeter = S::distance(ellipse.perimeter()).as_meters();

        let mut theta = Radiant::TWO_PI.as_f64() / (perimeter / meters);
        let mut coord = Coords::default()
            .with_x(a * (initial_theta + theta).cos())
            .with_y(b * (initial_theta + theta).sin());

        let mut distance = S::distance(coord.distance(&ellipse.start));

        while distance.as_meters() > meters {
            theta = theta * meters / distance.as_meters();
            coord = Coords::default()
                .with_x(a * (initial_theta + theta).cos())
                .with_y(b * (initial_theta + theta).sin());

            distance = S::distance(coord.distance(&ellipse.start));
        }

        coord
            .transform(rotation_x)
            .transform(rotation_y)
            .transform(rotation_z)
            .transform(translation)
    }
}

impl Default for Ellipse {
    fn default() -> Self {
        Self {
            start: Default::default(),
            foci: Default::default(),
            theta: Radiant::TWO_PI,
        }
    }
}

impl Ellipse {
    /// Returns the ellipse with the given semi-major axis and eccentricity.
    pub fn new(semi_major_axis: f64, eccentricity: f64) -> Self {
        let semi_minor_axis = semi_major_axis * (1. - eccentricity.powi(2)).sqrt();
        let linear_eccentricity = (semi_major_axis.powi(2) - semi_minor_axis.powi(2)).sqrt();

        Self {
            start: Coords::default().with_x(semi_major_axis),
            foci: [
                Coords::default().with_x(-linear_eccentricity),
                Coords::default().with_x(linear_eccentricity),
            ],
            ..Default::default()
        }
    }

    pub fn with_start(mut self, start: Coords) -> Self {
        self.start = start;
        self
    }

    pub fn with_foci(mut self, f1: Coords, f2: Coords) -> Self {
        self.foci = [f1, f2];
        self
    }

    pub fn with_theta(mut self, theta: Radiant) -> Self {
        self.theta = theta;
        self
    }

    /// Returns the semi major axis (aka. a) of the ellipse.
    pub fn semi_major_axis(&self) -> f64 {
        let r1 = self.start.distance(&self.foci[0]);
        let r2 = self.start.distance(&self.foci[1]);

        (r1 + r2) / 2.
    }

    /// Returns the semi minor axis (aka. b) of the allipse.
    pub fn semi_minor_axis(&self) -> f64 {
        let r1 = self.start.distance(&self.foci[0]);
        let r2 = self.start.distance(&self.foci[1]);

        (r1 * r2).sqrt()
    }

    /// Returns the center of the ellipse.
    pub fn center(&self) -> Coords {
        (self.foci[0] + self.foci[1]) / 2.
    }

    /// Returns the eccentricity of the ellipse.
    pub fn eccentricity(&self) -> f64 {
        (1. - (self.semi_minor_axis().powi(2) / self.semi_major_axis().powi(2))).sqrt()
    }

    /// Returns the linear eccentricity of the ellipse.
    pub fn linear_eccentricity(&self) -> f64 {
        (self.semi_major_axis().powi(2) - self.semi_minor_axis().powi(2)).sqrt()
    }

    /// Returns the perimeter of the ellipse, computed using the Ramanujan II-Cantrell formula.
    pub fn perimeter(&self) -> f64 {
        let a = self.semi_major_axis();
        let b = self.semi_minor_axis();
        let h = ((a - b) / (a + b)).powi(2);

        PI * (a + b)
            * (1. + 3. * h / (10. + (4. - 3. * h).sqrt()) + ((4. / PI - 14. / 11.) * h.powi(12)))
    }

    pub fn transform<T: Transform>(self, transformation: T) -> Self {
        Self {
            start: self.start.transform(transformation),
            foci: [
                self.foci[0].transform(transformation),
                self.foci[1].transform(transformation),
            ],
            theta: self.theta,
        }
    }

    fn theta_z(&self) -> Radiant {
        let translation = Translation::default().with_vector(self.center());
        let right_focus = self.foci[1].transform(-translation);

        let theta = right_focus.y().atan2(right_focus.x());

        theta.into()
    }

    fn theta_y(&self) -> Radiant {
        let translation = Translation::default().with_vector(self.center());
        let right_focus = self.foci[1].transform(-translation);

        let theta = right_focus.z().atan2(right_focus.x());

        theta.into()
    }

    fn theta_x(&self) -> Radiant {
        let translation = Translation::default().with_vector(self.center());
        let right_focus = self.foci[1].transform(-translation);
        let mut start = self.start.transform(-translation);

        if self.foci[0] != self.foci[1] && right_focus.0.angle(&start.0) % PI != 0. {
            let projection = start.0.dot(&right_focus.0) / right_focus.0.dot(&right_focus.0);

            let translation =
                Translation::default().with_vector((projection * right_focus.0).into());
            start = start.transform(-translation);
        }

        let theta = start.z().atan2(start.y());

        theta.into()
    }
}
