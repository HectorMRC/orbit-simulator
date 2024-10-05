use std::{collections::HashMap, f64::consts::FRAC_PI_2, time::Duration};

use alvidir::name::Name;
use bevy::{
    input::mouse::MouseButtonInput,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::{
        mesh::{AnnulusMeshBuilder, PrimitiveTopology, SphereKind, SphereMeshBuilder},
        render_asset::RenderAssetUsages,
        storage::ShaderStorageBuffer,
    },
};
use globe_rs::{
    cartesian::{
        shape::{Ellipse, Sample},
        transform::Translation,
    },
    Orbit as _,
};

use crate::{
    color,
    cursor::Cursor,
    event::{Clicked, Created, Deleted, Event, Updated},
    material::{OrbitTrailMaterial, RadialGradientMaterial, RadialGradientMaterialBuilder},
    ui::clock::Clock,
};

pub mod scroll;
pub mod zoom;

const SPHERE_SUBDIVISIONS: u32 = 16;
const MESH_RESOLUTION: u32 = 255;

#[derive(Resource)]
pub struct OrbitalSystemState {
    pub spec: globe_rs::OrbitalSystemState,
}

#[derive(Component, Clone)]
pub struct Body {
    pub name: Name<globe_rs::Body>,
    pub ruler: Option<Name<globe_rs::Body>>,
}

#[derive(Component)]
pub struct Orbit;

/// A description of the orbital system.
#[derive(Resource)]
pub struct OrbitalSystemStats {
    pub spec: globe_rs::SystemStats,
}

impl From<globe_rs::SystemStats> for OrbitalSystemStats {
    fn from(value: globe_rs::SystemStats) -> Self {
        Self { spec: value }
    }
}

/// The orbital system.
#[derive(Resource)]
pub struct OrbitalSystem {
    pub spec: globe_rs::OrbitalSystem<Ellipse>,
}

impl From<&globe_rs::OrbitalSystem<Ellipse>> for OrbitalSystem {
    fn from(system: &globe_rs::OrbitalSystem<Ellipse>) -> Self {
        Self {
            spec: system.clone(),
        }
    }
}

impl Plugin for OrbitalSystem {
    fn build(&self, app: &mut App) {
        app.add_event::<Event<Body, Created, Body>>()
            .add_event::<Event<Body, Updated, Body>>()
            .add_event::<Event<Body, Deleted, Body>>()
            .add_event::<Event<Body, Clicked, Body>>()
            .add_event::<Event<OrbitalSystemState, Updated>>()
            .add_systems(Startup, Self::setup)
            .add_systems(Update, Self::on_clock_tick_event)
            .add_systems(Update, Self::on_orbital_system_state_update)
            .add_systems(Update, Self::spawn_body_on_body_created)
            .add_systems(Update, Self::spawn_habitable_zone_on_body_created)
            .add_systems(Update, Self::spawn_orbit_on_body_created)
            .add_systems(Update, Self::on_body_updated)
            .add_systems(Update, Self::on_body_deleted)
            .add_systems(Update, Self::on_mouse_button_event)
            .add_plugins(zoom::LogarithmicZoom)
            .add_plugins(scroll::LinearScroll);
    }
}

impl OrbitalSystem {
    fn on_clock_tick_event(
        mut tick: EventReader<Event<Clock, Updated>>,
        mut state_updated: EventWriter<Event<OrbitalSystemState, Updated>>,
        mut state: ResMut<OrbitalSystemState>,
        system: Res<OrbitalSystem>,
        clock: Res<Clock>,
    ) {
        if tick.read().last().is_none() {
            return;
        };

        state.spec = system.spec.state_at(clock.elapsed_time);
        state_updated.send(Event::default());
    }

    pub fn setup(
        mut commands: Commands,
        mut state: EventWriter<Event<OrbitalSystemState, Updated>>,
        system: Res<OrbitalSystem>,
    ) {
        commands.insert_resource(OrbitalSystemState {
            spec: system.spec.state_at(Duration::ZERO),
        });

        commands.insert_resource(OrbitalSystemStats::from(globe_rs::SystemStats::from(
            &system.spec,
        )));

        state.send(Event::default());
    }

