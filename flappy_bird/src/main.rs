use bevy::{prelude::*, window::PrimaryWindow};
use game::Game;
use player::Player;

mod game;
mod physics;
mod pipes;
mod player;

struct FlappyBird;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let camera = Camera2d;
    commands.spawn(camera);

    let pipe = assets.load("pipe.png");
    let bird = assets.load("bird.png");

    if let Ok(window) = window.get_single() {
        pipes::spawn_pipes(&mut commands, pipe.clone_weak(), window.height());
        commands.insert_resource(Game::new(window.size(), pipe));
    }

    commands.spawn(Player::new(bird));
}

impl Plugin for FlappyBird {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                player::input,
                physics::apply_gravity,
                physics::apply_physics,
                pipes::move_pipes,
                (game::check_restart, game::restart).chain(),
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
                        title: "Flappy Bird".into(),
                        position: WindowPosition::Centered(MonitorSelection::Index(0)),
                        resolution: Vec2::new(720., 720.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FlappyBird)
        .run();
}
