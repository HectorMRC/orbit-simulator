use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use globe_rs::{System, SystemState};

use crate::{color, material::RadialGradientMaterial, shape};

/// The configuration of the game.
#[derive(Resource)]
pub struct Config {
    pub system: globe_rs::System,
}

/// A body in the system.
#[derive(Component)]
pub struct Body;

/// An orbit in the system.
#[derive(Component)]
pub struct Orbit;

/// Spawns the configuration.
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: (
        ResMut<Assets<ColorMaterial>>,
        ResMut<Assets<RadialGradientMaterial>>,
    ),
    config: Res<Config>,
) {
    spawn_system_state(
        &mut commands,
        &mut meshes,
        &mut materials,
        &config.system,
        &config.system.state_at(Duration::ZERO),
        None,
    );
}

struct ParentState<'a> {
    body: &'a globe_rs::Body,
    state: &'a SystemState,
}

fn spawn_system_state(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut (
        ResMut<Assets<ColorMaterial>>,
        ResMut<Assets<RadialGradientMaterial>>,
    ),
    system: &System,
    state: &SystemState,
    parent: Option<&ParentState>,
) {
    spawn_body(commands, meshes, materials, system, state);
    spawn_orbit(commands, meshes, materials, system, parent);

    let parent = ParentState {
        body: &system.primary,
        state: &state,
    };

    system
        .secondary
        .iter()
        .zip(state.secondary.iter())
        .for_each(|(system, state)| {
            spawn_system_state(commands, meshes, materials, system, state, Some(&parent))
        });
}

fn spawn_body(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut (
        ResMut<Assets<ColorMaterial>>,
        ResMut<Assets<RadialGradientMaterial>>,
    ),
    system: &System,
    state: &SystemState,
) {
    let transform = Transform::from_xyz(state.position.x() as f32, state.position.y() as f32, 0.);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(shape::circle_mesh(system.primary.radius.as_km() as f32)),
            ),
            transform,
            material: materials.0.add(color::PERSIAN_ORANGE),
            ..default()
        },
        Body,
    ));
}

fn spawn_orbit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut (
        ResMut<Assets<ColorMaterial>>,
        ResMut<Assets<RadialGradientMaterial>>,
    ),
    system: &System,
    parent: Option<&ParentState>,
) {
    let Some(ParentState { body, state }) = parent else {
        return;
    };

    let orbit_radius = (body.radius + system.distance + system.primary.radius).as_km() as f32;
    let shadow_radius = orbit_radius + 10_000_000.;

    let transform = Transform::from_xyz(
        state.position.x() as f32,
        state.position.y() as f32,
        -1. * system.distance.as_km() as f32,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::circle_mesh(orbit_radius))),
            transform,
            material: materials.0.add(color::EERIE_BLACK),
            ..default()
        },
        Orbit,
    ));

    let transform = transform.with_translation(
        transform
            .translation
            .with_z(transform.translation.z - system.primary.radius.as_km() as f32),
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::circle_mesh(shadow_radius))),
            transform,
            material: materials.1.add(RadialGradientMaterial {
                from_color: color::NIGHT.into(),
                to_color: color::NIGHT.with_alpha(0.).into(),
                center: transform.translation,
                start_at: orbit_radius,
                end_at: shadow_radius,
            }),
            ..default()
        },
        Orbit,
    ));
}
