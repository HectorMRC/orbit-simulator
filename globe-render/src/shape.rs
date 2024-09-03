use bevy::{prelude::*, render::mesh::CircleMeshBuilder};

/// Returns a [`CircleMeshBuilder`] with the given circle radius and a resolution of 255 edges.
pub fn circle_mesh(radius: f32) -> CircleMeshBuilder {
    CircleMeshBuilder {
        circle: Circle { radius },
        resolution: 255,
    }
}
