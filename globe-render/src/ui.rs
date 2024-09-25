use bevy::prelude::*;

use crate::{color, time::WorldTime};

#[derive(Component)]
pub struct WorldClock;

fn print_time(time: &WorldTime) -> String {
    let hours = (time.elapsed_time.as_secs_f64() / 3600.).floor();
    let mins = (time.elapsed_time.as_secs_f64()%3600. / 60.).floor();
    let secs = (time.elapsed_time.as_secs_f64()%60.).floor();

    format!("{hours:0>4}:{mins:0>2}:{secs:0>2}")
}

pub fn spawn(
    mut commands: Commands,
    time: Res<WorldTime>,
    asset_server: Res<AssetServer>,
) {
    

    commands.spawn((
        TextBundle::from_section(
            print_time(&time),
            TextStyle {
                font: asset_server.load("fonts/major_mono_display/MajorMonoDisplay-Regular.ttf"),
                font_size: 32.0,
                color: color::BATTLESHIP_GRAY,
                ..default()
            },
        ) 
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default() 
        }),
        WorldClock
    ));
}

pub fn update(
    mut clock: Query<&mut Text, With<WorldClock>>,
    time: Res<WorldTime>,
) {
    let mut world_clock = clock.single_mut();
    world_clock.sections[0].value = print_time(&time)
}