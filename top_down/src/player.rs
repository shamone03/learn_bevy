use bevy::{
    asset::Handle,
    image::Image,
    math::Vec2,
    prelude::{Camera, Camera2d, Component, GlobalTransform, Query, Res, ResMut, Transform, With},
    sprite::Sprite,
    time::Time,
    window::{PrimaryWindow, Window},
};
use input::{CursorPos, PlayerAction};

use crate::PlayerCam;

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

    #[derive(Resource)]
    pub struct CursorPos(Vec2);

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
            KeyCode::KeyW => Some(Direction::Up),
            KeyCode::KeyA => Some(Direction::Left),
            KeyCode::KeyS => Some(Direction::Down),
            KeyCode::KeyD => Some(Direction::Right),
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
    pub fn new(image: Handle<Image>) -> (Player, Sprite, Transform) {
        (
            Player,
            Sprite {
                custom_size: Some(Vec2 { x: 50., y: 50. }),
                ..Sprite::from_image(image)
            },
            Transform::IDENTITY,
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
) {
    if let Ok((_, camera, camera_transform)) = camera.get_single() {
        if let Ok(window) = window.get_single() {
            if let Some(cursor) = window.cursor_position() {
                let world_pos = camera.viewport_to_world_2d(camera_transform, cursor);
                if let Ok(pos) = world_pos {
                    println!("{cursor:?} {pos:?}");
                }
            }
        }
    }
}
