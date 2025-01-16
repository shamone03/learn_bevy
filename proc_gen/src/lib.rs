use std::ops::Range;

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct Terrain;

pub const CHUNK_X: i32 = 10;
pub const CHUNK_Y: i32 = 10;

const NOISE_ZOOM: f64 = 1. / 10.;

const BLOCK_SIZE: f32 = 16.;

fn cartesian_product<A, B>(
    a: A,
    b: B,
) -> impl Iterator<Item = impl Iterator<Item = (A::Item, B::Item)>>
where
    A: Iterator,
    A::Item: Copy,
    B: Iterator + Clone,
{
    a.into_iter().map(move |x| b.clone().map(move |y| (x, y)))
}

fn make_map(mut commands: Commands, assets: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let seed = rng.gen();
    println!("seed: {seed}");
    let perlin = Perlin::new(seed);

    let noise_map = get_chunks(&perlin);
    let grass = assets.load("grass.png");

    let bundles = noise_map.filter_map(move |(x, y, z)| {
        (z > 0.5).then_some((
            Sprite {
                image: grass.clone(),
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..Default::default()
            },
            Transform::from_xyz(x * BLOCK_SIZE, y * BLOCK_SIZE, 0.),
        ))
    });

    commands.spawn_batch(bundles.collect::<Vec<_>>());
}

fn get_chunks(perlin: &'_ Perlin) -> impl Iterator<Item = (f32, f32, f32)> + '_ {
    cartesian_product(-10..10, -10..10)
        .flatten()
        .map(move |(chunk_x, chunk_y)| get_chunk(perlin, chunk_x, chunk_y))
        .flatten()
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

fn get_chunk_extents(chunk_pos_x: i32, chunk_pos_y: i32) -> (Range<i32>, Range<i32>) {
    (
        (chunk_pos_x * CHUNK_X)..((chunk_pos_x * CHUNK_X) + (CHUNK_X)),
        (chunk_pos_y * CHUNK_Y)..((chunk_pos_y * CHUNK_Y) + (CHUNK_Y)),
    )
}

impl Plugin for Terrain {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_map);
    }
}
