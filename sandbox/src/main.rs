use bevy::prelude::*;
use proc_gen::Terrain;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Procedural Terrain".into(),
                    position: WindowPosition::Automatic,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),))
        .add_plugins(Terrain)
        .run();
}
