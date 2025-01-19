use bevy::{
    asset::Handle,
    image::Image,
    math::{Quat, Vec2, Vec3},
    prelude::{
        Bundle, Camera, Children, Component, GlobalTransform, Query, Res, ResMut, Transform, With,
        Without,
    },
    sprite::Sprite,
    time::Time,
    window::{PrimaryWindow, Window},
};
use input::PlayerAction;

use crate::{Arrow, PlayerCam};

pub mod input {
    use bevy::utils::HashSet;
    use bevy::{
        input::keyboard::KeyboardInput,
        math::Vec2,
        prelude::{EventReader, KeyCode, ResMut, Resource},
    };

    #[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Resource, Default)]
    pub struct PlayerAction {
        pressed: HashSet<Direction>,
        just_pressed: HashSet<Direction>,
        just_released: HashSet<Direction>,
        pub axis: Vec2,
    }

    impl PlayerAction {
        pub fn press(&mut self, input: Direction) {
            if self.pressed.insert(input) {
                match input {
                    Direction::Up => self.axis.y = 1.,
                    Direction::Down => self.axis.y = -1.,
                    Direction::Left => self.axis.x = -1.,
                    Direction::Right => self.axis.x = 1.,
                }
                self.pressed
                    .iter()
                    .for_each(|alr_pressed| match (alr_pressed, input) {
                        (Direction::Down, Direction::Up) => self.axis.y = 0.,
                        (Direction::Up, Direction::Down) => self.axis.y = 0.,
                        (Direction::Left, Direction::Right) => self.axis.x = 0.,
                        (Direction::Right, Direction::Left) => self.axis.x = 0.,
                        _ => {}
                    });
                self.just_pressed.insert(input);
            }
        }

        pub fn release(&mut self, input: Direction) {
            if self.pressed.remove(&input) {
                match input {
                    Direction::Up => self.axis.y = 0.,
                    Direction::Down => self.axis.y = 0.,
                    Direction::Left => self.axis.x = 0.,
                    Direction::Right => self.axis.x = 0.,
                }
                self.pressed
                    .iter()
                    .for_each(|alr_pressed| match (alr_pressed, input) {
                        (Direction::Down, Direction::Up) => self.axis.y = -1.,
                        (Direction::Up, Direction::Down) => self.axis.y = 1.,
                        (Direction::Left, Direction::Right) => self.axis.x = -1.,
                        (Direction::Right, Direction::Left) => self.axis.x = 1.,
                        _ => {}
                    });
                self.just_released.insert(input);
            }
        }

        pub fn clear(&mut self) {
            self.just_pressed.clear();
            self.just_released.clear();
        }
    }

    pub fn convert(input: &KeyCode) -> Option<Direction> {
        match input {
            KeyCode::KeyW | KeyCode::ArrowUp => Some(Direction::Up),
            KeyCode::KeyA | KeyCode::ArrowLeft => Some(Direction::Left),
            KeyCode::KeyS | KeyCode::ArrowDown => Some(Direction::Down),
            KeyCode::KeyD | KeyCode::ArrowRight => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn player_inputs(mut actions: ResMut<PlayerAction>, mut input: EventReader<KeyboardInput>) {
        actions.clear();
        input
            .read()
            .map_while(|input| {
                (!input.repeat).then_some(convert(&input.key_code).map(|key| (key, input.state))?)
            })
            .for_each(|(action, state)| match state {
                bevy::input::ButtonState::Pressed => actions.press(action),
                bevy::input::ButtonState::Released => actions.release(action),
            });
    }
}

impl Player {
    pub const Z: f32 = 1.;

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
}

#[derive(Component)]
pub struct Player;

pub fn movement(
    actions: ResMut<PlayerAction>,
    mut player: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    player.iter_mut().for_each(|(_, mut transform)| {
        transform.translation +=
            time.delta_secs() * actions.axis.normalize_or_zero().extend(0.) * 100.;
    });
}

pub fn aim(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&PlayerCam, &Camera, &GlobalTransform)>,
    player: Query<(&Player, &Transform, &Children), Without<Arrow>>,
    mut arrow: Query<&mut Transform, With<Arrow>>,
) {
    let Some(cursor) = get_cursor_world_pos(window, camera) else {
        return;
    };

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

    println!("{} {}", angle.to_degrees(), transform.translation);
}

fn get_cursor_world_pos(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&PlayerCam, &Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    camera
        .get_single()
        .map(|(.., camera, transform)| {
            window
                .get_single()
                .map(|window| {
                    window
                        .cursor_position()
                        .and_then(|cursor| camera.viewport_to_world_2d(transform, cursor).ok())
                })
                .ok()?
        })
        .ok()?
}
