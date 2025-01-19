use bevy::prelude::*;
use player::{
    input::{self},
    Player,
};

use proc_gen::Terrain;
struct TopDown;

mod player;

#[derive(Component)]
struct MainCam;

fn setup(mut commands: Commands) {
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

impl Plugin for TopDown {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, player::setup, input::setup));
        app.add_systems(PreUpdate, (input::kb_movement, input::mouse_world));
        app.add_systems(Update, (player::movement, player::aim, move_camera));
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
