use bevy::prelude::{Camera, Commands, GlobalTransform, Query, With};
use bevy::utils::HashSet;
use bevy::window::{PrimaryWindow, Window};
use bevy::{
    input::keyboard::KeyboardInput,
    math::Vec2,
    prelude::{EventReader, KeyCode, ResMut, Resource},
};

use crate::MainCam;

#[derive(Resource, Default)]
pub struct Cursor(pub Option<Vec2>);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Cursor::default());
    commands.insert_resource(PlayerAction::default());
}

pub fn mouse_world(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCam>>,
    mut cursor: ResMut<Cursor>,
) {
    cursor.0 = get_cursor_world_pos(window, camera)
}

fn get_cursor_world_pos(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCam>>,
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

pub fn kb_movement(mut actions: ResMut<PlayerAction>, mut input: EventReader<KeyboardInput>) {
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
