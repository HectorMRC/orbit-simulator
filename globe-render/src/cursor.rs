use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::MainCamera;

/// The world position of the mouse cursor.
#[derive(Resource, Default, Clone, Copy)]
pub struct Cursor {
    pub position: Vec3,
}

/// Updates the [Cursor] resource with the corresponding world-coordinates.
pub fn into_world_coords(
    mut cursor_coords: ResMut<Cursor>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(world_position) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.with_z(0.))
    {
        cursor_coords.position = world_position;
    }
}
