use bevy::prelude::*;

use crate::{color, system::System};

/// The main camera.
#[derive(Component)]
pub struct MainCamera {
    pub initial_scale: f32,
}

/// Spawns the main camera.
pub fn spawn<O>(mut commands: Commands, system: Res<System<O>>, window: Query<&Window>)
where
    O: 'static + globe_rs::Orbit + Sync + Send,
{
    let window = window.single();

    let system_radius = (system.radius() + system.primary.radius).as_meters() as f32;
    let scale =
        (2. * system_radius) / f32::min(window.resolution.width(), window.resolution.height());

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(color::NIGHT),
                hdr: true,
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
        // BloomSettings::NATURAL,
        MainCamera {
            initial_scale: scale,
        },
    ));
}
