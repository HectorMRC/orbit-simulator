use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode};

use crate::camera::MainCamera;

use super::OrbitalSystem;

/// Scrolls linearly towards the mouse wheel direction.
pub struct LinearScroll;

impl Plugin for LinearScroll {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::on_mouse_wheel_event);
    }
}

impl LinearScroll {
    pub fn on_mouse_wheel_event(
        mut scroll: EventReader<MouseWheel>,
        mut camera_query: Query<(&mut MainCamera, &mut Transform, &Projection)>,
        keys: Res<ButtonInput<KeyCode>>,
        system: Res<OrbitalSystem>,
    ) {
        if keys.pressed(KeyCode::ControlLeft) {
            // left ctrl key is reserved for zooming
            return;
        }

        scroll.read().for_each(|event| {
            if keys.pressed(KeyCode::ControlLeft) {
                // left ctrl key is reserved for zooming
                return;
            }

            let (mut camera, mut transform, projection) = camera_query.single_mut();
            let scale = match projection {
                Projection::Orthographic(projection) => match projection.scaling_mode {
                    ScalingMode::WindowSize(inv_scale) => 10. / inv_scale,
                    _ => panic!("scaling mode must be window size"),
                },

                Projection::Perspective(projection) => {
                    projection.fov / FRAC_PI_2 * (system.spec.radius().as_meters() / 50.) as f32
                }
            };

            camera.follow = None;
            transform.translation.x -= event.x * scale;
            transform.translation.y += event.y * scale;
        });
    }
}
