use bevy::prelude::*;
use player::{
    input::{player_inputs, PlayerAction},
    Player,
};

use proc_gen::Terrain;
struct TopDown;

mod player;

#[derive(Component)]
struct PlayerCam;

fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let camera = Camera2d;
    commands.spawn((
        camera,
        PlayerCam,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::linear_rgb(24. / 255., 101. / 255., 163. / 255.)),
            ..Default::default()
        },
    ));
    commands.insert_resource(PlayerAction::default());

    let player = assets.load("amogus.png");
    commands.spawn(Player::new(player));
}

fn move_camera(
    mut camera: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
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

impl Plugin for TopDown {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PreUpdate, player_inputs);
        app.add_systems(Update, (player::movement, move_camera));
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
