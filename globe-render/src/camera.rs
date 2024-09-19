use bevy::{prelude::*, render::camera::ScalingMode};

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

    let system_radius = system.radius().as_meters() as f32;
    let initial_scale =
        (2. * system_radius) / window.resolution.width().min(window.resolution.height());

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(color::NIGHT),
                ..default()
            },
            projection: OrthographicProjection {
                near: 0.,
                far: system_radius,
                viewport_origin: Vec2::new(0.5, 0.5),
                scaling_mode: ScalingMode::WindowSize(1. / initial_scale),
                area: Default::default(),
            },
            ..Default::default()
        },
        MainCamera { initial_scale },
    ));
}
