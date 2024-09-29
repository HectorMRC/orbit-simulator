use std::f64::consts::FRAC_PI_2;

use alvidir::name::Name;
use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        mesh::{AnnulusMeshBuilder, PrimitiveTopology, SphereKind, SphereMeshBuilder},
        render_asset::RenderAssetUsages,
        storage::ShaderStorageBuffer,
    },
};
use globe_rs::{
    cartesian::{shape::{Ellipse, Sample as _}, transform::Translation, Coords}, Luminosity, Orbit as _, Radian, SystemState, Velocity
};

use crate::{
    camera::MainCamera,
    color,
    material::{OrbitTrailMaterial, RadialGradientMaterial, RadialGradientMaterialBuilder}, ui::clock::Clock,
};

const SPHERE_SUBDIVISIONS: u32 = 16;
const MESH_RESOLUTION: u32 = 255;

/// The orbital system.
#[derive(Resource)]
pub struct System {
    pub spec: globe_rs::System<Ellipse>,
}

impl From<globe_rs::System<Ellipse>> for System {
    fn from(value: globe_rs::System<Ellipse>) -> Self {
        Self { spec: value }
    }
}

/// A description of the orbital system.
#[derive(Resource)]
pub struct SystemStats {
    pub spec: globe_rs::SystemStats,
}

impl From<globe_rs::SystemStats> for SystemStats {
    fn from(value: globe_rs::SystemStats) -> Self {
        Self { spec: value }
    }
}

/// A body in the system.
#[derive(Component)]
pub struct Body {
    pub spec: globe_rs::Body,
    pub velocity: Velocity,
    pub position: Coords,
    pub theta: Radian,
}

/// The habitable zone around a body.
#[derive(Component)]
pub struct HabitableZone;

/// An orbit in the system.
#[derive(Component, Clone)]
pub struct Orbit {
    pub system: Name<globe_rs::Body>,
    pub focus: Coords,
    pub spec: Ellipse,
}

