use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{camera::MainCamera, subject::Subject};

/// Scrolls linearly towards the mouse wheel direction.
pub fn linear(
    mut scroll: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    mut subject: ResMut<Subject>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ControlLeft) {
        // left ctrl key is reserved for zooming
        return;
    }
    
    subject.name = None;
    let (mut transform, projection) = camera_query.single_mut();

    for scroll in scroll.read() {
        transform.translation.x -= 10. * scroll.x * projection.scale;
        transform.translation.y += 10. * scroll.y * projection.scale;
    }
}
