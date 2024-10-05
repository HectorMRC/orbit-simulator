use std::{f32::consts::FRAC_PI_2, ops::DerefMut};

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::ScalingMode,
};

use crate::{camera::MainCamera, cursor::Cursor};

/// Logarithmically zooms towards the pointed object.
pub struct LogarithmicZoom;

impl Plugin for LogarithmicZoom {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::on_mouse_wheel_event);
    }
}

impl LogarithmicZoom {
    fn on_mouse_wheel_event(
        mut scroll: EventReader<MouseWheel>,
        mut camera: Query<(&MainCamera, &mut Transform, &mut Projection)>,
        keys: Res<ButtonInput<KeyCode>>,
        cursor: Res<Cursor>,
    ) {
        if !keys.pressed(KeyCode::ControlLeft) {
            // zoom required the left ctrl key to be pressed
            return;
        }

        let (camera, mut transform, mut projection) = camera.single_mut();

        scroll.read().for_each(|event| {
            if !keys.pressed(KeyCode::ControlLeft) {
                // zoom required the left ctrl key to be pressed
                return;
            }

            let orientation = match event.unit {
                MouseScrollUnit::Line => -1., // using hardware with fixed steps (e.g. mice wheel)
                MouseScrollUnit::Pixel => 1., // using fine-grained hardware (e.g. touchpads)
            };

            let scale_ratio = match projection.deref_mut() {
                Projection::Perspective(projection) => {
                    Self::on_mouse_wheel_event_for_perspective_projection(
                        event,
                        projection,
                        orientation,
                    )
                }
                Projection::Orthographic(projection) => {
                    Self::on_mouse_wheel_event_for_orthographic_projection(
                        event,
                        projection,
                        orientation,
                    )
                }
            };

            if camera.follow.is_none() {
                let relative_cursor_before = cursor.position - transform.translation;
                let relative_cursor_after = relative_cursor_before * scale_ratio;
                let translation = relative_cursor_after - relative_cursor_before;

                transform.translation.x += translation.x;
                transform.translation.y += translation.y;
            }
        });
    }

    fn on_mouse_wheel_event_for_orthographic_projection(
        event: &MouseWheel,
        projection: &mut OrthographicProjection,
        orientation: f32,
    ) -> f32 {
        let scale = match projection.scaling_mode {
            ScalingMode::WindowSize(inv_scale) => 1. / inv_scale,
            _ => panic!("scaling mode must be window size"),
        };

        let mut new_scale = scale.ln();
        new_scale += 0.1 * event.y * orientation;
        new_scale = new_scale.exp();

        let scale_ratio = scale / new_scale;
        projection.scaling_mode = ScalingMode::WindowSize(1. / new_scale);

        scale_ratio
    }

    fn on_mouse_wheel_event_for_perspective_projection(
        event: &MouseWheel,
        projection: &mut PerspectiveProjection,
        orientation: f32,
    ) -> f32 {
        let scale = projection.fov / FRAC_PI_2;

        let mut new_scale = scale.ln();
        new_scale += 0.1 * event.y * orientation;
        new_scale = new_scale.exp();

        let scale_ratio = scale / new_scale;
        projection.fov = FRAC_PI_2.min(FRAC_PI_2 * new_scale);

        scale_ratio
    }
}
