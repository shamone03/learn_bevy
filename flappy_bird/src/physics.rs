use bevy::{
    math::Vec2,
    prelude::{Component, Query, Res, Transform},
    time::Time,
};

#[derive(Component, Default)]
pub struct PhysicsBody {
    pub velocity: Vec2,
}

const GRAVITY: f32 = 1000.;
pub fn apply_gravity(mut query: Query<&mut PhysicsBody>, time: Res<Time>) {
    query.iter_mut().for_each(|mut pbody| {
        pbody.velocity.y -= GRAVITY * time.delta_secs();
    });
}

pub fn apply_physics(mut query: Query<(&mut Transform, &PhysicsBody)>, time: Res<Time>) {
    query.iter_mut().for_each(|(mut transform, pbody)| {
        transform.translation += (pbody.velocity * time.delta_secs()).extend(0.);
    });
}
