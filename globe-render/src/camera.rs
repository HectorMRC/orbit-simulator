use alvidir::name::Name;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};

use crate::{
    color,
    event::{Clicked, Event, Updated},
    orbit::{Body, OrbitalSystem, OrbitalSystemState},
};

/// The main camera.
#[derive(Component, Default)]
pub struct MainCamera {
    pub initial_scale: f32,
    pub follow: Option<Name<globe_rs::Body>>,
}

impl Plugin for MainCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn)
            .add_systems(Update, Self::on_body_clicked)
            .add_systems(Update, Self::on_body_updated);
    }
}

impl MainCamera {
    /// Spawns the main camera.
    fn spawn(mut commands: Commands, system: Res<OrbitalSystem>, window: Query<&Window>) {
        let window = window.single();

        let system_radius = system.spec.radius().as_meters() as f32;
        let initial_scale =
            (2. * system_radius) / window.resolution.width().min(window.resolution.height());

        commands.spawn((
            Camera3dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(color::NIGHT),
                    hdr: true,
                    ..default()
                },
                projection: Projection::Orthographic(OrthographicProjection {
                    near: 0.,
                    far: 2. * system_radius,
                    viewport_origin: Vec2::new(0.5, 0.5),
                    scaling_mode: ScalingMode::WindowSize(1. / initial_scale),
                    area: Default::default(),
                }),
                transform: Transform::from_xyz(0., 0., system_radius)
                    .looking_at(Vec3::new(0., 0., 0.), Dir3::Y),
                tonemapping: Tonemapping::TonyMcMapface,
                ..Default::default()
            },
            Bloom::NATURAL,
            MainCamera {
                initial_scale,
                follow: None,
            },
        ));
    }

    pub fn on_body_clicked(
        mut body_clicked: EventReader<Event<Body, Clicked, Body>>,
        mut camera: Query<(&mut MainCamera, &mut Transform), With<MainCamera>>,
        state: Res<OrbitalSystemState>,
    ) {
        let Some(state) = body_clicked
            .read()
            .last()
            .and_then(|event| state.spec.state(&event.data.name))
        else {
            return;
        };

        let (mut camera, mut transform) = camera.single_mut();

        camera.follow = Some(state.body.clone());
        transform.translation.x = state.position.x() as f32;
        transform.translation.y = state.position.y() as f32;
    }

    pub fn on_body_updated(
        mut body_updated: EventReader<Event<Body, Updated, Body>>,
        mut camera: Query<(&mut Transform, &MainCamera), With<MainCamera>>,
        state: Res<OrbitalSystemState>,
    ) {
        let (mut transform, camera) = camera.single_mut();
        let Some(subject) = &camera.follow else {
            return;
        };

        if let Some(state) = body_updated
            .read()
            .filter(|event| &event.data.name == subject)
            .last()
            .and_then(|event| state.spec.state(&event.data.name))
        {
            transform.translation.x = state.position.x() as f32;
            transform.translation.y = state.position.y() as f32;
        };
    }
}
