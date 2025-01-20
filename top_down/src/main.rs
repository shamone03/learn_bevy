use std::time::Duration;

use bevy::prelude::*;
use physics::PhysicsBody;
use player::{
    input::{self, Cursor, PlayerActions},
    Player,
};

use proc_gen::Terrain;
struct TopDown;

mod physics;
mod player;

#[derive(Component)]
struct MainCam;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let camera = Camera2d;
    commands.spawn((
        camera,
        MainCam,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::linear_rgb(
                24. / 255.,
                101. / 255.,
                163. / 255.,
            )),
            ..Default::default()
        },
    ));

    commands.insert_resource(Game {
        projectile: assets.load("arrow.png"),
    });

    commands.insert_resource(ShootTimer(Timer::new(
        Duration::from_millis(500),
        TimerMode::Repeating,
    )));
}

fn move_camera(
    mut camera: Query<&mut Transform, (With<MainCam>, Without<Player>)>,
    player: Query<(&Player, &Transform)>,
    time: Res<Time>,
) {
    if let Ok(mut cam) = camera.get_single_mut() {
        if let Ok((.., player)) = player.get_single() {
            let pos = Vec3 {
                z: cam.translation.z,
                ..player.translation
            };
            cam.translation.smooth_nudge(&pos, 3., time.delta_secs());
        }
    }
}

#[derive(Resource)]
struct Game {
    projectile: Handle<Image>,
}

#[derive(Component)]
struct Projectile;

#[derive(Resource)]
struct ShootTimer(Timer);

fn shoot(
    input: Res<PlayerActions>,
    mut commands: Commands,
    game: Res<Game>,
    player: Query<&Transform, With<Player>>,
    cursor: Res<Cursor>,
    mut timer: ResMut<ShootTimer>,
    time: Res<Time>,
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

impl Plugin for TopDown {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, player::setup, input::setup));
        app.add_systems(PreUpdate, (input::kb_movement, input::mouse_world));
        app.add_systems(
            Update,
            (
                player::movement,
                player::aim,
                physics::apply_physics,
                move_camera,
                shoot,
            ),
        );
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Top Down Shooter".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TopDown)
        .add_plugins(Terrain)
        .run();
}
