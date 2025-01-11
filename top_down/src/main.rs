use bevy::prelude::*;
use player::{
    aim,
    input::{player_inputs, PlayerAction},
    Player,
};

struct TopDown;

mod player;

#[derive(Component)]
struct PlayerCam;

fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let camera = Camera2d;
    commands.spawn((camera, PlayerCam));
    commands.insert_resource(PlayerAction::default());

    let player = assets.load("amogus.png");
    commands.spawn(Player::new(player));
}

impl Plugin for TopDown {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PreUpdate, (player_inputs, aim));
        app.add_systems(Update, player::movement);
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
        .run();
}
