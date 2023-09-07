use crate::physics::*;

use bevy::math::Vec2;
use bevy::prelude::{Bundle, default};

#[derive(Bundle, Default)]
pub struct ParticleBundle {
  pub pos: Pos,
  pub pos_prev: PosPrev,
  pub mass: Mass,
  pub collider: CircleCollider,
  pub vel: Vel,
  pub vel_pre_solve: VelPreSolve,
  pub restitution: Restitution,
}

impl ParticleBundle {
  pub fn from_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
    Self {
      pos: Pos(pos),
      pos_prev: PosPrev(pos - vel * DELTA_TIME),
      vel: Vel(vel),
      ..default()
    }
  }
}