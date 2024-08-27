use crate::{cartesian::shape::Arc, Distance, Frequency, Mass, Velocity};

/// The gravitational constant as N⋅m^2⋅kg^−2.
pub const G: f64 = 6.67430e-11;

pub trait Orbit {   
    fn velocity(&self, central_body: Body) -> Velocity;
    fn frequency(&self, central_body: Body) -> Frequency;
}

/// An orbit in which the orbiting body moves in a perfect circle around the central body.
impl Orbit for Arc {
    fn velocity(&self, central_body: Body) -> Velocity {
        Velocity::meters_sec(
            (G * central_body.mass.as_kg() / self.radius().as_meters()).sqrt()
        )
    }
    
    fn frequency(&self, central_body: Body) -> Frequency {
        Frequency::hz(
            self.perimeter().as_meters() / self.velocity(central_body).as_meters_sec()
        )
    }
}   

/// An arbitrary spherical body.
pub struct Body {
    /// The radius of the body.
    pub radius: Distance,
    /// The mass of the body.
    pub mass: Mass,
}
