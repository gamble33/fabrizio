use fabrizio_rs::physics::*;

use bevy::prelude::*;
use rand::random;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugin))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Mesh::from(shape::UVSphere {
        radius: 5.0,
        ..default()
    }));

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });

    let not_white = materials.add(StandardMaterial {
        base_color: Color::PINK,
        unlit: true,
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: sphere.clone(),
            material: white.clone(),
            ..default()
        })
        .insert(ParticleBundle::from_pos_and_vel(
            Vec2::new(-20.0, 0.0),
            Vec2::new(6.0, 0.0),
        ))
        .insert(Mass(1.0));

    commands
        .spawn(PbrBundle {
            mesh: sphere.clone(),
            material: not_white.clone(),
            ..default()
        })
        .insert(ParticleBundle::from_pos_and_vel(
            Vec2::new(20.0, 0.0),
            Vec2::new(-6.0, 0.0),
        ))
        .insert(Mass(1.0));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        projection: Projection::from(OrthographicProjection {
            scale: 0.1,
            ..default()
        }),
        ..default()
    });
}
