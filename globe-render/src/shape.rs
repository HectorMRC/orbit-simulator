use bevy::{
    prelude::*,
    render::mesh::{AnnulusMeshBuilder, CircleMeshBuilder},
};

/// Returns a [`CircleMeshBuilder`] with the given circle radius and a resolution of 255 edges.
pub fn circle_mesh(radius: f32) -> CircleMeshBuilder {
    CircleMeshBuilder {
        circle: Circle { radius },
        resolution: 255,
    }
}

/// Returns an [`AnnulusMeshBuilder`] with the given radius and a resolution of 255 edges.
pub fn annulus_mesh(inner_radius: f32, outer_radius: f32) -> AnnulusMeshBuilder {
    AnnulusMeshBuilder {
        annulus: Annulus::new(inner_radius, outer_radius),
        resolution: 255,
    }
}
