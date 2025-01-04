use bevy::{
    asset::Handle,
    image::Image,
    math::{Rect, Vec2, Vec3},
    prelude::{Commands, Entity, Query, Res, ResMut, Resource, Transform, Vec3Swizzles},
};

use crate::{
    physics::PhysicsBody,
    pipes::{self, Pipe, PIPE_HEIGHT, PIPE_WIDTH},
    player::Player,
};

#[derive(Resource)]
pub struct Game {
    pub window_size: Vec2,
    pipe: Handle<Image>,
    restart: bool,
}

impl Game {
    pub fn new(window_size: Vec2, pipe: Handle<Image>) -> Self {
        Game {
            pipe,
            window_size,
            restart: false,
        }
    }
}

pub fn check_restart(
    query: Query<(&Player, &Transform)>,
    pipe_query: Query<(&Pipe, &Transform)>,
    mut game: ResMut<Game>,
) {
    if let Ok((_, player)) = query.get_single() {
        game.restart = !((-game.window_size.y / 2.)..=(game.window_size.y / 2.))
            .contains(&player.translation.y)
            || pipe_query.iter().any(|(_, pipe)| {
                Rect::from_center_size(pipe.translation.xy(), Vec2::new(PIPE_WIDTH, PIPE_HEIGHT))
                    .contains(player.translation.xy())
            });
    }
}

pub fn restart(
    game: Res<Game>,
    mut player_query: Query<(&Player, &mut Transform, &mut PhysicsBody)>,
    pipe_query: Query<(&Pipe, Entity)>,
    mut commands: Commands,
) {
    if game.restart {
        if let Ok((_, mut player, mut physics)) = player_query.get_single_mut() {
            player.translation = Vec3::ZERO;
            physics.velocity = Vec2::ZERO;
        }
        pipe_query
            .iter()
            .for_each(|(_, entity)| commands.entity(entity).despawn());
        pipes::spawn_pipes(&mut commands, game.pipe.clone_weak(), game.window_size.y);
    }
}
