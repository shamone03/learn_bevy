use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use noise::{NoiseFn, Perlin};
use rand::Rng;

struct Terrain;

const CHUNK_X: i32 = 10;
const CHUNK_Y: i32 = 10;

const NOISE_ZOOM: f64 = 1. / 10.;

const BLOCK_SIZE: f32 = 5.;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn cartesian_product<T: IntoIterator + Clone>(
    a: T,
    b: T,
) -> impl Iterator<Item = impl Iterator<Item = (T::Item, T::Item)>>
where
    T::Item: Clone + Copy,
{
    a.into_iter()
        .map(move |x| b.clone().into_iter().map(move |y| (x, y)))
}

fn make_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let perlin = Perlin::new(rng.gen());

    let noise_map = get_chunks(&perlin);

    let bundles = noise_map.flatten().filter_map(|(x, y, z)| {
        (z > 0.5).then_some((
            Mesh2d(meshes.add(Circle::new(BLOCK_SIZE))),
            MeshMaterial2d(materials.add(Color::linear_rgba(1., 0., 0., z))),
            Transform::from_xyz(x * BLOCK_SIZE * 2., y * BLOCK_SIZE * 2., 0.),
        ))
    });

    commands.spawn_batch(bundles.collect::<Vec<_>>());
}

fn get_chunks(
    perlin: &'_ Perlin,
) -> impl Iterator<Item = impl Iterator<Item = (f32, f32, f32)> + '_> {
    cartesian_product(-3..=3, -3..=3)
        .flatten()
        .map(move |(chunk_x, chunk_y)| get_chunk(perlin, chunk_x, chunk_y))
}

fn get_chunk(
    perlin: &'_ Perlin,
    chunk_x: i32,
    chunk_y: i32,
) -> impl Iterator<Item = (f32, f32, f32)> + '_ {
    let (x_range, y_range) = get_chunk_extents(chunk_x, chunk_y);
    cartesian_product(x_range, y_range).flatten().map(|(i, j)| {
        (
            i as f32,
            j as f32,
            perlin
                .get([(i as f64) * NOISE_ZOOM, (j as f64) * NOISE_ZOOM])
                .abs() as f32,
        )
    })
}

fn get_chunk_extents(
    chunk_pos_x: i32,
    chunk_pos_y: i32,
) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    (
        ((chunk_pos_x * CHUNK_X) - (CHUNK_X / 2))..=((chunk_pos_x * CHUNK_X) + (CHUNK_X / 2)),
        ((chunk_pos_y * CHUNK_Y) - (CHUNK_Y / 2))..=((chunk_pos_y * CHUNK_Y) + (CHUNK_Y / 2)),
    )
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
