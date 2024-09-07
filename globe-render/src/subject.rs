use bevy::{input::mouse::MouseButtonInput, prelude::*};
use alvidir::name::Name;

use crate::{config::Body, cursor::Cursor};

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
                transform.translation.distance(cursor.position) <= body.0.radius.as_km() as f32
            })
            .map(|(body, _)| body)
            .next()
        {
            subject.name = Some(body.0.name.clone());
            println!("SELECTED: {:?}", subject.name);
        };
    }
}