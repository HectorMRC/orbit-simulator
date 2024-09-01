use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::color;

/// The configuration of the game.
#[derive(Resource)]
pub struct Config {
    pub system: globe_rs::System,
}

/// Spawns the configuration.
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
) {
    spawn_system_state(
        &mut commands,
        &mut meshes,
        &mut materials,
        &config.system,
        &config.system.state_at(Duration::ZERO),
    );
}

pub fn spawn_system_state(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    system: &globe_rs::System,
    state: &globe_rs::SystemState,
) {
    let mesh = Mesh2dHandle(meshes.add(Circle {
        radius: system.primary.radius.as_km() as f32,
    }));

    let transform = Transform::from_xyz(state.position.x() as f32, state.position.y() as f32, 0.);

    commands.spawn(MaterialMesh2dBundle {
        mesh,
        transform,
        material: materials.add(color::PERSIAN_ORANGE),
        ..default()
    });

    system
        .secondary
        .iter()
        .zip(state.secondary.iter())
        .for_each(|(system, state)| spawn_system_state(commands, meshes, materials, system, state));
}
