use bevy::{asset::AssetLoader, prelude::*, sprite::Anchor, window::PrimaryWindow};
use rand::Rng;

struct FlappyBird;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct PhysicsBody {
    velocity: Vec2,
}

#[derive(Resource)]
struct Game {
    pipe: Handle<Image>,
    window: Vec2,
}

const GRAVITY: f32 = 1000.;
const JUMP: f32 = 500.;
const VERT_GAP: f32 = 100.;
const HORZ_GAP: i32 = 200;
const PIPE_HEIGHT: f32 = 500.;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let camera = Camera2d;
    commands.spawn(camera);
    let pipe = assets.load("pipe.png");

    if let Ok(window) = window.get_single() {
        println!("{}", window.height());
        spawn_pipes(&mut commands, pipe.clone_weak(), window.height());
        commands.insert_resource(Game {
            pipe,
            window: window.size(),
        });
    }

    let mut bird = Sprite::from_image(assets.load("bird.png"));
    bird.custom_size = Some(Vec2 { x: 100., y: 100. });
    commands.spawn((
        Player,
        bird,
        Transform::from_xyz(0., -100., 0.),
        PhysicsBody::default(),
    ));
}

fn spawn_pipes(commands: &mut Commands, pipe: Handle<Image>, height: f32) {
    let mut rng = rand::thread_rng();

    let pipes = (1..=5)
        .map(|i| Vec2::new((i * HORZ_GAP) as f32, rng.gen_range((0.)..=(height / 3.))))
        .map(|pos| {
            [
                (
                    Sprite {
                        flip_y: true,
                        custom_size: Some(Vec2 {
                            x: 100.,
                            y: PIPE_HEIGHT,
                        }),
                        ..Sprite::from_image(pipe.clone_weak())
                    },
                    Transform::from_xyz(pos.x, pos.y + ((PIPE_HEIGHT / 2.) + VERT_GAP), 0.),
                ),
                (
                    Sprite {
                        custom_size: Some(Vec2 {
                            x: 100.,
                            y: PIPE_HEIGHT,
                        }),
                        ..Sprite::from_image(pipe.clone_weak())
                    },
                    Transform::from_xyz(pos.x, pos.y - ((PIPE_HEIGHT / 2.) + VERT_GAP), 0.),
                ),
            ]
            .into_iter()
        })
        .flatten()
        .collect::<Vec<_>>();
    commands.spawn_batch(pipes);
}

fn apply_gravity(mut query: Query<&mut PhysicsBody>, time: Res<Time>) {
    query.iter_mut().for_each(|mut pbody| {
        pbody.velocity.y -= GRAVITY * time.delta_secs();
    });
}

fn apply_physics(mut query: Query<(&mut Transform, &PhysicsBody)>, time: Res<Time>) {
    query.iter_mut().for_each(|(mut transform, pbody)| {
        transform.translation += (pbody.velocity * time.delta_secs()).extend(0.);
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
