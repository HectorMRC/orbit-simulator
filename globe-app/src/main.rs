use bevy::prelude::*;
use globe_app::GlobePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GlobePlugin)
        .run();
}
