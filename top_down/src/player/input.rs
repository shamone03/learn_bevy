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
    commands.insert_resource(PlayerActions::default());
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
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Shoot,
}

#[derive(Resource, Default)]
pub struct PlayerActions {
    pressed: HashSet<PlayerAction>,
    just_pressed: HashSet<PlayerAction>,
    just_released: HashSet<PlayerAction>,
    pub axis: Vec2,
}

impl PlayerActions {
    pub fn press(&mut self, input: PlayerAction) {
        if self.pressed.insert(input) {
            match input {
                PlayerAction::Up => self.axis.y = 1.,
                PlayerAction::Down => self.axis.y = -1.,
                PlayerAction::Left => self.axis.x = -1.,
                PlayerAction::Right => self.axis.x = 1.,
                _ => {}
            }
            self.pressed
                .iter()
                .for_each(|alr_pressed| match (alr_pressed, input) {
                    (PlayerAction::Down, PlayerAction::Up) => self.axis.y = 0.,
                    (PlayerAction::Up, PlayerAction::Down) => self.axis.y = 0.,
                    (PlayerAction::Left, PlayerAction::Right) => self.axis.x = 0.,
                    (PlayerAction::Right, PlayerAction::Left) => self.axis.x = 0.,
                    _ => {}
                });
            self.just_pressed.insert(input);
        }
    }

    pub fn release(&mut self, input: PlayerAction) {
        if self.pressed.remove(&input) {
            match input {
                PlayerAction::Up => self.axis.y = 0.,
                PlayerAction::Down => self.axis.y = 0.,
                PlayerAction::Left => self.axis.x = 0.,
                PlayerAction::Right => self.axis.x = 0.,
                _ => {}
            }
            self.pressed
                .iter()
                .for_each(|alr_pressed| match (alr_pressed, input) {
                    (PlayerAction::Down, PlayerAction::Up) => self.axis.y = -1.,
                    (PlayerAction::Up, PlayerAction::Down) => self.axis.y = 1.,
                    (PlayerAction::Left, PlayerAction::Right) => self.axis.x = -1.,
                    (PlayerAction::Right, PlayerAction::Left) => self.axis.x = 1.,
                    _ => {}
                });
            self.just_released.insert(input);
        }
    }

    pub fn pressed(&self, input: PlayerAction) -> bool {
        self.pressed.contains(&input)
    }

    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

pub fn convert(input: &KeyCode) -> Option<PlayerAction> {
    match input {
        KeyCode::KeyW | KeyCode::ArrowUp => Some(PlayerAction::Up),
        KeyCode::KeyA | KeyCode::ArrowLeft => Some(PlayerAction::Left),
        KeyCode::KeyS | KeyCode::ArrowDown => Some(PlayerAction::Down),
        KeyCode::KeyD | KeyCode::ArrowRight => Some(PlayerAction::Right),
        KeyCode::Space => Some(PlayerAction::Shoot),
        _ => None,
    }
}

pub fn kb_movement(mut actions: ResMut<PlayerActions>, mut input: EventReader<KeyboardInput>) {
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
