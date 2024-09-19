use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::camera::ScalingMode,
};

use crate::{camera::MainCamera, cursor::Cursor};

/// Logarithmically zooms towards the pointed object.
pub fn logarithmic(
    mut scroll: EventReader<MouseWheel>,
    mut camera: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    cursor: Res<Cursor>,
) {
    if !keys.pressed(KeyCode::ControlLeft) {
        // zoom required the left ctrl key to be pressed
        return;
    }

    let (mut projection, mut transform) = camera.single_mut();
    for scroll in scroll.read() {
        let orientation = match scroll.unit {
            MouseScrollUnit::Line => -1., // using hardware with fixed steps (e.g. mice wheel)
            MouseScrollUnit::Pixel => 1., // using fine-grained hardware (e.g. touchpads)
        };

        let scale = match projection.scaling_mode {
            ScalingMode::WindowSize(inv_scale) => 1. / inv_scale,
            _ => panic!("scaling mode must be window size"),
        };

        let mut new_scale = scale.ln();
        new_scale += 0.1 * scroll.y * orientation;
        new_scale = new_scale.exp();

        let scale_ratio = scale / new_scale;
        projection.scaling_mode = ScalingMode::WindowSize(1. / new_scale);

        let relative_cursor_before = cursor.position - transform.translation;
        let relative_cursor_after = relative_cursor_before * scale_ratio;
        let translation = relative_cursor_after - relative_cursor_before;

        transform.translation.x += translation.x;
        transform.translation.y += translation.y;
    }
}
