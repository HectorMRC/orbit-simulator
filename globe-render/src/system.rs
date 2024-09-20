use std::ops::Deref;

use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        mesh::{AnnulusMeshBuilder, CircleMeshBuilder, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        storage::ShaderStorageBuffer,
    },
    sprite::{AlphaMode2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use globe_rs::{
    cartesian::{transform::Translation, Coords},
    Luminosity, SystemState, Velocity,
};

use crate::{
    camera::MainCamera,
    color,
    material::{RadialGradientMaterial, RadialGradientMaterialBuilder},
    time::WorldTime,
};

const ORBIT_Z_PLANE: f32 = -1.;
const HABITABLE_ZONE_Z_PLANE: f32 = -2.;
const HELIOSPHERE_Z_PLANE: f32 = -3.;

/// The orbital system.
#[derive(Resource)]
pub struct System<O: globe_rs::Orbit> {
    pub spec: globe_rs::System<O>,
}

impl<O: globe_rs::Orbit> Deref for System<O> {
    type Target = globe_rs::System<O>;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl<O: globe_rs::Orbit> From<globe_rs::System<O>> for System<O> {
    fn from(value: globe_rs::System<O>) -> Self {
        Self { spec: value }
    }
}

/// A body in the system.
#[derive(Component)]
pub struct Body {
    pub spec: globe_rs::Body,
    pub position: Coords,
    pub velocity: Velocity,
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
pub struct Orbit<O: globe_rs::Orbit> {
    pub focus: Coords,
    pub spec: O,
}

pub fn clear_all<O>(
    mut commands: Commands,
    bodies: Query<Entity, With<Body>>,
    orbits: Query<Entity, With<Orbit<O>>>,
    habitable_zone: Query<Entity, With<HabitableZone>>,
) where
    O: 'static + globe_rs::Orbit + Sync + Send,
{
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

pub fn spawn_bodies<O>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    system: Res<System<O>>,
    time: Res<WorldTime>,
) where
    O: 'static + globe_rs::Orbit + Sync + Send,
{
    fn spawn_bodies_immersion<O>(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        buffers: &mut ResMut<Assets<ShaderStorageBuffer>>,
        materials: &mut Assets<ColorMaterial>,
        system: &globe_rs::System<O>,
        state: SystemState,
        focus: Option<Coords>,
    ) where
        O: 'static + globe_rs::Orbit + Sync + Send,
    {
        let transform = Transform::from_xyz(
            state.position.x() as f32,
            state.position.y() as f32,
            state.position.z() as f32,
        );

        let radius = system.primary.radius.as_meters() as f32;
        let material = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(CircleMeshBuilder {
                circle: Circle::new(radius),
                resolution: 255,
            })),
            material: materials.add(ColorMaterial {
                color: if system.primary.is_luminous() {
                    color::PERSIAN_ORANGE
                } else {
                    color::KHAKI
                },
                alpha_mode: AlphaMode2d::Blend,
                ..Default::default()
            }),
            transform,
            ..Default::default()
        };

        let body = Body {
            spec: system.primary.clone(),
            position: state.position,
            velocity: state.velocity,
        };

        if let (Some(focus), Some(orbit)) = (focus, system.orbit) {
            commands.spawn((material, body, Orbit { focus, spec: orbit }));
        } else {
            commands.spawn((material, body));
        }

        system
            .secondary
            .iter()
            .zip(state.secondary)
            .for_each(|(subsystem, substate)| {
                spawn_bodies_immersion::<O>(
                    commands,
                    meshes,
                    buffers,
                    materials,
                    subsystem,
                    substate,
                    Some(state.position),
                )
            });
    }

    spawn_bodies_immersion::<O>(
        &mut commands,
        &mut meshes,
        &mut buffers,
        &mut materials,
        &system,
        system.state_at(time.elapsed_time),
        None,
    );
}

pub fn spawn_orbits<O>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    orbits: Query<&Orbit<O>>,
) where
    O: 'static + globe_rs::Orbit + Sync + Send,
{
    orbits.into_iter().for_each(|&orbit| {
        let mut orbit_points: Vec<[f32; 3]> = orbit
            .spec
            .sample(1024)
            .points
            .into_iter()
            .map(|coord| {
                coord
                    .transform(Translation::default().with_vector(orbit.spec.focus()))
                    .transform(Translation::default().with_vector(orbit.focus))
            })
            .map(|point| [point.x() as f32, point.y() as f32, point.z() as f32])
            .collect();

        // ensure the mesh is closed.
        orbit_points.push(orbit_points[0]);

        let orbit_mesh = Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, orbit_points);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(orbit_mesh)),
                transform: Transform::from_xyz(0., 0., ORBIT_Z_PLANE),
                material: materials.add(ColorMaterial {
                    color: color::DAVYS_GRAY,
                    alpha_mode: AlphaMode2d::Blend,
                    ..default()
                }),
                ..default()
            },
            orbit,
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
    let (projection, camera) = camera.single();
    let scale = match projection.scaling_mode {
        ScalingMode::WindowSize(inv_scale) => 1. / inv_scale,
        _ => panic!("scaling mode must be window size"),
    };

    bodies
        .iter()
        .filter(|body| body.spec.luminosity != Luminosity::ZERO)
        .for_each(|body| {
            let hz = globe_rs::HabitableZone::from(&body.spec);
            if hz.outer_edge <= body.spec.radius {
                return;
            }

            let transform = Transform::from_xyz(
                body.position.y() as f32,
                -body.position.x() as f32,
                HABITABLE_ZONE_Z_PLANE,
            );

            let inner_radius = hz.inner_edge.as_meters() as f32;
            let outer_radius = hz.outer_edge.as_meters() as f32;
            let quarter = (outer_radius - inner_radius) / 4.;

            let transparency = f32::min(0.1, scale / camera.initial_scale * 0.1);

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(AnnulusMeshBuilder {
                        annulus: Annulus::new(inner_radius, outer_radius),
                        resolution: 255,
                    })),
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

pub fn spawn_heliosphere<O>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<RadialGradientMaterial>>,
    system: Res<System<O>>,
) where
    O: 'static + globe_rs::Orbit + Sync + Send,
{
    let system_radius = 1.1 * system.radius().as_meters() as f32;
    let shadow_radius = 1.05 * system_radius;

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(CircleMeshBuilder {
            circle: Circle::new(shadow_radius),
            resolution: 255,
        })),
        transform: Transform::from_xyz(0., 0., HELIOSPHERE_Z_PLANE),
        material: materials.add(
            RadialGradientMaterialBuilder::new(&mut buffers)
                .with_segment(color::EERIE_BLACK, system_radius)
                .with_segment(Color::BLACK.with_alpha(0.5), system_radius)
                .with_segment(color::NIGHT.with_alpha(0.), shadow_radius)
                .build(),
        ),
        ..Default::default()
    });
}
