use std::{num::NonZeroU32, time::Duration};

use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldTime {
    pub elapsed_time: Duration,
    pub started_at: Option<Duration>,
    pub scale: NonZeroU32,
}

impl Default for WorldTime {
    fn default() -> Self {
        Self {
            elapsed_time: Default::default(),
            started_at: Default::default(),
            scale: NonZeroU32::new(3600).unwrap(),
        }
    }
}

impl WorldTime {
    pub fn with_scale(mut self, scale: NonZeroU32) -> Self {
        self.scale = scale;
        self
    }
}

pub fn update_time(mut world_time: ResMut<WorldTime>, time: Res<Time>) {
    if let Some(started_at) = world_time.started_at {
        let elapsed = time.elapsed();
        let scale = world_time.scale.get();
        world_time.elapsed_time += elapsed.abs_diff(started_at) * scale;
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
    } else if keys.pressed(KeyCode::ArrowUp) {
        world_time.scale = world_time.scale.checked_add(3600).unwrap();
    } else if keys.pressed(KeyCode::ArrowDown) {
        world_time.scale = world_time
            .scale
            .get()
            .checked_sub(3600)
            .and_then(NonZeroU32::new)
            .unwrap_or(NonZeroU32::MIN);
    } else if keys.just_pressed(KeyCode::KeyR) {
        world_time.elapsed_time = Duration::ZERO;
    }
}
