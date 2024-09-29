use std::time::Duration;

use bevy::prelude::*;

use crate::color;

use super::{LARGE_PADDING, NUMERIC_FONT, REGULAR_BORDER, REGULAR_PADDING, TEXT_FONT, UI_PADDING};

const SECS_PER_HOUR: u32 = 3600;

fn print_hours(duration: Duration) -> String {
    let hours = (duration.as_secs_f64() / 3600.).floor();
    format!("{hours:0>4}")
}

fn print_mins_and_secs(duration: Duration) -> String {
    let mins = (duration.as_secs_f64() % 3600. / 60.).floor();
    let secs = (duration.as_secs_f64() % 60.).floor();
    format!(":{mins:0>2}:{secs:0>2}")
}

/// Represents a clock's tick.
#[derive(Event)]
pub struct TickEvent {
    pub at: Duration,
}

impl From<Duration> for TickEvent {
    fn from(at: Duration) -> Self {
        Self { at }
    }
}

/// The world's clock.
#[derive(Resource, Component, Clone, Copy)]
pub struct Clock {
    pub elapsed_time: Duration,
    pub started_at: Option<Duration>,
    pub scale: u32,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            elapsed_time: Default::default(),
            started_at: Default::default(),
            scale: 1,
        }
    }
}

impl Plugin for Clock {
    fn build(&self, app: &mut App) {
        app.init_resource::<Self>()
            .add_event::<TickEvent>()
            .add_systems(Startup, Self::spawn)
            .add_systems(Update, Self::update)
            .add_systems(Update, Self::on_clock_tick_event)
            .add_systems(Update, Self::on_user_input_event);
    }
}

impl Clock {
    fn spawn(mut commands: Commands, clock: Res<Self>, asset_server: Res<AssetServer>) {
        // clock box
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    bottom: Val::Px(0.),
                    right: Val::Px(0.),
                    padding: UI_PADDING,
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
                            border: UiRect::all(REGULAR_BORDER),
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
                                value: print_hours(clock.elapsed_time),
                                style: TextStyle {
                                    font: asset_server.load(NUMERIC_FONT),
                                    font_size: 32.0,
                                    color: color::BATTLESHIP_GRAY,
                                },
                            },
                            TextSection {
                                value: print_mins_and_secs(clock.elapsed_time),
                                style: TextStyle {
                                    font: asset_server.load(NUMERIC_FONT),
                                    font_size: 24.0,
                                    color: color::BATTLESHIP_GRAY,
                                },
                            },
                        ])
                        .with_text_justify(JustifyText::Center),
                        *clock,
                    ));
            });
    }

    /// Updates the clock resource.
    fn update(mut tick: EventWriter<TickEvent>, mut clock: ResMut<Self>, time: Res<Time>) {
        if let Some(started_at) = clock.started_at {
            let elapsed = time.elapsed();
            let scale = clock.scale.saturating_mul(SECS_PER_HOUR);
            clock.elapsed_time += elapsed.abs_diff(started_at).saturating_mul(scale);
            clock.started_at = Some(elapsed);

            tick.send(clock.elapsed_time.into());
        }
    }

    /// Displays the latest time in the clock component.
    fn on_clock_tick_event(
        mut tick: EventReader<TickEvent>,
        mut clock_ui: Query<&mut Text, With<Clock>>,
    ) {
        let Some(tick) = tick.read().last() else {
            return;
        };

        let mut clock_ui = clock_ui.single_mut();
        clock_ui.sections[0].value = print_hours(tick.at);
        clock_ui.sections[1].value = print_mins_and_secs(tick.at);
    }

    /// Handles the user input.
    fn on_user_input_event(
        mut clock: ResMut<Self>,
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
    ) {
        if keys.just_pressed(KeyCode::Space) {
            if clock.started_at.take().is_none() {
                clock.started_at = Some(time.elapsed());
            }
        } else if keys.just_pressed(KeyCode::ArrowUp) {
            clock.scale = clock.scale.saturating_mul(2);
        } else if keys.just_pressed(KeyCode::ArrowDown) {
            clock.scale = clock.scale.saturating_div(2).max(1);
        } else if keys.just_pressed(KeyCode::KeyR) {
            clock.elapsed_time = Duration::ZERO;
        }
    }
}
