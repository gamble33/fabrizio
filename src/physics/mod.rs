/// https://johanhelsing.studio/posts/physics-01-balls
/// https://github.com/Jondolf/bevy_xpbd/blob/main/src/plugins/integrator.rs
/// https://matthias-research.github.io/pages/publications/PBDBodies.pdf
/// https://www.youtube.com/watch?v=jrociOAYqxA
mod component;
mod particle;

pub use component::*;
pub use particle::*;

use bevy::app::{App, FixedUpdate, Plugin};
use bevy::math::Vec2;
use bevy::prelude::{
    Entity, IntoSystemConfigs, Query, Res, ResMut, Resource, Transform,
};

pub const DELTA_TIME: f32 = 1.0 / 60.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
            .init_resource::<Contacts>()
            .add_systems(
                FixedUpdate,
                (
                    collect_collision_pairs,
                    integrate.after(collect_collision_pairs),
                    solve_pos.after(integrate),
                    update_vel.after(solve_pos),
                    solve_vel.after(update_vel),
                    sync_transforms.after(update_vel),
                ),
            );
    }
}

#[derive(Resource)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0.0, -9.81))
    }
}

#[derive(Resource, Default)]
pub struct Contacts(pub Vec<(Entity, Entity)>);

fn collect_collision_pairs() {}

fn integrate(
    mut query: Query<(
        &mut Pos,
        &mut PosPrev,
        &mut Vel,
        &mut VelPreSolve,
        &Mass,
    )>,
    gravity: Res<Gravity>,
) {
    for (mut pos, mut pos_prev, mut vel, mut vel_pre_solve, mass) in
        query.iter_mut()
    {
        let weight = mass.0 * gravity.0;
        let external_forces = weight;

        pos_prev.0 = pos.0;
        vel.0 += external_forces / mass.0 * DELTA_TIME;
        pos.0 += vel.0 * DELTA_TIME;

        vel_pre_solve.0 = vel.0;
    }
}

fn solve_pos(
    mut query: Query<(Entity, &mut Pos, &Mass, &CircleCollider)>,
    mut contacts: ResMut<Contacts>,
) {
    contacts.0.clear();
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [(entity_a, mut pos_a, mass_a, circle_a), (entity_b, mut pos_b, mass_b, circle_b)],
    ) = iter.fetch_next()
    {
        let ab = pos_b.0 - pos_a.0;
        let dist_sqr = ab.length_squared();
        let combined_radius = circle_a.radius + circle_b.radius;
        if dist_sqr < combined_radius * combined_radius {
            contacts.0.push((entity_a, entity_b));

            let ab_length = dist_sqr.sqrt();
            let penetration_depth = combined_radius - ab_length;
            let n = ab / ab_length;

            let w_a = 1.0 / mass_a.0;
            let w_b = 1.0 / mass_b.0;
            let w_sum = w_a + w_b;

            pos_a.0 -= w_a / w_sum * n * penetration_depth;
            pos_b.0 += w_b / w_sum * n * penetration_depth;
        }
    }
}

fn update_vel(mut query: Query<(&Pos, &PosPrev, &mut Vel)>) {
    for (pos, pos_prev, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - pos_prev.0) / DELTA_TIME;
    }
}

fn solve_vel(
    mut query: Query<(&mut Vel, &VelPreSolve, &Pos, &Mass, &Restitution)>,
    contacts: Res<Contacts>,
) {
    for (entity_a, entity_b) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, vel_pre_solve_a, pos_a, mass_a, restitution_a),
            (mut vel_b, vel_pre_solve_b, pos_b, mass_b, restitution_b),
        ) = unsafe {
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };

        let n = (pos_b.0 - pos_a.0).normalize();
        let vel_relative_pre_solve = vel_pre_solve_a.0 - vel_pre_solve_b.0;
        let vel_normal_pre_solve = vel_relative_pre_solve.dot(n);

        let vel_relative = vel_a.0 - vel_b.0;
        let vel_normal = vel_relative.dot(n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.0;

        let w_a = 1.0 / mass_a.0;
        let w_b = 1.0 / mass_b.0;
        let w_sum = w_a + w_b;

        vel_a.0 += n * (-vel_normal - restitution * vel_normal_pre_solve) * w_a
            / w_sum;
        vel_b.0 -= n * (-vel_normal - restitution * vel_normal_pre_solve) * w_b
            / w_sum;
    }
}

fn sync_transforms(mut query: Query<(&Pos, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = pos.0.extend(0.0);
    }
}
