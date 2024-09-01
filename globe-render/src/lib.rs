use bevy::prelude::*;

mod camera;
mod color;
pub mod config;
mod cursor;
mod scroll;
mod zoom;

#[derive(Component)]
pub struct Globe2DPlugin;

impl Plugin for Globe2DPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<cursor::Cursor>()
            .add_systems(Startup, camera::spawn)
            .add_systems(Startup, config::spawn)
            .add_systems(Update, zoom::logarithmic)
            .add_systems(Update, scroll::linear)
            .add_systems(Update, cursor::into_world_coords);
    }
}
