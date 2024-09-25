use bevy::{prelude::*, sprite::Material2dPlugin};
use globe_rs::cartesian::shape::Ellipse;
use material::{OrbitTrailMaterial, RadialGradientMaterial};

mod camera;
mod color;
mod cursor;
mod material;
mod scroll;
mod subject;
pub mod system;
mod time;
mod ui;
mod zoom;

#[derive(Component)]
pub struct Globe2DPlugin;

impl Plugin for Globe2DPlugin {
    fn build(&self, app: &mut App) {
        add_orbital_system::<Ellipse>(app)
            .add_plugins(DefaultPlugins)
            .add_plugins(Material2dPlugin::<OrbitTrailMaterial>::default())
            .add_plugins(Material2dPlugin::<RadialGradientMaterial>::default())
            .init_resource::<cursor::Cursor>()
            .init_resource::<subject::Subject>()
            .init_resource::<time::WorldTime>()
            .add_systems(Startup, ui::spawn)
            .add_systems(PreUpdate, time::update_time)
            .add_systems(Update, ui::update)
            .add_systems(Update, subject::select_on_click)
            .add_systems(Update, zoom::logarithmic)
            .add_systems(Update, scroll::linear)
            .add_systems(Update, cursor::into_world_coords)
            .add_systems(Update, time::update_time_settings);
    }
}

fn add_orbital_system<O>(app: &mut App) -> &mut App
where
    O: 'static + globe_rs::Orbit + Sync + Send,
{
    app.add_systems(Startup, system::describe::<O>)
        .add_systems(Startup, camera::spawn::<O>)
        .add_systems(
            Update,
            (
                system::clear_all::<O>,
                system::spawn_bodies::<O>,
                (
                    system::spawn_orbits::<O>,
                    system::spawn_habitable_zone,
                    subject::update_camera,
                ),
            )
                .chain(),
        )
}
