use std::str::FromStr;

use alvidir::name::Name;
use bevy::prelude::*;
use globe_render::{system::System, Globe2DPlugin};
use globe_rs::{cartesian::shape::Ellipse, Body, Distance, Frequency, Luminosity, Mass, Ratio};

fn main() {
    let system = globe_rs::System::<Ellipse> {
        primary: Body {
            name: Name::from_str("Sun").unwrap(),
            radius: Distance::km(696_340.),
            rotation: Frequency::hz(4.2866941e-7),
            mass: Mass::kg(1.9891e30),
            luminosity: Luminosity::SUN,
        },
        orbit: None,
        secondary: vec![
            globe_rs::System {
                primary: Body {
                    name: Name::from_str("Mercury").unwrap(),
                    radius: Distance::km(2_439.7),
                    rotation: Frequency::hz(1.9728535e-7),
                    mass: Mass::kg(3.30104e23),
                    luminosity: Luminosity::ZERO,
                },
                orbit: Some(Ellipse {
                    semi_major_axis: Distance::ASTRONOMICAL_UNIT * 0.38700,
                    eccentricity: Ratio::from(0.206),
                    ..Default::default()
                }),
                secondary: vec![],
            },
            globe_rs::System {
                primary: Body {
                    name: Name::from_str("Venus").unwrap(),
                    radius: Distance::km(2_439.7),
                    rotation: Frequency::hz(4.7626395e-8),
                    mass: Mass::kg(4.867e24),
                    luminosity: Luminosity::ZERO,
                },
                orbit: Some(Ellipse {
                    semi_major_axis: Distance::ASTRONOMICAL_UNIT * 0.72300,
                    eccentricity: Ratio::from(0.007),
                    ..Default::default()
                }),
                secondary: vec![],
            },
            globe_rs::System {
                primary: Body {
                    name: Name::from_str("Earth").unwrap(),
                    radius: Distance::km(6_371.),
                    rotation: Frequency::hz(1.1574074e-5),
                    mass: Mass::kg(5.97219e24),
                    luminosity: Luminosity::ZERO,
                },
                orbit: Some(Ellipse {
                    semi_major_axis: Distance::ASTRONOMICAL_UNIT,
                    eccentricity: Ratio::from(0.017),
                    ..Default::default()
                }),
                secondary: vec![globe_rs::System {
                    primary: Body {
                        name: Name::from_str("Moon").unwrap(),
                        radius: Distance::km(1_737.4),
                        rotation: Frequency::hz(4.2361738e-7),
                        mass: Mass::kg(7.34767309e22),
                        luminosity: Luminosity::ZERO,
                    },
                    orbit: Some(Ellipse {
                        semi_major_axis: Distance::km(384_748.),
                        eccentricity: Ratio::from(0.0549006),
                        ..Default::default()
                    }),
                    secondary: Default::default(),
                }],
            },
            globe_rs::System {
                primary: Body {
                    name: Name::from_str("Mars").unwrap(),
                    radius: Distance::km(3_389.5),
                    rotation: Frequency::hz(1.1111111e-5),
                    mass: Mass::kg(6.39e23),
                    luminosity: Luminosity::ZERO,
                },
                orbit: Some(Ellipse {
                    semi_major_axis: Distance::ASTRONOMICAL_UNIT * 1.52400,
                    eccentricity: Ratio::from(0.093),
                    ..Default::default()
                }),
                secondary: vec![],
            },
        ],
    };

    App::new()
        .insert_resource(System::from(system))
        .add_plugins(Globe2DPlugin)
        .run();
}