    fn on_orbital_system_state_update(
        mut state_updated: EventReader<Event<OrbitalSystemState, Updated>>,
        mut body_created: EventWriter<Event<Body, Created, Body>>,
        mut body_updated: EventWriter<Event<Body, Updated, Body>>,
        mut body_deleted: EventWriter<Event<Body, Deleted, Body>>,
        bodies: Query<&Body>,
        state: Res<OrbitalSystemState>,
    ) {
        if state_updated.read().last().is_none() {
            return;
        };

        let mut body_by_name: HashMap<Name<globe_rs::Body>, &Body> =
            HashMap::from_iter(bodies.iter().map(|body| (body.name.clone(), body)));

        fn spawn_or_update_immersion(
            body_created: &mut EventWriter<Event<Body, Created, Body>>,
            body_updated: &mut EventWriter<Event<Body, Updated, Body>>,
            bodies: &mut HashMap<Name<globe_rs::Body>, &Body>,
            state: &globe_rs::OrbitalSystemState,
            ruler: Option<&Name<globe_rs::Body>>,
        ) {
            if let Some(body) = bodies.remove(&state.body) {
                body_updated.send(body.clone().into());
            } else {
                body_created.send(
                    Body {
                        name: state.body.clone(),
                        ruler: ruler.cloned(),
                    }
                    .into(),
                );
            }

            state.secondary.iter().for_each(|substate| {
                spawn_or_update_immersion(
                    body_created,
                    body_updated,
                    bodies,
                    substate,
                    Some(&state.body),
                )
            });
        }

        spawn_or_update_immersion(
            &mut body_created,
            &mut body_updated,
            &mut body_by_name,
            &state.spec,
            None,
        );

        body_by_name.into_values().for_each(|body| {
            body_deleted.send(body.clone().into());
        });
    }

    fn on_body_updated(
        mut body_updated: EventReader<Event<Body, Updated, Body>>,
        mut bodies: Query<(&mut Transform, &Body), Without<Orbit>>,
        state: Res<OrbitalSystemState>,
    ) {
        body_updated
            .read()
            .filter_map(|event| state.spec.state(&event.data.name))
            .for_each(|state| {
                bodies
                    .iter_mut()
                    .filter(|(_, body)| body.name == state.body)
                    .map(|(transform, _)| transform)
                    .for_each(|mut transform| {
                        *transform = Transform::from_xyz(
                            state.position.x() as f32,
                            state.position.y() as f32,
                            state.position.z() as f32,
                        );
                    });
            });
    }

    fn on_body_deleted(
        mut commands: Commands,
        mut body_deleted: EventReader<Event<Body, Deleted, Body>>,
        mut bodies: Query<(Entity, &Body)>,
    ) {
        body_deleted.read().for_each(|event| {
            bodies
                .iter_mut()
                .filter(|(_, body)| body.name == event.data.name)
                .map(|(entity, _)| entity)
                .for_each(|entity| {
                    commands.entity(entity).clear();
                });
        });
    }

    fn spawn_body_on_body_created(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut body_created: EventReader<Event<Body, Created, Body>>,
        state: Res<OrbitalSystemState>,
        system: Res<OrbitalSystem>,
    ) {
        body_created
            .read()
            .filter_map(|event| {
                system
                    .spec
                    .system(&event.data.name)
                    .zip(state.spec.state(&event.data.name))
                    .map(|(system, state)| (system, state, event.data.clone()))
            })
            .for_each(|(system, state, body)| {
                let radius = system.primary.radius.as_meters() as f32;
                let mesh = SphereMeshBuilder {
                    sphere: Sphere::new(radius),
                    kind: SphereKind::Ico {
                        subdivisions: SPHERE_SUBDIVISIONS,
                    },
                };

                let material = StandardMaterial {
                    base_color: if system.primary.is_luminous() {
                        color::PERSIAN_ORANGE
                    } else {
                        color::KHAKI
                    },
                    alpha_mode: AlphaMode::Blend,
                    // emissive: system.primary.is_luminous().then_some(color::PERSIAN_ORANGE.into()).unwrap_or_default()      ,
                    ..Default::default()
                };

                let mut entity = commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(material)),
                    Transform::from_xyz(
                        state.position.x() as f32,
                        state.position.y() as f32,
                        state.position.z() as f32,
                    ),
                    body,
                ));

