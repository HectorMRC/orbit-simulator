use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode};

use crate::{camera::MainCamera, subject::Subject};

/// Scrolls linearly towards the mouse wheel direction.
pub fn linear(
    mut scroll: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Transform, &Projection), With<MainCamera>>,
    mut subject: ResMut<Subject>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ControlLeft) {
        // left ctrl key is reserved for zooming
        return;
    }

    let (mut transform, projection) = camera_query.single_mut();
    let Projection::Orthographic(projection) = projection else {
        panic!("projection must be orthographic");
    };

    let scale = match projection.scaling_mode {
        ScalingMode::WindowSize(inv_scale) => 1. / inv_scale,
        _ => panic!("scaling mode must be window size"),
    };

    for scroll in scroll.read() {
        subject.name = None;
        transform.translation.x -= 10. * scroll.x * scale;
        transform.translation.y += 10. * scroll.y * scale;
    }
}
