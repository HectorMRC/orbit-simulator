use bevy::prelude::*;

pub struct GlobePlugin;

impl Plugin for GlobePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_bodies);
    }
}

impl GlobePlugin {
    fn spawn_bodies(mut commands: Commands) {}
}
