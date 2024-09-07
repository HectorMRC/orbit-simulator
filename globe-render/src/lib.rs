use std::num::NonZeroU32;

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
mod subject;
mod zoom;

#[derive(Component)]
pub struct Globe2DPlugin;

impl Plugin for Globe2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(Material2dPlugin::<RadialGradientMaterial>::default())
            .init_resource::<cursor::Cursor>()
            .init_resource::<subject::Subject>()
            .insert_resource(TimeScale(NonZeroU32::new(1).unwrap()))
            .add_systems(Startup, camera::spawn)
            .add_systems(Update, (config::clear, config::spawn).chain())
            .add_systems(Update, zoom::logarithmic)
            .add_systems(Update, scroll::linear)
            .add_systems(Update, cursor::into_world_coords)
            .add_systems(Update, subject::select_on_click);
    }
}
