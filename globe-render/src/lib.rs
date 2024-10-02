use bevy::prelude::*;
use camera::MainCamera;
use cursor::Cursor;
use globe_rs::cartesian::shape::Ellipse;
use material::{OrbitTrailMaterial, RadialGradientMaterial};
use orbit::OrbitalSystem;
use ui::Ui;

mod camera;
mod color;
mod cursor;
mod event;
mod material;
mod orbit;
mod ui;

#[derive(Component)]
pub struct GlobeRsPlugin {
    pub system: globe_rs::OrbitalSystem<Ellipse>,
}

impl Plugin for GlobeRsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrbitalSystem::from(&self.system))
            .add_plugins(DefaultPlugins)
            .add_plugins(MaterialPlugin::<OrbitTrailMaterial>::default())
            .add_plugins(MaterialPlugin::<RadialGradientMaterial>::default())
            .add_plugins(OrbitalSystem::from(&self.system))
            .add_plugins(MainCamera::default())
            .add_plugins(Cursor::default())
            .add_plugins(Ui);
        // .init_resource::<subject::Subject>()
        // .add_systems(Update, subject::select_on_click);
        // .add_systems(Startup, system::describe::<Orbit>)
        // .add_systems(
        //     Update,
        //     (
        //         system::clear_all::<Orbit>,
        //         system::spawn_bodies::<Orbit>,
        //         (
        //             system::spawn_orbits::<Orbit>,
        //             system::spawn_habitable_zone,
        //             subject::update_camera,
        //         ),
        //     )
        //     .chain(),
        // );
    }
}
