use std::ops::Deref;

use bevy::{
    prelude::*,
    render::storage::ShaderStorageBuffer,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use globe_rs::{cartesian::Coords, Distance, Luminosity, SystemState};

use crate::{
    camera::MainCamera,
    color,
    material::{
        LinearGradientMaterial, LinearGradientMaterialBuilder, RadialGradientMaterial,
        RadialGradientMaterialBuilder,
    },
    shape,
    time::WorldTime,
};

const HABITABLE_ZONE_Z_PLANE: f32 = -1.;
const ORBIT_Z_PLANE: f32 = -2.;

/// The orbital system.
#[derive(Resource)]
pub struct System {
    pub spec: globe_rs::System,
}

impl Deref for System {
    type Target = globe_rs::System;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl From<globe_rs::System> for System {
    fn from(value: globe_rs::System) -> Self {
        Self { spec: value }
    }
}

impl System {
    pub fn min_interorbit_distance(system: &globe_rs::System) -> Distance {
        system
            .secondary
            .iter()
            .enumerate()
            .fold(system.radius(), |min, (index, secondary)| {
                let diff = index
                    .checked_sub(1)
                    .and_then(|index| system.secondary.get(index))
                    .map(|previous| secondary.distance.diff(previous.distance))
                    .unwrap_or(system.secondary[0].distance);

                core::cmp::min(min, diff)
            })
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
    pub radius: Distance,
    pub shadow: Distance,
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
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<LinearGradientMaterial>>,
    system: Res<System>,
    time: Res<WorldTime>,
) {
    fn spawn_bodies_immersion(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        buffers: &mut ResMut<Assets<ShaderStorageBuffer>>,
        materials: &mut Assets<LinearGradientMaterial>,
        system: &globe_rs::System,
        state: SystemState,
        orbit: Option<Orbit>,
    ) {
        let transform = Transform::from_xyz(
            state.position.x() as f32,
            state.position.y() as f32,
            state.position.z() as f32,
        );

        let radius = system.primary.radius.as_km() as f32;
        let colors = if system.primary.luminosity == Luminosity::ZERO {
            (color::KHAKI, color::BATTLESHIP_GRAY)
        } else {
            let start = color::PERSIAN_ORANGE;
            (start, start.lighter(0.2))
        };

        let material = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::circle_mesh(radius))),
            transform,
            material: materials.add(
                LinearGradientMaterialBuilder::new(buffers)
                    .with_center(transform.translation)
                    .with_theta(-state.rotation.as_f64() as f32)
                    .with_segment(colors.0, 0.)
                    .with_segment(colors.1, 0.)
                    .build(),
            ),
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
                    radius: (system.primary.radius + subsystem.distance + subsystem.primary.radius),
                    shadow: System::min_interorbit_distance(system) / 10.,
                };

                spawn_bodies_immersion(
                    commands,
                    meshes,
                    buffers,
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
        &mut buffers,
        &mut materials,
        &system,
        system.state_at(time.elapsed_time),
        None,
    );
}

pub fn spawn_orbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<RadialGradientMaterial>>,
    orbits: Query<&Orbit>,
) {
    let mut orbits: Vec<&Orbit> = orbits.iter().collect();
    orbits.sort_by(|a, b| a.radius.cmp(&b.radius));

    fn coords_to_vec(coords: Coords) -> Vec3 {
        Vec3 {
            x: coords.x() as f32,
            y: coords.y() as f32,
            z: coords.z() as f32,
        }
    }

    orbits
        .into_iter()
        .enumerate()
        .map(|(index, orbit)| (index as f32, orbit))
        .for_each(|(index, orbit)| {
            let orbit_radius = orbit.radius.as_km() as f32;
            let shadow_radius = (orbit.radius + orbit.shadow).as_km() as f32;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(shape::circle_mesh(shadow_radius))),
                    transform: Transform::from_translation(
                        coords_to_vec(orbit.center).with_z(ORBIT_Z_PLANE - index),
                    ),
                    material: materials.add(
                        RadialGradientMaterialBuilder::new(&mut buffers)
                            .with_center(coords_to_vec(orbit.center))
                            .with_segment(color::EERIE_BLACK, orbit_radius)
                            .with_segment(color::NIGHT, orbit_radius)
                            .with_segment(color::NIGHT.with_alpha(0.), shadow_radius)
                            .build(),
                    ),
                    ..default()
                },
                *orbit,
            ));
        });
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
