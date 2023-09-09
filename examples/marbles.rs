use std::time::Duration;

use fabrizio_rs::physics::*;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::random;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugin))
        .add_systems(Startup, startup)
        .add_systems(
            FixedUpdate,
            (
                spawn_particle.run_if(on_timer(Duration::from_millis(200))),
                despawn_marbles.run_if(on_timer(Duration::from_millis(200))),
            ),
        )
        .run();
}

#[derive(Resource)]
struct Materials {
    blue: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct Meshes {
    sphere: Handle<Mesh>,
}

fn spawn_particle(
    mut commands: Commands,
    meshes: Res<Meshes>,
    materials: Res<Materials>,
) {
    let radius = 0.5;
    let pos = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * 0.5
        + Vec2::Y * 3.;
    let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
    commands
        .spawn(PbrBundle {
            mesh: meshes.sphere.clone(),
            material: materials.blue.clone(),
            transform: Transform {
                scale: Vec3::splat(radius),
                translation: pos.extend(0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ParticleBundle {
            collider: CircleCollider { radius },
            ..ParticleBundle::from_pos_and_vel(pos, vel)
        });
}

fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Pos)>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -20.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Meshes {
        sphere: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.,
            ..default()
        })),
    });

    commands.insert_resource(Materials {
        blue: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.4, 0.6),
            unlit: true,
            ..Default::default()
        }),
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        projection: Projection::from(OrthographicProjection {
            scale: 0.1,
            ..default()
        }),
        ..default()
    });
}
