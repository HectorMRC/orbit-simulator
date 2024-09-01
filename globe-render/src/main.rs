use std::str::FromStr;

use alvidir::name::Name;
use bevy::prelude::*;
use globe_render::{config, Globe2DPlugin};
use globe_rs::{Body, Distance, Frequency, Mass};

fn main() {
    App::new()
        .insert_resource(load_storage())
        .add_plugins(DefaultPlugins)
        .add_plugins(Globe2DPlugin)
        .run();
}

fn load_storage() -> config::Config {
    let system = globe_rs::System {
        primary: Body {
            name: Name::from_str("Sun").unwrap(),
            radius: Distance::km(696_340.),
            rotation: Frequency::hz(4.2866941e-7),
            mass: Mass::kg(1.9891e30),
        },
        distance: Distance::NONE,
        secondary: vec![globe_rs::System {
            primary: Body {
                name: Name::from_str("Earth").unwrap(),
                radius: Distance::km(6_371.),
                rotation: Frequency::hz(1.1574074e-5),
                mass: Mass::kg(5.97219e24),
            },
            distance: Distance::km(150_950_000.),
            secondary: vec![globe_rs::System {
                primary: Body {
                    name: Name::from_str("Moon").unwrap(),
                    radius: Distance::km(1_737.4),
                    rotation: Frequency::hz(4.2364839e-7),
                    mass: Mass::kg(7.34767309e22),
                },
                distance: Distance::km(384_400.),
                secondary: Default::default(),
            }],
        }],
    };

    config::Config { system }
}
