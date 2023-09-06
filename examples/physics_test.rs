use fabrizio_rs::physics::*;

use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins,
      PhysicsPlugin
    ))
    .add_systems(Startup, startup)
    .run();
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let sphere = meshes.add(Mesh::from(shape::UVSphere {
    radius: 0.5,
    ..default()
  }));

  let white = materials.add(StandardMaterial {
    base_color: Color::WHITE,
    unlit: true,
    ..default()
  });

  commands
    .spawn(PbrBundle {
      mesh: sphere.clone(),
      material: white.clone(),
      ..default()
    })
    .insert(ParticleBundle::from_pos_and_vel(Vec2::ZERO, Vec2::new(2.0, 0.0)));

  commands.spawn(Camera3dBundle {
    transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
    projection: Projection::from(OrthographicProjection {
      scale: 0.01,
      ..default()
    }),
    ..default()
  });
}