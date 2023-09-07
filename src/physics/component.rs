use bevy::prelude::{Component, Vec2};

#[derive(Component, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Default)]
pub struct PosPrev(pub Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Component, Default)]
pub struct Vel(pub Vec2);

#[derive(Component, Default)]
pub struct VelPreSolve(pub Vec2);

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 5.0 }
    }
}
