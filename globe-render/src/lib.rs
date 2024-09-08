use bevy::{prelude::*, sprite::Material2dPlugin};
use material::RadialGradientMaterial;

mod camera;
mod color;
pub mod system;
mod cursor;
mod material;
mod scroll;
mod shape;
mod subject;
mod time;
mod zoom;

#[derive(Component)]
pub struct Globe2DPlugin;

impl Plugin for Globe2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(Material2dPlugin::<RadialGradientMaterial>::default())
            .init_resource::<cursor::Cursor>()
            .init_resource::<subject::Subject>()
            .init_resource::<time::WorldTime>()
            .add_systems(Startup, camera::spawn)
            .add_systems(PreUpdate, time::update_time)
            .add_systems(Update, (system::clear, system::spawn).chain())
            .add_systems(Update, zoom::logarithmic)
            .add_systems(Update, scroll::linear)
            .add_systems(Update, cursor::into_world_coords)
            .add_systems(Update, subject::select_on_click)
            .add_systems(Update, time::update_time_settings);
    }
}
