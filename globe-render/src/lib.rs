use bevy::{prelude::*, sprite::Material2dPlugin};
use config::TimeScale;
use material::RadialGradientMaterial;

mod camera;
mod color;
pub mod config;
mod cursor;
mod material;
mod scroll;
mod shape;
mod zoom;

#[derive(Component)]
pub struct Globe2DPlugin;

impl Plugin for Globe2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(Material2dPlugin::<RadialGradientMaterial>::default())
            .init_resource::<cursor::Cursor>()
            .insert_resource(TimeScale(3600))
            .add_systems(Startup, camera::spawn)
            .add_systems(Update, (config::clear, config::spawn).chain())
            .add_systems(Update, zoom::logarithmic)
            .add_systems(Update, scroll::linear)
            .add_systems(Update, cursor::into_world_coords);
    }
}
