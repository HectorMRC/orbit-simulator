use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::MainCamera;

/// The world position of the mouse cursor.
#[derive(Resource, Default, Clone, Copy)]
pub struct Cursor(Vec2);

impl Cursor {
    /// Returns the position of the cursor in world-coordinates
    pub fn coords(&self) -> Vec2 {
        self.0
    }
}

/// Updates the [Cursor] resource with the corresponding world-coordinates.
pub fn into_world_coords(
    mut cursor_coords: ResMut<Cursor>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = camera_query.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = window_query.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_coords.0 = world_position;
    }
}
