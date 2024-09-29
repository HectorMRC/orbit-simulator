use std::time::Duration;

use alvidir::name::Name;
use serde::{Deserialize, Serialize};

use crate::{Distance, Luminosity, Mass, GRAVITATIONAL_CONSTANT};

/// The period and direction of a rotation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rotation {
    pub period: Duration,
    pub clockwise: bool,
}

/// An arbitrary spherical body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    /// The name of the body.
    pub name: Name<Self>,
    /// The radius of the body.
    pub radius: Distance,
    /// The rotation of the body over its own axis.
    pub spin: Rotation,
    /// The mass of the body.
    pub mass: Mass,
    /// The luminosity of the body.
    pub luminosity: Luminosity,
}

impl Body {
    /// Returns the standard gravitational parameter of the body.
    pub fn gravitational_parameter(&self) -> f64 {
        GRAVITATIONAL_CONSTANT * self.mass.as_kg()
    }

    /// Returns true if, and only if, the body has a luminousity other than zero.
    pub fn is_luminous(&self) -> bool {
        self.luminosity != Luminosity::ZERO
    }

    /// The time it takes to the body to complete a rotation.
    pub fn sideral_period(&self) -> Duration {
        self.spin.period
    }
}
