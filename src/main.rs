use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;



fn main() {
  App::new()
    .add_plugins(
      (
        DefaultPlugins
        .set(ImagePlugin::default_nearest()),
        PlayerPlugin,
      )
    )
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let mut camera = Camera2dBundle {
    camera_2d: Camera2d {
      clear_color: ClearColorConfig::Custom(Color::BEIGE)
    },
    ..default()
  };

  camera.projection.scaling_mode = ScalingMode::AutoMin {
    min_width: 256.0,
    min_height: 144.0
  };

  commands.spawn(camera);

  let texture = asset_server.load("player.png");

  commands.spawn((
    SpriteBundle {
      texture,
      ..default()
    },
  ));
}

