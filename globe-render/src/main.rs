use std::str::FromStr;

use alvidir::name::Name;
use bevy::prelude::*;
use globe_render::{system::System, Globe2DPlugin};
use globe_rs::{cartesian::shape::Ellipse, Body, Distance, Frequency, Luminosity, Mass, Ratio};

fn main() {
    let sun_radius = Distance::km(696_340.);
    let mercury_radius = Distance::km(2_439.7);
    let venus_radius = Distance::km(2_439.7);
    let earth_radius = Distance::km(6_371.);
    let moon_radius = Distance::km(1_737.4);
    let mars_radius = Distance::km(3_389.5);

    let system = globe_rs::System::<Ellipse> {
        primary: Body {
            name: Name::from_str("Sun").unwrap(),
            radius: sun_radius,
            rotation: Frequency::hz(4.2866941e-7),
            mass: Mass::kg(1.9891e30),
            luminosity: Luminosity::SUN,
        },
        orbit: None,
        secondary: vec![
            globe_rs::System {
                primary: Body {
                    name: Name::from_str("Mercury").unwrap(),
                    radius: mercury_radius,
                    rotation: Frequency::hz(1.9728535e-7),
                    mass: Mass::kg(3.30104e23),
                    luminosity: Luminosity::ZERO,
                },
                // orbit: Some(Circ ularOrbitBuilder {
                //     distance: Distance::km(58_000_000.) + sun_radius + mercury_radius,
                //     ..Default::default()
                // }),
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
                    radius: venus_radius,
                    rotation: Frequency::hz(4.7626395e-8),
                    mass: Mass::kg(4.867e24),
                    luminosity: Luminosity::ZERO,
                },
                // orbit: Some(CircularOrbitBuilder {
                //     distance: Distance::km(108_208_930.) + sun_radius + venus_radius,
                //     ..Default::default()
                // }),
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
                    radius: earth_radius,
                    rotation: Frequency::hz(1.1574074e-5),
                    mass: Mass::kg(5.97219e24),
                    luminosity: Luminosity::ZERO,
                },
                // orbit: Some(CircularOrbitBuilder {
                //     distance: Distance::km(150_950_000.) + sun_radius + earth_radius,
                //     ..Default::default()
                // }),
                orbit: Some(Ellipse {
                    semi_major_axis: Distance::ASTRONOMICAL_UNIT,
                    eccentricity: Ratio::from(0.017),
                    ..Default::default()
                }),
                secondary: vec![globe_rs::System {
                    primary: Body {
                        name: Name::from_str("Moon").unwrap(),
                        radius: moon_radius,
                        rotation: Frequency::hz(4.2361738e-7),
                        mass: Mass::kg(7.34767309e22),
                        luminosity: Luminosity::ZERO,
                    },
                    // orbit: Some(CircularOrbitBuilder {
                    //     distance: Distance::km(384_400.) + earth_radius + moon_radius,
                    //     ..Default::default()
                    // }),
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
                    radius: mars_radius,
                    rotation: Frequency::hz(1.1111111e-5),
                    mass: Mass::kg(6.39e23),
                    luminosity: Luminosity::ZERO,
                },
                // orbit: Some(CircularOrbitBuilder {
                //     distance: Distance::km(228_000_000.) + sun_radius + mars_radius,
                //     ..Default::default()
                // }),
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
