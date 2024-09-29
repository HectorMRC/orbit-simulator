use std::{str::FromStr, time::Duration};

use alvidir::name::Name;
use bevy::prelude::*;
use globe_render::GlobeRsPlugin;
use globe_rs::{cartesian::shape::Ellipse, Body, Distance, Luminosity, Mass, Ratio, Rotation};

fn main() {
    let system = globe_rs::OrbitalSystem::<Ellipse> {
        primary: Body {
            name: Name::from_str("Sun").unwrap(),
            radius: Distance::km(696_340.),
            spin: Rotation {
                period: Duration::from_secs(27 * 24 * 3600),
                ..Default::default()
            },
            mass: Mass::kg(1.9891e30),
            luminosity: Luminosity::SUN,
        },
        orbit: None,
        secondary: vec![
            globe_rs::OrbitalSystem {
                primary: Body {
                    name: Name::from_str("Mercury").unwrap(),
                    radius: Distance::km(2_439.7),
                    spin: Rotation {
                        period: Duration::from_secs(59 * 24 * 3600),
                        ..Default::default()
                    },
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
            // globe_rs::System {
            //     primary: Body {
            //         name: Name::from_str("Venus").unwrap(),
            //         radius: Distance::km(2_439.7),
            //         spin: Rotation {
            //             period: Duration::from_secs(243 * 24 * 3600),
            //             clockwise: true,
            //             ..Default::default()
            //         },
            //         mass: Mass::kg(4.867e24),
            //         luminosity: Luminosity::ZERO,
            //     },
            //     orbit: Some(Ellipse {
            //         semi_major_axis: Distance::ASTRONOMICAL_UNIT * 0.72300,
            //         eccentricity: Ratio::from(0.007),
            //         ..Default::default()
            //     }),
            //     secondary: vec![],
            // },
            // globe_rs::System {
            //     primary: Body {
            //         name: Name::from_str("Earth").unwrap(),
            //         radius: Distance::km(6_371.),
            //         spin: Rotation {
            //             period: Duration::from_secs(23 * 3600 + 56 * 60 + 4),
            //             ..Default::default()
            //         },
            //         mass: Mass::kg(5.97219e24),
            //         luminosity: Luminosity::ZERO,
            //     },
            //     orbit: Some(Ellipse {
            //         semi_major_axis: Distance::ASTRONOMICAL_UNIT,
            //         eccentricity: Ratio::from(0.017),
            //         ..Default::default()
            //     }),
            //     secondary: vec![globe_rs::System {
            //         primary: Body {
            //             name: Name::from_str("Moon").unwrap(),
            //             radius: Distance::km(1_737.4),
            //             spin: Rotation {
            //                 period: Duration::from_secs(27 * 24 * 3600),
            //                 ..Default::default()
            //             },
            //             mass: Mass::kg(7.34767309e22),
            //             luminosity: Luminosity::ZERO,
            //         },
            //         orbit: Some(Ellipse {
            //             semi_major_axis: Distance::km(384_748.),
            //             eccentricity: Ratio::from(0.0549006),
            //             ..Default::default()
            //         }),
            //         secondary: Default::default(),
            //     }],
            // },
            // globe_rs::System {
            //     primary: Body {
            //         name: Name::from_str("Mars").unwrap(),
            //         radius: Distance::km(3_389.5),
            //         spin: Rotation {
            //             period: Duration::from_secs_f64(24.6 * 3600.),
            //             ..Default::default()
            //         },
            //         mass: Mass::kg(6.39e23),
            //         luminosity: Luminosity::ZERO,
            //     },
            //     orbit: Some(Ellipse {
            //         semi_major_axis: Distance::ASTRONOMICAL_UNIT * 1.52400,
            //         eccentricity: Ratio::from(0.093),
            //         ..Default::default()
            //     }),
            //     secondary: vec![],
            // },
            // globe_rs::System {
            //     primary: Body {
            //         name: Name::from_str("Jupiter").unwrap(),
            //         radius: Distance::km(69_911.),
            //         spin: Rotation {
            //             period: Duration::from_secs_f64(9. * 3600. + 55. * 60.),
            //             ..Default::default()
            //         },
            //         mass: Mass::kg(1.898e27),
            //         luminosity: Luminosity::ZERO,
            //     },
            //     orbit: Some(Ellipse {
            //         semi_major_axis: Distance::ASTRONOMICAL_UNIT * 5.2,
            //         eccentricity: Ratio::from(0.0487),
            //         ..Default::default()
            //     }),
            //     secondary: vec![],
            // },
        ],
    };

    App::new().add_plugins(GlobeRsPlugin { system }).run();
}
