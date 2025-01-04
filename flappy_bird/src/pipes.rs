use bevy::{
    asset::Handle,
    image::Image,
    math::Vec2,
    prelude::{Commands, Component, Query, Res, Transform},
    sprite::Sprite,
    time::Time,
};
use rand::Rng;

use crate::game::Game;

pub const PIPE_WIDTH: f32 = 100.;
pub const PIPE_HEIGHT: f32 = 500.;

const VERT_GAP: f32 = 100.;
const HORZ_GAP: i32 = 200;
const NUM_PIPES: i32 = 8;

pub enum Direction {
    Top,
    Bottom,
}

#[derive(Component)]
pub struct Pipe(Direction);

fn get_pipes(
    x: f32,
    y: f32,
    pipe: Handle<Image>,
) -> std::array::IntoIter<(Pipe, Sprite, Transform), 2> {
    [
        (
            Pipe(Direction::Top),
            Sprite {
                flip_y: true,
                custom_size: Some(Vec2 {
                    x: PIPE_WIDTH,
                    y: PIPE_HEIGHT,
                }),
                ..Sprite::from_image(pipe.clone_weak())
            },
            Transform::from_xyz(x, y + ((PIPE_HEIGHT / 2.) + VERT_GAP), 0.),
        ),
        (
            Pipe(Direction::Bottom),
            Sprite {
                custom_size: Some(Vec2 {
                    x: PIPE_WIDTH,
                    y: PIPE_HEIGHT,
                }),
                ..Sprite::from_image(pipe.clone_weak())
            },
            Transform::from_xyz(x, y - ((PIPE_HEIGHT / 2.) + VERT_GAP), 0.),
        ),
    ]
    .into_iter()
}

pub fn spawn_pipes(commands: &mut Commands, pipe: Handle<Image>, height: f32) {
    let mut rng = rand::thread_rng();

    commands.spawn_batch(
        (1..=NUM_PIPES)
            .flat_map(|i| {
                get_pipes(
                    (i * HORZ_GAP) as f32,
                    rng.gen_range((0.)..=(height / 3.)),
                    pipe.clone_weak(),
                )
            })
            .collect::<Vec<_>>(),
    );
}

pub fn move_pipes(mut query: Query<(&Pipe, &mut Transform)>, time: Res<Time>, game: Res<Game>) {
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range((0.)..=(game.window_size.y / 3.));
    query.iter_mut().for_each(|(Pipe(direction), mut pipe)| {
        pipe.translation.x -= time.delta_secs() * 100.;
        if (pipe.translation.x + PIPE_WIDTH) < -(game.window_size.x / 2.) {
            pipe.translation.x += ((NUM_PIPES) * HORZ_GAP) as f32;
            match direction {
                Direction::Top => pipe.translation.y = offset + ((PIPE_HEIGHT / 2.) + VERT_GAP),
                Direction::Bottom => pipe.translation.y = offset - ((PIPE_HEIGHT / 2.) + VERT_GAP),
            }
        }
    })
}
