use bevy::prelude::*;

use crate::{color, time::WorldTime};

#[derive(Component)]
pub struct WorldClock;

const NUMERIC_FONT: &str = "fonts/major_mono_display/MajorMonoDisplay-Regular.ttf";
const TEXT_FONT: &str = "fonts/orbitron/static/Orbitron-Bold.ttf";

const REGULAR_BORDER: Val = Val::Px(1.);
const REGULAR_PADDING: Val = Val::Px(14.);
const LARGE_PADDING: Val = Val::Px(36.);

fn world_hour(time: &WorldTime) -> String {
    let hours = (time.elapsed_time.as_secs_f64() / 3600.).floor();
    format!("{hours:0>4}")
}

fn world_min_secs(time: &WorldTime) -> String {
    let mins = (time.elapsed_time.as_secs_f64() % 3600. / 60.).floor();
    let secs = (time.elapsed_time.as_secs_f64() % 60.).floor();
    format!(":{mins:0>2}:{secs:0>2}")
}

pub fn spawn(mut commands: Commands, time: Res<WorldTime>, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(REGULAR_PADDING),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // The interface border
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(REGULAR_BORDER),
                        ..default()
                    },
                    border_color: color::BATTLESHIP_GRAY.into(),
                    ..default()
                })
                .with_children(|parent| spawn_world_clock(parent, &time, &asset_server));
        });
}

pub fn update(mut clock: Query<&mut Text, With<WorldClock>>, time: Res<WorldTime>) {
    let mut world_clock = clock.single_mut();
    world_clock.sections[0].value = world_hour(&time);
    world_clock.sections[1].value = world_min_secs(&time);
}

fn spawn_world_clock(parent: &mut ChildBuilder, time: &WorldTime, asset_server: &AssetServer) {
    // clock box
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                bottom: Val::Px(0.),
                right: Val::Px(0.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // clock label
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        padding: UiRect {
                            right: LARGE_PADDING,
                            ..Default::default()
                        },
                        justify_content: JustifyContent::End,
                        ..Default::default()
                    },
                    background_color: color::BATTLESHIP_GRAY.into(),
                    ..default()
                })
                .with_child(TextBundle::from_section(
                    "Hours      Mins      Secs",
                    TextStyle {
                        font: asset_server.load(TEXT_FONT),
                        font_size: 12.,
                        color: color::NIGHT,
                    },
                ));

            // actual clock
            parent
                .spawn(NodeBundle {
                    style: Style {
                        border: UiRect {
                            bottom: Default::default(),
                            right: Default::default(),
                            ..UiRect::all(REGULAR_BORDER)
                        },
                        padding: UiRect {
                            left: LARGE_PADDING,
                            right: LARGE_PADDING,
                            ..UiRect::all(REGULAR_PADDING)
                        },
                        ..default()
                    },
                    border_color: color::BATTLESHIP_GRAY.into(),
                    background_color: color::NIGHT.with_alpha(0.7).into(),
                    ..default()
                })
                .with_child((
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: world_hour(time),
                            style: TextStyle {
                                font: asset_server.load(NUMERIC_FONT),
                                font_size: 32.0,
                                color: color::BATTLESHIP_GRAY,
                            },
                        },
                        TextSection {
                            value: world_min_secs(time),
                            style: TextStyle {
                                font: asset_server.load(NUMERIC_FONT),
                                font_size: 24.0,
                                color: color::BATTLESHIP_GRAY,
                            },
                        },
                    ])
                    .with_text_justify(JustifyText::Center),
                    WorldClock,
                ));
        });
}
