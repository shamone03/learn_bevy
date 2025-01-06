use bevy::{
    asset::Handle,
    image::Image,
    math::Vec2,
    prelude::{Component, Query, Res, ResMut, Transform},
    sprite::Sprite,
    time::Time,
};
use input::PlayerAction;

pub mod input {
    use bevy::utils::HashSet;
    use bevy::{
        input::keyboard::KeyboardInput,
        math::Vec2,
        prelude::{EventReader, KeyCode, ResMut, Resource},
    };

    #[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
    pub enum PlayerInput {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Resource)]
    pub struct PlayerAction {
        pressed: HashSet<PlayerInput>,
        just_pressed: HashSet<PlayerInput>,
        just_released: HashSet<PlayerInput>,
        pub axis: Vec2,
    }

    impl Default for PlayerAction {
        fn default() -> Self {
            Self {
                pressed: Default::default(),
                just_pressed: Default::default(),
                just_released: Default::default(),
                axis: Default::default(),
            }
        }
    }

    impl PlayerAction {
        pub fn press(&mut self, input: PlayerInput) {
            if self.pressed.insert(input) {
                match input {
                    PlayerInput::Up => self.axis.y = 1.,
                    PlayerInput::Down => self.axis.y = -1.,
                    PlayerInput::Left => self.axis.x = -1.,
                    PlayerInput::Right => self.axis.x = 1.,
                }
                self.pressed
                    .iter()
                    .for_each(|alr_pressed| match (alr_pressed, input) {
                        (PlayerInput::Down, PlayerInput::Up) => self.axis.y = 0.,
                        (PlayerInput::Up, PlayerInput::Down) => self.axis.y = 0.,
                        (PlayerInput::Left, PlayerInput::Right) => self.axis.x = 0.,
                        (PlayerInput::Right, PlayerInput::Left) => self.axis.x = 0.,
                        _ => {}
                    });
                self.just_pressed.insert(input);
            }
        }

        pub fn release(&mut self, input: PlayerInput) {
            if self.pressed.remove(&input) {
                match input {
                    PlayerInput::Up => self.axis.y = 0.,
                    PlayerInput::Down => self.axis.y = 0.,
                    PlayerInput::Left => self.axis.x = 0.,
                    PlayerInput::Right => self.axis.x = 0.,
                }
                self.pressed
                    .iter()
                    .for_each(|alr_pressed| match (alr_pressed, input) {
                        (PlayerInput::Down, PlayerInput::Up) => self.axis.y = -1.,
                        (PlayerInput::Up, PlayerInput::Down) => self.axis.y = 1.,
                        (PlayerInput::Left, PlayerInput::Right) => self.axis.x = -1.,
                        (PlayerInput::Right, PlayerInput::Left) => self.axis.x = 1.,
                        _ => {}
                    });
                self.just_released.insert(input);
            }
        }

        pub fn clear(&mut self) {
            self.just_pressed.clear();
            self.just_released.clear();
        }

        // pub fn pressed(&self, input: PlayerInput) -> bool {
        //     self.pressed.contains(&input)
        // }

        // pub fn just_pressed(&self, input: PlayerInput) -> bool {
        //     self.just_pressed.contains(&input)
        // }

        // pub fn just_released(&self, input: PlayerInput) -> bool {
        //     self.just_released.contains(&input)
        // }
    }

    pub fn convert(input: &KeyCode) -> Option<PlayerInput> {
        match input {
            KeyCode::KeyW => Some(PlayerInput::Up),
            KeyCode::KeyA => Some(PlayerInput::Left),
            KeyCode::KeyS => Some(PlayerInput::Down),
            KeyCode::KeyD => Some(PlayerInput::Right),
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
        transform.translation += time.delta_secs() * actions.axis.extend(0.) * 100.;
    });
}
