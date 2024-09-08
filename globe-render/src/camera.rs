use bevy::prelude::*;

use crate::{color, system::System};

/// The main camera.
#[derive(Component)]
pub struct MainCamera;

/// Spawns the main camera.
pub fn spawn(mut commands: Commands, system: Res<System>, window: Query<&Window>) {
    let window = window.single();

    let system_radius = system.radius().as_km() as f32;
    let scale =
        (2. * system_radius) / f32::min(window.resolution.width(), window.resolution.height());

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(color::NIGHT),
                ..default()
            },
            projection: OrthographicProjection {
                near: -system_radius,
                far: system_radius,
                scale,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}
