use std::{collections::HashMap, marker::PhantomData, time::Duration};

use alvidir::name::Name;
use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    render::mesh::{SphereKind, SphereMeshBuilder},
};
use globe_rs::cartesian::{shape::Ellipse, Coords};

use crate::{camera::MainCamera, color, cursor::Cursor, ui::clock::TickEvent};

pub mod scroll;
pub mod zoom;

const SPHERE_SUBDIVISIONS: u32 = 16;
const MESH_RESOLUTION: u32 = 255;

pub struct Created;
pub struct Updated;
pub struct Deleted;
pub struct Clicked;

#[derive(Event)]
pub struct OrbitalSystemState {
    pub spec: globe_rs::OrbitalSystemState,
}

#[derive(Event)]
pub struct BodyEvent<K> {
    pub body: Body,
    pub kind: PhantomData<K>,
}

impl<K> From<&Body> for BodyEvent<K> {
    fn from(body: &Body) -> Self {
        Self {
            body: body.clone(),
            kind: PhantomData,
        }
    }
}

impl<K> From<Body> for BodyEvent<K> {
    fn from(body: Body) -> Self {
        Self {
            body,
            kind: PhantomData,
        }
    }
}

#[derive(Component, Clone)]
pub struct Body {
    pub name: Name<globe_rs::Body>,
    pub position: Coords,
    pub ruler: Option<Name<globe_rs::Body>>,
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
        app.add_event::<OrbitalSystemState>()
            .add_event::<BodyEvent<Created>>()
            .add_event::<BodyEvent<Updated>>()
            .add_event::<BodyEvent<Deleted>>()
            .add_event::<BodyEvent<Clicked>>()
            .add_plugins(zoom::LogarithmicZoom)
            .add_plugins(scroll::LinearScroll)
            .add_systems(Startup, Self::setup)
            .add_systems(Update, Self::on_clock_tick_event)
            .add_systems(Update, Self::on_orbital_system_state)
            .add_systems(Update, Self::on_body_created)
            .add_systems(Update, Self::on_body_updated)
            .add_systems(Update, Self::on_body_deleted)
            .add_systems(Update, Self::on_mouse_button_event);
    }
}

impl OrbitalSystem {
    fn on_clock_tick_event(
        mut state: EventWriter<OrbitalSystemState>,
        mut tick: EventReader<TickEvent>,
        system: Res<OrbitalSystem>,
    ) {
        let Some(tick) = tick.read().last() else {
            return;
        };

        state.send(OrbitalSystemState {
            spec: system.spec.state_at(tick.at),
        });
    }

    pub fn setup(mut state: EventWriter<OrbitalSystemState>, system: Res<OrbitalSystem>) {
        state.send(OrbitalSystemState {
            spec: system.spec.state_at(Duration::ZERO),
        });
    }

    fn on_orbital_system_state(
        mut state: EventReader<OrbitalSystemState>,
        mut body_created: EventWriter<BodyEvent<Created>>,
        mut body_updated: EventWriter<BodyEvent<Updated>>,
        mut body_deleted: EventWriter<BodyEvent<Deleted>>,
        mut bodies: Query<&mut Body>,
    ) {
        let Some(state) = state.read().last() else {
            return;
        };

        let mut body_by_name: HashMap<Name<globe_rs::Body>, Mut<Body>> =
            HashMap::from_iter(bodies.iter_mut().map(|body| (body.name.clone(), body)));

        fn spawn_or_update_immersion(
            body_created: &mut EventWriter<BodyEvent<Created>>,
            body_updated: &mut EventWriter<BodyEvent<Updated>>,
            bodies: &mut HashMap<Name<globe_rs::Body>, Mut<Body>>,
            state: &globe_rs::OrbitalSystemState,
            ruler: Option<&Name<globe_rs::Body>>,
        ) {
            if let Some(mut body) = bodies.remove(&state.body) {
                body.position = state.position;
                body_updated.send(body.as_ref().into());
            } else {
                body_created.send(
                    Body {
                        name: state.body.clone(),
                        position: state.position,
                        ruler: ruler.cloned(),
                    }
                    .into(),
                );
            }

            state.secondary.iter().for_each(|subsystem| {
                spawn_or_update_immersion(
                    body_created,
                    body_updated,
                    bodies,
                    subsystem,
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
            body_deleted.send(body.as_ref().into());
        });
    }

    fn on_body_created(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut body_created: EventReader<BodyEvent<Created>>,
        system: Res<OrbitalSystem>,
    ) {
        body_created.read().for_each(|event| {
            let Some(system) = system.spec.system(&event.body.name) else {
                return;
            };

            let transform = Transform::from_xyz(
                event.body.position.x() as f32,
                event.body.position.y() as f32,
                event.body.position.z() as f32,
            );

            let radius = system.primary.radius.as_meters() as f32;
            let material = MaterialMeshBundle {
                mesh: meshes.add(SphereMeshBuilder {
                    sphere: Sphere::new(radius),
                    kind: SphereKind::Ico {
                        subdivisions: SPHERE_SUBDIVISIONS,
                    },
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

            commands
                .spawn((material, event.body.clone()))
                .with_child(PointLightBundle {
                    point_light: PointLight {
                        radius,
                        color: Color::WHITE,
                        intensity: system.primary.luminosity.as_lm() as f32,
                        range: system.radius().as_meters() as f32,
                        shadows_enabled: true,
                        // shadow_depth_bias: todo!(),
                        // shadow_normal_bias: todo!(),
                        ..Default::default()
                    },
                    ..default()
                });
        });
    }

    fn on_body_updated(
        mut body_updated: EventReader<BodyEvent<Updated>>,
        mut bodies: Query<(&mut Transform, &Body), With<Body>>,
    ) {
        body_updated.read().for_each(|event| {
            bodies
                .iter_mut()
                .filter(|(_, body)| body.name == event.body.name)
                .map(|(transform, _)| transform)
                .for_each(|mut transform| {
                    *transform = Transform::from_xyz(
                        event.body.position.x() as f32,
                        event.body.position.y() as f32,
                        event.body.position.z() as f32,
                    );
                });
        });
    }

    fn on_body_deleted(
        mut commands: Commands,
        mut body_deleted: EventReader<BodyEvent<Deleted>>,
        mut bodies: Query<(Entity, &Body), With<Body>>,
    ) {
        body_deleted.read().for_each(|event| {
            bodies
                .iter_mut()
                .filter(|(_, body)| body.name == event.body.name)
                .map(|(entity, _)| entity)
                .for_each(|entity| {
                    commands.entity(entity).clear();
                });
        });
    }

    pub fn on_mouse_button_event(
        mut body_clicked: EventWriter<BodyEvent<Clicked>>,
        mut mouse_button: EventReader<MouseButtonInput>,
        bodies: Query<(&Body, &Transform), With<Body>>,
        cursor: Res<Cursor>,
        system: Res<OrbitalSystem>,
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
                    .and_then(|system| Some((system, body, transform)))
            })
            .filter(|(system, _, transform)| {
                transform.translation.distance(cursor.position)
                    <= system.primary.radius.as_meters() as f32
            })
            .map(|(_, body, _)| body)
            .next()
        {
            body_clicked.send(body.into());
        };
    }
}
