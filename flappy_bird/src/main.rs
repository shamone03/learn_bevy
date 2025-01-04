use bevy::{asset::AssetLoader, prelude::*, sprite::Anchor, window::PrimaryWindow};
use rand::Rng;

struct FlappyBird;

#[derive(Component)]
struct Player;

enum Direction {
    Top,
    Bottom,
}

#[derive(Component)]
struct Pipe(Direction);

#[derive(Component, Default)]
struct PhysicsBody {
    velocity: Vec2,
}

#[derive(Resource)]
struct Game {
    pipe: Handle<Image>,
    window: Vec2,
    restart: bool,
}

const GRAVITY: f32 = 1000.;
const JUMP: f32 = 500.;
const VERT_GAP: f32 = 100.;
const HORZ_GAP: i32 = 200;
const PIPE_HEIGHT: f32 = 500.;
const PIPE_WIDTH: f32 = 100.;
const NUM_PIPES: i32 = 8;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let camera = Camera2d;
    commands.spawn(camera);
    let pipe = assets.load("pipe.png");

    if let Ok(window) = window.get_single() {
        println!("{:?}", window.size());
        spawn_pipes(&mut commands, pipe.clone_weak(), window.height());
        commands.insert_resource(Game {
            pipe,
            window: window.size(),
            restart: true,
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

fn get_pipes(
    x: f32,
    y: f32,
    pipe: Handle<Image>,
) -> std::array::IntoIter<(Pipe, Sprite, Transform), 2> {
    [
        (
            Pipe(Direction::Top),
            Sprite {
                flip_y: true,
                custom_size: Some(Vec2 {
                    x: PIPE_WIDTH,
                    y: PIPE_HEIGHT,
                }),
                ..Sprite::from_image(pipe.clone_weak())
            },
            Transform::from_xyz(x, y + ((PIPE_HEIGHT / 2.) + VERT_GAP), 0.),
        ),
        (
            Pipe(Direction::Bottom),
            Sprite {
                custom_size: Some(Vec2 {
                    x: PIPE_WIDTH,
                    y: PIPE_HEIGHT,
                }),
                ..Sprite::from_image(pipe.clone_weak())
            },
            Transform::from_xyz(x, y - ((PIPE_HEIGHT / 2.) + VERT_GAP), 0.),
        ),
    ]
    .into_iter()
}

fn spawn_pipes(commands: &mut Commands, pipe: Handle<Image>, height: f32) {
    let mut rng = rand::thread_rng();

    commands.spawn_batch(
        (1..=NUM_PIPES)
            .map(|i| {
                get_pipes(
                    (i * HORZ_GAP) as f32,
                    rng.gen_range((0.)..=(height / 3.)),
                    pipe.clone_weak(),
                )
            })
            .flatten()
            .collect::<Vec<_>>(),
    );
}

fn move_pipes(mut query: Query<(&Pipe, &mut Transform)>, time: Res<Time>, game: Res<Game>) {
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range((0.)..=(game.window.y / 3.));
    query.iter_mut().for_each(|(Pipe(direction), mut pipe)| {
        pipe.translation.x -= time.delta_secs() * 100.;
        if (pipe.translation.x + PIPE_WIDTH) < -(game.window.x / 2.) {
            pipe.translation.x += ((NUM_PIPES) * HORZ_GAP) as f32;
            match direction {
                Direction::Top => pipe.translation.y = offset + ((PIPE_HEIGHT / 2.) + VERT_GAP),
                Direction::Bottom => pipe.translation.y = offset - ((PIPE_HEIGHT / 2.) + VERT_GAP),
            }
        }
    })
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

fn check_offscreen(query: Query<(&Player, &Transform)>, mut game: ResMut<Game>) {
    if let Ok((_, body)) = query.get_single() {
        game.restart =
            !((-game.window.y / 2.)..=(game.window.y / 2.)).contains(&body.translation.y);
    }
}

fn check_collision(
    player_query: Query<(&Player, &Transform)>,
    pipe_query: Query<(&Pipe, &Transform)>,
    mut game: ResMut<Game>,
) {
    if let Ok((_, player)) = player_query.get_single() {
        game.restart = pipe_query.iter().any(|(_, pipe)| {
            Rect::from_center_size(pipe.translation.xy(), Vec2::new(PIPE_WIDTH, PIPE_HEIGHT))
                .contains(player.translation.xy())
        });
    }
}

impl Plugin for FlappyBird {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                input,
                apply_gravity,
                apply_physics,
                check_offscreen,
                check_collision,
                move_pipes,
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
