use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use noise::{NoiseFn, Perlin};

struct Terrain;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let perlin = Perlin::new(90);
    let noise_map = (-50..=50)
        .map(|i| (-50..=50).map(move |j| (i, j, perlin.get([(i as f64) + 0.5, (j as f64) + 0.5]))));

    commands.spawn_batch(
        noise_map
            .flatten()
            // .filter(|(_, _, z)| z > &0.5)
            .map(|(x, y, z)| {
                (
                    Mesh2d(meshes.add(Circle::new(5.0))),
                    MeshMaterial2d(materials.add(Color::linear_rgba(1., 0., 0., z as f32))),
                    Transform::from_xyz(x as f32 * 10., y as f32 * 10., 0.),
                )
            })
            .collect::<Vec<_>>(),
    );
}

impl Plugin for Terrain {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, toggle_wireframe);
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Procedural Terrain".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            Wireframe2dPlugin,
        ))
        .add_plugins(Terrain)
        .run();
}
