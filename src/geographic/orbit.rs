use crate::{cartesian::shape::Arc, Distance, Frequency, Mass, Velocity};

/// The gravitational constant as N⋅m^2⋅kg^−2.
pub const G: f64 = 6.67430e-11;

/// The orbit of an object around a central body.
pub trait Orbit {
    /// The orbital velocity of the object.
    fn velocity(&self, central_body: Body) -> Velocity;
    /// The orbit's frequency.
    fn frequency(&self, central_body: Body) -> Frequency;
}

/// An orbit in which the orbiting body moves in a perfect circle around the central body.
impl Orbit for Arc {
    fn velocity(&self, central_body: Body) -> Velocity {
        Velocity::meters_sec((G * central_body.mass.as_kg() / self.radius().as_meters()).sqrt())
    }

    fn frequency(&self, central_body: Body) -> Frequency {
        Frequency::hz(self.velocity(central_body).as_meters_sec() / self.perimeter().as_meters())
    }
}

/// An arbitrary spherical body.
pub struct Body {
    /// The radius of the body.
    pub radius: Distance,
    /// The mass of the body.
    pub mass: Mass,
}

/// An orbital system.
pub struct System {
    /// The central body of the system.
    pub primary: Body,
    /// The distance between the surface of the primary body of this system and that of the one it
    /// orbits, if any.
    pub radius: Option<Distance>,
    /// The systems orbiting the primary body.
    pub secondary: Vec<System>,
}
