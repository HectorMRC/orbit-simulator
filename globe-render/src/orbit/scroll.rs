use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode};

use crate::camera::MainCamera;

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
        mut camera_query: Query<(&mut MainCamera, &mut Transform, &Projection), With<MainCamera>>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        if keys.pressed(KeyCode::ControlLeft) {
            // left ctrl key is reserved for zooming
            return;
        }

        let (mut camera, mut transform, projection) = camera_query.single_mut();
        let Projection::Orthographic(projection) = projection else {
            panic!("projection must be orthographic");
        };

        let scale = match projection.scaling_mode {
            ScalingMode::WindowSize(inv_scale) => 1. / inv_scale,
            _ => panic!("scaling mode must be window size"),
        };

        for scroll in scroll.read() {
            camera.follow = None;
            transform.translation.x -= 10. * scroll.x * scale;
            transform.translation.y += 10. * scroll.y * scale;
        }
    }
}
