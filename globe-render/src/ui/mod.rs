use bevy::prelude::*;
use clock::Clock;

use crate::color;

pub mod clock;

const NUMERIC_FONT: &str = "fonts/major_mono_display/MajorMonoDisplay-Regular.ttf";
const TEXT_FONT: &str = "fonts/orbitron/static/Orbitron-Bold.ttf";

const REGULAR_BORDER: Val = Val::Px(1.);
const REGULAR_PADDING: Val = Val::Px(14.);
const LARGE_PADDING: Val = Val::Px(36.);

const UI_PADDING: UiRect = UiRect::all(REGULAR_PADDING);

/// The simulator UI.
#[derive(Component)]
pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_plugins(Clock::default())
            .add_systems(Startup, Self::spawn);
    }
}

impl Ui {
    fn spawn(mut commands: Commands) {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UI_PADDING,
                    ..default()
                },
                ..default()
            })
            .with_child(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(REGULAR_BORDER),
                    ..default()
                },
                border_color: color::BATTLESHIP_GRAY.into(),
                ..default()
            });
    }
}
