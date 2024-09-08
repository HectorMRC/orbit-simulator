use alvidir::name::Name;
use bevy::{input::mouse::MouseButtonInput, prelude::*};

use crate::{camera::MainCamera, cursor::Cursor, system::Body};

#[derive(Resource, Default)]
pub struct Subject {
    pub name: Option<Name<globe_rs::Body>>,
}

pub fn select_on_click(
    mut event: EventReader<MouseButtonInput>,
    bodies: Query<(&Body, &mut Transform), With<Body>>,
    mut subject: ResMut<Subject>,
    cursor: Res<Cursor>,
) {
    for event in event.read() {
        if event.state.is_pressed() {
            continue;
        }

        if let Some(body) = bodies
            .iter()
            .filter(|(body, transform)| {
                transform.translation.distance(cursor.position) <= body.radius.as_km() as f32
            })
            .map(|(body, _)| body)
            .next()
        {
            subject.name = Some(body.name.clone());
        };
    }
}

pub fn focus(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    bodies: Query<&Body>,
    subject: Res<Subject>,
) {
    let mut camera = camera.single_mut();

    let Some(subject) = &subject.name else {
        return;
    };

    if let Some(body) = bodies.iter().find(|body| &body.spec.name == subject) {
        camera.translation.x = body.position.x;
        camera.translation.y = body.position.y;
    }
}