pub fn describe(mut commands: Commands, system: Res<System>) {
    commands.insert_resource(SystemStats::from(globe_rs::SystemStats::from(&system.spec)));
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

pub fn spawn_bodies<O>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    system: Res<System>,
    clock: Res<Clock>,
) {
    fn spawn_bodies_immersion(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        system: &globe_rs::System<Ellipse>,
        state: SystemState,
        parent: Option<(Name<globe_rs::Body>, Coords)>,
    ) {
        let transform = Transform::from_xyz(
            state.position.x() as f32,
            state.position.y() as f32,
            state.position.z() as f32,
        );

        let radius = system.primary.radius.as_meters() as f32;
        let material = MaterialMeshBundle {
            mesh: meshes.add(SphereMeshBuilder {
                sphere: Sphere::new(radius),
                kind: SphereKind::Ico { subdivisions: SPHERE_SUBDIVISIONS },
            }),
            material: materials.add(StandardMaterial {
                base_color: if system.primary.is_luminous() {
                    color::PERSIAN_ORANGE
                } else {
                    color::KHAKI
                },
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            transform,
            ..Default::default()
        };

        let body = Body {
            spec: system.primary.clone(),
            velocity: state.velocity,
            position: state.position,
            theta: state.theta,
        };

        let mut entity = commands.spawn((material, body));
        if let (Some((system, focus)), Some(spec)) = (parent, system.orbit) {
            entity = entity.insert(Orbit {
                system, 
                focus,
                spec,
            });
        }
        
        // entity.with_child(PointLightBundle {
        //     point_light: PointLight {   
        //         radius,
        //         color: Color::WHITE,
        //         intensity: system.primary.luminosity.as_lm() as f32,
        //         range: system.radius().as_meters() as f32,
        //         shadows_enabled: true,
        //         // shadow_depth_bias: todo!(),
        //         // shadow_normal_bias: todo!(),
        //         ..Default::default()
        //     },
        //     ..default()
        // });

        system
            .secondary
            .iter()
            .zip(state.secondary)
            .for_each(|(subsystem, substate)| {
                spawn_bodies_immersion(
                    commands,
                    meshes,
                    materials,
                    subsystem,
                    substate,
                    Some((system.primary.name.clone(), state.position)),
                )
            });
    }

    spawn_bodies_immersion(
        &mut commands,
        &mut meshes,
        &mut materials,
        &system.spec,
        system.spec.state_at(clock.elapsed_time),
        None,
    );
}

pub fn spawn_orbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OrbitTrailMaterial>>,
    orbits: Query<(&Body, &Orbit), With<Orbit>>,
    system: Res<SystemStats>,
) {
    orbits.into_iter().for_each(|(body, orbit)| {
        let Some(stats) = system.spec.stats(&orbit.system) else {
            panic!(
                "stats for system with primary body {} does not exist",
                orbit.system
            );
        };

        let mut orbit_points: Vec<[f32; 3]> = orbit
            .spec
            .with_initial_theta(body.theta)
            .sample(MESH_RESOLUTION as usize)
            .points
            .into_iter()
            .map(|coord| {
                coord
                    .transform(Translation::default().with_vector(orbit.spec.focus()))
                    .transform(Translation::default().with_vector(orbit.focus))
            })
            .map(|point| [point.x() as f32, point.y() as f32, point.z() as f32])
            .collect();

        //ensure the mesh is closed.
        orbit_points.push(orbit_points[0]);

        let orbit_mesh = Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::new(0., 0., 1.); orbit_points.len()])
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![Vec2::new(0., 0.); orbit_points.len()])
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, orbit_points);

        let trail_ratio = stats
            .secondary
            .iter()
            .fold(None, |fastest, stats| {
                let Some(fast) = fastest else {
                    return Some(stats);
                };

                if fast.max_velocity < stats.max_velocity {
                    return Some(stats);
                }

                fastest
            })
            .map(|fastest| {
                let radius_inv = 1. / fastest.radius.as_meters();
                FRAC_PI_2 / fastest.max_velocity.as_meters_sec() / radius_inv
            })
            .unwrap_or_default();

        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(orbit_mesh),
                material: materials.add(OrbitTrailMaterial {
                    center: Vec3 {
                        x: (orbit.focus + orbit.spec.focus()).x() as f32,
                        y: (orbit.focus + orbit.spec.focus()).y() as f32,
                        z: (orbit.focus + orbit.spec.focus()).z() as f32,
                    },
                    origin: Vec3 {
                        x: body.position.x() as f32,
                        y: body.position.y() as f32,
                        z: body.position.z() as f32,
                    },
                    background_color: color::JET.to_linear().to_vec4(),
                    trail_color: color::KHAKI.to_linear().to_vec4(),
                    trail_theta: (body.velocity.as_meters_sec() / orbit.spec.radius().as_meters()
                        * trail_ratio) as f32,
                    clockwise: orbit.spec.is_clockwise().then_some(1).unwrap_or_default(),
                }),
                ..default()
            },
            orbit.clone(),
        ));
    });
}

pub fn spawn_habitable_zone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<RadialGradientMaterial>>,
    camera: Query<(&Projection, &MainCamera), With<MainCamera>>,
    bodies: Query<&Body>,
) {
    let (projection, camera) = camera.single();
    let Projection::Orthographic(projection) = projection else {
        panic!("projection must be orthographic");
    };

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
                0.,
            );

            let inner_radius = hz.inner_edge.as_meters() as f32;
            let outer_radius = hz.outer_edge.as_meters() as f32;
            let quarter = (outer_radius - inner_radius) / 4.;

            let transparency = f32::min(0.1, scale / camera.initial_scale * 0.1);

            commands.spawn((    
                MaterialMeshBundle {
                    mesh: meshes.add(AnnulusMeshBuilder {
                        annulus: Annulus::new(inner_radius, outer_radius),
                        resolution: MESH_RESOLUTION,
                    }),
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