                if system.primary.is_luminous() {
                    entity.with_child((
                        PointLight {
                            radius,
                            color: Color::WHITE,
                            intensity: system.primary.luminosity.as_lm() as f32,
                            range: system.radius().as_meters() as f32,
                            shadows_enabled: true,
                            // shadow_depth_bias: todo!(),
                            // shadow_normal_bias: todo!(),
                            ..Default::default()
                        },
                        CascadeShadowConfigBuilder {
                            first_cascade_far_bound: 7.0,
                            maximum_distance: system.radius().as_meters() as f32,
                            num_cascades: 120,
                            ..Default::default()
                        }
                        .build(),
                    ));
                }
            });
    }

    #[allow(clippy::too_many_arguments)]
    fn spawn_habitable_zone_on_body_created(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
        mut materials: ResMut<Assets<RadialGradientMaterial>>,
        mut body_created: EventReader<Event<Body, Created, Body>>,
        state: Res<OrbitalSystemState>,
        system: Res<OrbitalSystem>,
    ) {
        body_created
            .read()
            .filter_map(|event| {
                system
                    .spec
                    .system(&event.data.name)
                    .zip(state.spec.state(&event.data.name))
                    .map(|(system, state)| (system, state, event.data.clone()))
            })
            .for_each(|(system, state, body)| {
                let hz = globe_rs::HabitableZone::from(&system.primary);
                if hz.outer_edge <= system.primary.radius {
                    return;
                }

                let transform =
                    Transform::from_xyz(state.position.y() as f32, -state.position.x() as f32, 0.);

                let inner_radius = hz.inner_edge.as_meters() as f32;
                let outer_radius = hz.outer_edge.as_meters() as f32;
                let quarter = (outer_radius - inner_radius) / 4.;

                let transparency = 0.1;
                let mesh = AnnulusMeshBuilder {
                    annulus: Annulus::new(inner_radius, outer_radius),
                    resolution: MESH_RESOLUTION,
                };

                let material = RadialGradientMaterialBuilder::new(&mut buffers)
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
                    .build();

                commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(material)),
                    transform,
                    body,
                ));
            });
    }

    pub fn spawn_orbit_on_body_created(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<OrbitTrailMaterial>>,
        mut body_created: EventReader<Event<Body, Created, Body>>,
        mut body_updated: EventReader<Event<Body, Updated, Body>>,
        orbits: Query<(Entity, &Body)>,
        state: Res<OrbitalSystemState>,
        stats: Res<OrbitalSystemStats>,
        system: Res<OrbitalSystem>,
    ) {
        body_created
            .read()
            .map(|read| &read.data)
            .chain(body_updated.read().map(|read| &read.data))
            .for_each(|body| {
                orbits
                    .iter()
                    .filter(|(_, object)| object.name == body.name)
                    .map(|(orbit, _)| orbit)
                    .for_each(|entity| {
                        commands.entity(entity).clear();
                    });

                let Some((body_system, body_state)) = system
                    .spec
                    .system(&body.name)
                    .zip(state.spec.state(&body.name))
                else {
                    return;
                };

                let Some(((ruler_state, ruler_stats), orbit)) = body
                    .ruler
                    .as_ref()
                    .and_then(|ruler| state.spec.state(ruler).zip(stats.spec.stats(ruler)))
                    .zip(body_system.orbit)
                else {
                    return;
                };

                let mut orbit_points: Vec<[f32; 3]> = orbit
                    .with_initial_theta(body_state.theta)
                    .sample(MESH_RESOLUTION as usize)
                    .points
                    .into_iter()
                    .map(|coord| {
                        coord
                            .transform(Translation::default().with_vector(orbit.focus()))
                            .transform(Translation::default().with_vector(ruler_state.position))
                    })
                    .map(|point| [point.x() as f32, point.y() as f32, point.z() as f32])
                    .collect();

                //ensure the mesh is closed.
                orbit_points.push(orbit_points[0]);

                let mesh = Mesh::new(
                    PrimitiveTopology::LineStrip,
                    RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    vec![Vec3::new(0., 0., 1.); orbit_points.len()],
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    vec![Vec2::new(0., 0.); orbit_points.len()],
                )
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, orbit_points);

                let trail_ratio = ruler_stats
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

                let material = OrbitTrailMaterial {
                    center: Vec3 {
                        x: (ruler_state.position.x() + orbit.focus().x()) as f32,
                        y: (ruler_state.position.y() + orbit.focus().y()) as f32,
                        z: (ruler_state.position.z() + orbit.focus().z()) as f32,
                    },
                    origin: Vec3 {
                        x: body_state.position.x() as f32,
                        y: body_state.position.y() as f32,
                        z: body_state.position.z() as f32,
                    },
                    background_color: color::JET.to_linear().to_vec4(),
                    trail_color: color::KHAKI.to_linear().to_vec4(),
                    trail_theta: (body_state.velocity.as_meters_sec() / orbit.radius().as_meters()
                        * trail_ratio) as f32,
                    clockwise: orbit.is_clockwise().then_some(1).unwrap_or_default(),
                };

                commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(material)),
                    body.clone(),
                    Orbit,
                ));
            });
    }

    pub fn on_mouse_button_event(
        mut body_clicked: EventWriter<Event<Body, Clicked, Body>>,
        mut mouse_button: EventReader<MouseButtonInput>,
        bodies: Query<(&Body, &Transform)>,
        system: Res<OrbitalSystem>,
        cursor: Res<Cursor>,
    ) {
        let Some(event) = mouse_button.read().last() else {
            return;
        };

        if event.state.is_pressed() {
            return;
        }

        if let Some(body) = bodies
            .iter()
            .filter_map(|(body, transform)| {
                system
                    .spec
                    .system(&body.name)
                    .map(|system| (system, body, transform))
            })
            .filter(|(system, _, transform)| {
                transform.translation.distance(cursor.position)
                    <= system.primary.radius.as_meters() as f32
            })
            .map(|(_, body, _)| body)
            .next()
        {
            body_clicked.send(body.clone().into());
        };
    }
}
