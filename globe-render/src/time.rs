use std::{time::Duration, u32};

use bevy::prelude::*;

const SECS_PER_HOUR: u32 = 3600;

#[derive(Resource)]
pub struct WorldTime {
    pub elapsed_time: Duration,
    pub started_at: Option<Duration>,
    pub scale: u32,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self {
            elapsed_time: Default::default(),
            started_at: Default::default(),
            scale: 1,
        }
    }
}

pub fn update_time(mut world_time: ResMut<WorldTime>, time: Res<Time>) {
    if let Some(started_at) = world_time.started_at {
        let elapsed = time.elapsed();
        let scale =  world_time.scale.saturating_mul(SECS_PER_HOUR);
        world_time.elapsed_time += elapsed.abs_diff(started_at).saturating_mul(scale);
        world_time.started_at = Some(elapsed);
    }
}

pub fn update_time_settings(
    mut world_time: ResMut<WorldTime>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if world_time.started_at.take().is_none() {
            world_time.started_at = Some(time.elapsed());
        }
    } else if keys.just_pressed(KeyCode::ArrowUp) {
        world_time.scale = world_time.scale.saturating_mul(2);
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        world_time.scale = world_time.scale.saturating_div(2).max(1);
    } else if keys.just_pressed(KeyCode::KeyR) {
        world_time.elapsed_time = Duration::ZERO;
    }
}
