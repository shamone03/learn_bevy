use bevy::prelude::*;

struct FlappyBird;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct PhysicsBody {
    velocity: Vec2,
}

const GRAVITY: f32 = 9.81;
const JUMP: f32 = 9.;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let bird = Sprite::from_image(assets.load("bevy_bird.png"));

    commands.spawn((Player, bird, Transform::IDENTITY, PhysicsBody::default()));
}

fn apply_gravity(mut query: Query<&mut PhysicsBody>, time: Res<Time>) {
    query.iter_mut().for_each(|mut pbody| {
        pbody.velocity -= GRAVITY * time.delta_secs();
    });
}

fn apply_physics(mut query: Query<(&mut Transform, &PhysicsBody)>, time: Res<Time>) {
    query.iter_mut().for_each(|(mut transform, pbody)| {
        transform.translation.y += pbody.velocity.y * time.delta_secs();
    });
}

fn input(inputs: Res<ButtonInput<KeyCode>>, mut query: Query<(&Player, &mut PhysicsBody)>) {
    if let Ok((_, mut body)) = query.get_single_mut() {
        if inputs.just_pressed(KeyCode::Space) {
            body.velocity.y = JUMP;
        }
    }
}

impl Plugin for FlappyBird {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (input, apply_gravity, apply_physics));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".into(),
                position: WindowPosition::Centered(MonitorSelection::Index(0)),
                resolution: Vec2::new(720., 720.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FlappyBird)
        .run();
}
