use std::{cmp::min, ops::Deref};

use bevy::{
    prelude::*,
    render::storage::ShaderStorageBuffer,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use globe_rs::{Distance, SystemState};

use crate::{
    camera::MainCamera,
    color,
    material::{RadialGradientMaterial, RadialGradientMaterialBuilder},
    shape,
    subject::Subject,
    time::WorldTime,
};

/// The configuration of the game.
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
pub struct Body(globe_rs::Body);

impl Deref for Body {
    type Target = globe_rs::Body;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<globe_rs::Body> for Body {
    fn from(value: globe_rs::Body) -> Self {
        Self(value)
    }
}

/// An orbit in the system.
#[derive(Component)]
pub struct Orbit;

/// Spawns the configuration.
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: (
        ResMut<Assets<ColorMaterial>>,
        ResMut<Assets<RadialGradientMaterial>>,
    ),
    mut camera: Query<&mut Transform, With<MainCamera>>,
    mut subject: ResMut<Subject>,
    system: Res<System>,
    time: Res<WorldTime>,
) {
    let mut camera = camera.single_mut();

    spawn_system_state(
        &mut commands,
        &mut meshes,
        &mut buffers,
        &mut materials,
        &mut camera,
        &mut subject,
        SystemFrame::new(&system, &system.state_at(time.elapsed_time)),
        None,
    );
}

pub fn clear(
    mut commands: Commands,
    bodies: Query<Entity, With<Body>>,
    orbits: Query<Entity, With<Orbit>>,
) {
    bodies.iter().for_each(|body| {
        commands.entity(body).clear();
    });

    orbits.iter().for_each(|orbit| {
        commands.entity(orbit).clear();
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
    camera: &mut Transform,
    subject: &mut ResMut<Subject>,
    current_frame: SystemFrame,
    previous_frame: Option<&SystemFrame>,
) {
    update_camera(camera, subject, &current_frame);
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
                camera,
                subject,
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
            material: materials.add(color::PERSIAN_ORANGE),
            ..default()
        },
        Body(frame.system.primary.clone()),
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

fn update_camera(camera: &mut Transform, subject: &mut Subject, current_frame: &SystemFrame) {
    if subject
        .name
        .as_ref()
        .map(|name| name == &current_frame.system.primary.name)
        .unwrap_or_default()
    {
        camera.translation.x = current_frame.state.position.x() as f32;
        camera.translation.y = current_frame.state.position.y() as f32;
    }
}
