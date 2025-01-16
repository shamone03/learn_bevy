use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use noise::{NoiseFn, Perlin};
use rand::Rng;

struct Terrain;

const CHUNK_X: i32 = 50;
const CHUNK_Y: i32 = 50;

const NOISE_ZOOM: f64 = 1. / 10.;

fn get_chunk_extents(
    chunk_pos_x: i32,
    chunk_pos_y: i32,
) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    (
        ((chunk_pos_x * CHUNK_X) - (CHUNK_X / 2))..=((chunk_pos_x * CHUNK_X) + (CHUNK_X / 2)),
        ((chunk_pos_y * CHUNK_Y) - (CHUNK_Y / 2))..=((chunk_pos_y * CHUNK_Y) + (CHUNK_Y / 2)),
    )
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn make_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let perlin = Perlin::new(rng.gen());

    let noise_map = (-10..10).map(|chunk_x| {
        (-10..10).map(move |chunk_y| {
            let (x_range, y_range) = get_chunk_extents(chunk_x, chunk_y);
            x_range.map(move |i| {
                y_range.clone().map(move |j| {
                    (
                        i as f32,
                        j as f32,
                        perlin
                            .get([(i as f64) * NOISE_ZOOM, (j as f64) * NOISE_ZOOM])
                            .abs() as f32,
                    )
                })
            })
        })
    });

    
    let bundles = noise_map
        .flatten()
        .flatten()
        .flatten()
        .filter_map(|(x, y, z)| {
            (z > 0.5).then_some((
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(Color::linear_rgba(1., 0., 0., z))),
                Transform::from_xyz(x * 10., y * 10., 0.),
            ))
        });

    commands.spawn_batch(bundles.collect::<Vec<_>>());
}

impl Plugin for Terrain {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, make_map));
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
                        position: WindowPosition::Automatic,
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
