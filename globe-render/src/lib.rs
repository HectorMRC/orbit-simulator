use bevy::{
    prelude::*,
    render::mesh::CircleMeshBuilder,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use globe_rs::Distance;

pub mod color;

#[derive(Resource)]
pub struct Storage {
    pub system: globe_rs::System,
}

#[derive(Component)]
pub struct Globe2DPlugin;

impl Plugin for Globe2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_camera)
            .add_systems(Startup, Self::spawn_orbits)
            .add_systems(Startup, Self::spawn_bodies);
    }
}

impl Globe2DPlugin {
    fn setup_camera(mut commands: Commands, storage: Res<Storage>, window: Query<&Window>) {
        let window = window.single();

        let system_radius = storage.system.radius().as_km() as f32;
        let scale =
            (2. * system_radius) / f32::min(window.resolution.width(), window.resolution.height());

        commands.spawn(Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(color::JET),
                ..default()
            },
            projection: OrthographicProjection {
                near: -system_radius,
                far: system_radius,
                scale,
                ..default()
            },
            ..default()
        });
    }

    fn spawn_bodies(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        storage: Res<Storage>,
    ) {
        storage.system.into_iter().for_each(|system| {
            let mesh = Mesh2dHandle(meshes.add(Circle {
                radius: system.primary.radius.as_km() as f32,
            }));

            let transform = if system.distance != Distance::NONE {
                Transform::from_xyz(system.distance.as_km() as f32, 0., 0.)
            } else {
                Transform::default()
            };

            commands.spawn(MaterialMesh2dBundle {
                mesh,
                transform,
                material: materials.add(color::PERSIAN_ORANGE),
                ..default()
            });
        })
    }

    fn spawn_orbits(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        storage: Res<Storage>,
    ) {
        storage.system.into_iter().for_each(|system| {
            if system.distance == Distance::NONE {
                return;
            }

            let mesh = Mesh2dHandle(meshes.add(CircleMeshBuilder {
                circle: Circle {
                    radius: system.distance.as_km() as f32,
                },
                resolution: 255,
            }));

            commands.spawn(MaterialMesh2dBundle {
                mesh,
                transform: Transform::from_xyz(0., 0., -1.),
                material: materials.add(color::DAVYS_GRAY),
                ..default()
            });
        })
    }
}
