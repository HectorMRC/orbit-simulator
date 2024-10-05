use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::MainCamera;

/// The world position of the mouse cursor.
#[derive(Component, Resource, Default, Clone, Copy)]
pub struct Cursor {
    pub position: Vec3,
}

impl Plugin for Cursor {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(Update, Self::into_world_coords);
    }
}

impl Cursor {
    /// Updates the [Cursor] resource with the corresponding world-coordinates.
    fn into_world_coords(
        mut cursor_coords: ResMut<Cursor>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &Projection, &GlobalTransform), With<MainCamera>>,
    ) {
        let (camera, projection, transform) = camera.single();
        let window = window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| match projection {
                Projection::Perspective(projection) => {
                    Cursor::into_world_coords_for_perspective_projection(
                        cursor, &window, projection, transform,
                    )
                }
                Projection::Orthographic(_) => {
                    Cursor::into_world_coords_for_orthographic_projection(cursor, camera, transform)
                }
            })
        {
            cursor_coords.position = world_position;
        }
    }

    fn into_world_coords_for_orthographic_projection(
        cursor: Vec2,
        camera: &Camera,
        transform: &GlobalTransform,
    ) -> Option<Vec3> {
        camera
            .viewport_to_world(transform, cursor)
            .map(|ray| ray.origin.with_z(0.))
            .ok()
    }

    fn into_world_coords_for_perspective_projection(
        cursor: Vec2,
        window: &Window,
        projection: &PerspectiveProjection,
        transform: &GlobalTransform,
    ) -> Option<Vec3> {
        let theta = projection.fov / 2.;
        let vertical_distance = 2. * transform.translation().z * theta.tan();
        let horizontal_distance = window.width() * vertical_distance / window.height();

        // from viewport coordinates to the range of [-0.5, 0.5]
        let normal_position = cursor / window.size() - 0.5;

        Some(Vec3 {
            x: transform.translation().x + normal_position.x * horizontal_distance,
            y: transform.translation().y - normal_position.y * vertical_distance,
            z: transform.translation().z,
        })
    }
}
