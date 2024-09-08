use std::{cmp::min, ops::Deref};

use bevy::{
    prelude::*,
    render::storage::ShaderStorageBuffer,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use globe_rs::{Distance, Luminosity, SystemState};

use crate::{
    camera::MainCamera,
    color,
    material::{RadialGradientMaterial, RadialGradientMaterialBuilder},
    shape,
    time::WorldTime,
};

/// The orbital system.
#[derive(Resource)]
pub struct System (globe_rs::System);

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
pub struct Body{
    pub spec: globe_rs::Body,
    pub position: Vec3,
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
#[derive(Component)]
pub struct Orbit;

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

pub fn spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: (
        ResMut<Assets<ColorMaterial>>,
        ResMut<Assets<RadialGradientMaterial>>,
    ),
    system: Res<System>,
    time: Res<WorldTime>,
) {
    spawn_system_state(
        &mut commands,
        &mut meshes,
        &mut buffers,
        &mut materials,
        SystemFrame::new(&system, &system.state_at(time.elapsed_time)),
        None,
    );
}

pub fn spawn_habitable_zone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<RadialGradientMaterial>>,
    camera: Query<(&OrthographicProjection, &MainCamera), With<MainCamera>>,
    bodies: Query<&Body>,
) {

    bodies.iter().filter(|body| body.spec.luminosity != Luminosity::ZERO).for_each(|body| {
        let hz = globe_rs::HabitableZone::from(&body.spec);
        let transform = Transform::from_translation(body.position.with_z(-1.));
    
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
                        .with_segment(color::SPRING_GREEN.with_alpha(transparency), inner_radius + quarter)
                        .with_segment(color::SPRING_GREEN.with_alpha(transparency), inner_radius + 2.*quarter)
                        .with_segment(color::SPRING_GREEN.with_alpha(0.), outer_radius)
                        .build(),
                ),
                ..default()
            },
            HabitableZone,
        ));
    });
}

struct SystemFrame<'a> {
    min_interorbit_distance: Distance,
    system: &'a globe_rs::System,
    state: &'a SystemState,
}

impl<'a> SystemFrame<'a> {
    fn new(system: &'a globe_rs::System, state: &'a SystemState) -> Self {
        SystemFrame {
            min_interorbit_distance: Self::min_interorbit_distance(system),
            system: &system,
            state,
        }
    }

    fn min_interorbit_distance(system: &'a globe_rs::System) -> Distance {
        system
            .secondary
            .iter()
            .enumerate()
            .fold(Distance::ZERO, |diff, (index, secondary)| {
                if index == 0 {
                    return secondary.distance;
                }

                let previous = system.secondary[index - 1].distance;
                min(diff, secondary.distance.diff(previous))
            })
    }
}

fn spawn_system_state(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    buffers: &mut ResMut<Assets<ShaderStorageBuffer>>,
    materials: &mut (
        ResMut<Assets<ColorMaterial>>,
        ResMut<Assets<RadialGradientMaterial>>,
    ),
    current_frame: SystemFrame, 
    previous_frame: Option<&SystemFrame>,
) {
    spawn_body(commands, meshes, &mut materials.0, &current_frame);
    spawn_orbit(
        commands,
        meshes,
        buffers,
        &mut materials.1,
        &current_frame,
        previous_frame,
    );

    current_frame
        .system
        .secondary
        .iter()
        .zip(current_frame.state.secondary.iter())
        .map(|(system, state)| SystemFrame::new(system, state))
        .for_each(|frame| {
            spawn_system_state(
                commands,
                meshes,
                buffers,
                materials,
                frame,
                Some(&current_frame),
            )
        });
}   

fn spawn_body(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    frame: &SystemFrame,
) {
    let transform = Transform::from_xyz(
        frame.state.position.x() as f32,
        frame.state.position.y() as f32,
        0.,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::circle_mesh(
                frame.system.primary.radius.as_km() as f32
            ))),
            transform,
            material: if frame.system.primary.luminosity == Luminosity::ZERO {
                materials.add(color::KHAKI)     
            } else {
                materials.add(color::PERSIAN_ORANGE)
            },
            ..default()
        },
        Body{
            spec: frame.system.primary.clone(),
            position: transform.translation,
        },
    ));
}

fn spawn_orbit(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    buffers: &mut Assets<ShaderStorageBuffer>,
    materials: &mut Assets<RadialGradientMaterial>,
    current_frame: &SystemFrame,
    previous_frame: Option<&SystemFrame>,
) {
    let Some(previous_frame) = previous_frame else {
        return;
    };

    let orbit_radius = (previous_frame.system.primary.radius
        + current_frame.system.distance
        + current_frame.system.primary.radius)
        .as_km() as f32;

    let shadow_radius =
        orbit_radius + (previous_frame.min_interorbit_distance / 10.).as_km() as f32;

    let transform = Transform::from_xyz(
        previous_frame.state.position.x() as f32,
        previous_frame.state.position.y() as f32,
        -1. * current_frame.system.distance.as_km() as f32,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::circle_mesh(shadow_radius))),
            transform,
            material: materials.add(
                RadialGradientMaterialBuilder::new(buffers)
                    .with_center(transform.translation)
                    .with_segment(color::EERIE_BLACK, orbit_radius)
                    .with_segment(color::NIGHT, orbit_radius)
                    .with_segment(color::NIGHT.with_alpha(0.), shadow_radius)
                    .build(),
            ),
            ..default()
        },
        Orbit,
    ));
}
