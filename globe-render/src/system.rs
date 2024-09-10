use std::ops::Deref;

use bevy::{
    prelude::*,
    render::{
        mesh::PrimitiveTopology, render_asset::RenderAssetUsages, storage::ShaderStorageBuffer,
    },
    sprite::{AlphaMode2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use globe_rs::{
    cartesian::{
        shape::{Arc, Sample},
        Coords,
    },
    Luminosity, Radiant, SystemState,
};

use crate::{
    camera::MainCamera,
    color,
    material::{RadialGradientMaterial, RadialGradientMaterialBuilder},
    shape,
    time::WorldTime,
};

const HABITABLE_ZONE_Z_PLANE: f32 = -1.;
const ORBIT_Z_PLANE: f32 = -2.;

/// The orbital system.
#[derive(Resource)]
pub struct System(globe_rs::System);

impl Deref for System {
    type Target = globe_rs::System;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<globe_rs::System> for System {
    fn from(value: globe_rs::System) -> Self {
        Self(value)
    }
}

/// A body in the system.
#[derive(Component)]
pub struct Body {
    pub spec: globe_rs::Body,
    pub position: Coords,
}

impl Deref for Body {
    type Target = globe_rs::Body;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

/// The habitable zone around a body.
#[derive(Component)]
pub struct HabitableZone;

/// An orbit in the system.
#[derive(Component, Default, Clone, Copy)]
pub struct Orbit {
    pub center: Coords,
}

pub fn clear_all(
    mut commands: Commands,
    bodies: Query<Entity, With<Body>>,
    orbits: Query<Entity, With<Orbit>>,
    habitable_zone: Query<Entity, With<HabitableZone>>,
) {
    bodies.iter().for_each(|body| {
        commands.entity(body).clear();
    });

    orbits.iter().for_each(|orbit| {
        commands.entity(orbit).clear();
    });

    habitable_zone.iter().for_each(|hz| {
        commands.entity(hz).clear();
    });
}

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    system: Res<System>,
    time: Res<WorldTime>,
) {
    fn spawn_bodies_immersion(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        system: &globe_rs::System,
        state: SystemState,
        orbit: Option<Orbit>,
    ) {
        let transform = Transform::from_xyz(
            state.position.x() as f32,
            state.position.y() as f32,
            state.position.z() as f32,
        );

        let material = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(shape::circle_mesh(system.primary.radius.as_km() as f32)),
            ),
            transform,
            material: materials.add(ColorMaterial {
                alpha_mode: AlphaMode2d::Blend,
                color: if system.primary.luminosity == Luminosity::ZERO {
                    color::KHAKI
                } else {
                    color::PERSIAN_ORANGE
                },
                ..Default::default()
            }),
            ..default()
        };

        let body = Body {
            spec: system.primary.clone(),
            position: state.position,
        };

        if let Some(orbit) = orbit {
            commands.spawn((material, body, orbit));
        } else {
            commands.spawn((material, body));
        }

        system
            .secondary
            .iter()
            .zip(state.secondary)
            .for_each(|(subsystem, substate)| {
                let orbit = Orbit {
                    center: state.position,
                };

                spawn_bodies_immersion(
                    commands,
                    meshes,
                    materials,
                    subsystem,
                    substate,
                    Some(orbit),
                )
            });
    }

    spawn_bodies_immersion(
        &mut commands,
        &mut meshes,
        &mut materials,
        &system,
        system.state_at(time.elapsed_time),
        None,
    );
}

pub fn spawn_orbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    orbits: Query<(&Body, &Orbit), With<Orbit>>,
) {
    // orbits.into_iter().for_each(|(body, &orbit)| {
    //     let mut orbit_points: Vec<[f32; 3]> = Arc::default()
    //         .with_center(orbit.center)
    //         .with_start(body.position)
    //         .with_axis(Coords::default().with_z(1.))
    //         .with_theta(Radiant::TWO_PI)
    //         .sample(255)
    //         .points
    //         .into_iter()
    //         .map(|point| [point.x() as f32, point.y() as f32, point.z() as f32])
    //         .collect();

    //     // ensure the mesh is closed.
    //     orbit_points.push(orbit_points[0]);

    //     let orbit_mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::RENDER_WORLD)
    //         .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, orbit_points);

    //     commands.spawn((
    //         MaterialMesh2dBundle {
    //             mesh: Mesh2dHandle(meshes.add(orbit_mesh)),
    //             material: materials.add(ColorMaterial {
    //                 color: Color::linear_rgb(0., 1., 0.),
    //                 alpha_mode: AlphaMode2d::Blend,
    //                 ..Default::default()
    //             }),
    //             ..default()
    //         },
    //         orbit,
    //     ));
    // });

    let orbit_points: Vec<[f32; 3]> = orbits
        .into_iter()
        .flat_map(|(body, &orbit)| {
            let orbit_points: Vec<[f32; 3]> = Arc::default()
                .with_center(orbit.center)
                .with_start(body.position)
                .with_axis(Coords::default().with_z(1.))
                .with_theta(Radiant::TWO_PI)
                .sample(255)
                .points
                .into_iter()
                .map(|point| [point.x() as f32, point.y() as f32, point.z() as f32])
                .collect();

            let mut next_points = orbit_points.iter().cycle();
            next_points.next();

            orbit_points
                .iter()
                .zip(next_points)
                .flat_map(|(&current, &next)| [current, next])
                .collect::<Vec<[f32; 3]>>()
        })
        .collect();

    let orbit_mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, orbit_points);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(orbit_mesh)),
            transform: Transform::from_xyz(0., 0., ORBIT_Z_PLANE),
            material: materials.add(ColorMaterial {
                color: color::BATTLESHIP_GRAY,
                alpha_mode: AlphaMode2d::Blend,
                ..Default::default()
            }),
            ..default()
        },
        Orbit::default(),
    ));
}

pub fn spawn_habitable_zone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<RadialGradientMaterial>>,
    camera: Query<(&OrthographicProjection, &MainCamera), With<MainCamera>>,
    bodies: Query<&Body>,
) {
    bodies
        .iter()
        .filter(|body| body.spec.luminosity != Luminosity::ZERO)
        .for_each(|body| {
            let hz = globe_rs::HabitableZone::from(&body.spec);
            let transform = Transform::from_xyz(
                body.position.y() as f32,
                -body.position.x() as f32,
                HABITABLE_ZONE_Z_PLANE,
            );

            let inner_radius = hz.inner_edge.as_km() as f32;
            let outer_radius = hz.outer_edge.as_km() as f32;
            let quarter = (outer_radius - inner_radius) / 4.;

            let (projection, camera) = camera.single();
            let transparency = f32::min(0.1, projection.scale / camera.initial_scale * 0.1);

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(shape::annulus_mesh(inner_radius, outer_radius))),
                    transform,
                    material: materials.add(
                        RadialGradientMaterialBuilder::new(&mut buffers)
                            .with_center(transform.translation)
                            .with_segment(color::SPRING_GREEN.with_alpha(0.), inner_radius)
                            .with_segment(
                                color::SPRING_GREEN.with_alpha(transparency),
                                inner_radius + quarter,
                            )
                            .with_segment(
                                color::SPRING_GREEN.with_alpha(transparency),
                                inner_radius + 2. * quarter,
                            )
                            .with_segment(color::SPRING_GREEN.with_alpha(0.), outer_radius)
                            .build(),
                    ),
                    ..default()
                },
                HabitableZone,
            ));
        });
}
