use std::time::Duration;

use bevy::{
    asset::{AssetServer, Handle},
    image::Image,
    math::{Quat, Vec2, Vec3},
    prelude::{
        BuildChildren, Bundle, Children, Commands, Component, Query, Res, ResMut, Resource,
        Transform, With, Without,
    },
    sprite::Sprite,
    time::{Time, Timer, TimerMode},
};
use input::{Cursor, PlayerActions};

use crate::physics::PhysicsBody;

pub mod input;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Arrow;

impl Player {
    pub const Z: f32 = 1.;
}

pub fn character(image: Handle<Image>) -> impl Bundle {
    (
        Player,
        Sprite {
            custom_size: Some(Vec2 { x: 50., y: 50. }),
            ..Sprite::from_image(image)
        },
        Transform::IDENTITY.with_translation(Vec3::new(0., 0., Player::Z)),
    )
}

pub fn pointer(image: Handle<Image>) -> impl Bundle {
    (
        Arrow,
        Sprite {
            anchor: bevy::sprite::Anchor::Custom(Vec2::new(-1., 0.)),
            ..Sprite::from_image(image)
        },
        Transform::from_xyz(0., 0., Player::Z),
    )
}

pub fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let player = assets.load("amogus.png");
    let arrow = assets.load("arrow.png");
    commands.spawn(character(player)).with_child(pointer(arrow));
    commands.insert_resource(Game {
        projectile: assets.load("arrow.png"),
    });

    commands.insert_resource(ShootTimer(Timer::new(
        Duration::from_millis(500),
        TimerMode::Repeating,
    )));
}

pub fn movement(
    actions: ResMut<PlayerActions>,
    mut player: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    player.iter_mut().for_each(|(_, mut transform)| {
        transform.translation +=
            time.delta_secs() * actions.axis.normalize_or_zero().extend(0.) * 100.;
    });
}

pub fn aim(
    cursor: Res<Cursor>,
    player: Query<(&Player, &Transform, &Children), Without<Arrow>>,
    mut arrow: Query<&mut Transform, With<Arrow>>,
) {
    let Some(cursor) = cursor.0 else { return };

    let Ok((_, player_transform, children)) = player.get_single() else {
        return;
    };

    let Some(id) = children.iter().next() else {
        return;
    };

    let Ok(mut transform) = arrow.get_mut(*id) else {
        return;
    };

    let Vec2 { x, y } = cursor - player_transform.translation.truncate();

    let angle = y.atan2(x);

    transform.rotation = Quat::from_rotation_z(angle);
}

#[derive(Resource)]
pub struct Game {
    projectile: Handle<Image>,
}

#[derive(Component)]
struct Projectile;

#[derive(Resource)]
pub struct ShootTimer(Timer);

pub fn shoot(
    mut commands: Commands,
    mut timer: ResMut<ShootTimer>,
    time: Res<Time>,
    input: Res<PlayerActions>,
    game: Res<Game>,
    cursor: Res<Cursor>,
    player: Query<&Transform, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() && input.pressed(input::PlayerAction::Shoot) {
        let Ok(player) = player.get_single().cloned() else {
            return;
        };

        let Some((velocity, angle)) = cursor.0.map(|cur| {
            let diff = (cur - player.translation.truncate()).normalize_or_zero();
            (diff * 200., diff.y.atan2(diff.x))
        }) else {
            return;
        };

        commands.spawn((
            Projectile,
            PhysicsBody { velocity },
            player.with_rotation(Quat::from_rotation_z(angle)),
            Sprite::from_image(game.projectile.clone_weak()),
        ));
    }
}
