use crate::physics::PhysicsBody;
use bevy::{
    asset::Handle,
    image::Image,
    input::ButtonInput,
    math::Vec2,
    prelude::{Component, KeyCode, Query, Res, Transform},
    sprite::Sprite,
};

#[derive(Component)]
pub struct Player;

pub const JUMP: f32 = 500.;

impl Player {
    pub fn new(bird: Handle<Image>) -> (Player, Sprite, Transform, PhysicsBody) {
        (
            Player,
            Sprite {
                custom_size: Some(Vec2 { x: 50., y: 50. }),
                ..Sprite::from_image(bird)
            },
            Transform::IDENTITY,
            PhysicsBody::default(),
        )
    }
}

pub fn input(inputs: Res<ButtonInput<KeyCode>>, mut query: Query<(&Player, &mut PhysicsBody)>) {
    if let Ok((_, mut body)) = query.get_single_mut() {
        if inputs.just_pressed(KeyCode::Space) {
            body.velocity.y = JUMP;
        }
    }
}
