use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
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

        let mut scale = projection.scale.ln();
        scale += 0.1 * scroll.y * orientation;
        scale = scale.exp();

        let scale_ratio = projection.scale / scale;
        projection.scale = scale;

        let relative_cursor_before = cursor.position - transform.translation;
        let relative_cursor_after = relative_cursor_before * scale_ratio;
        let translation = relative_cursor_after - relative_cursor_before;

        transform.translation.x += translation.x;
        transform.translation.y += translation.y;
    }
}
