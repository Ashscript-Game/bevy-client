use bevy::{
    app::{App, Plugin, Startup},
    prelude::*, render::camera::RenderTarget,
};
use bevy_magic_light_2d::{prelude::CameraTargets, FloorCamera, SpriteCamera};

use crate::{
    controls::{camera::CameraControlsPlugin, plugin::ControlsPlugin}, lighting::plugin::LightingPlugin, terrain::{plugin::TerrainPlugin, tiles::TilePlugin}
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TerrainPlugin, ControlsPlugin, LightingPlugin))
            .add_systems(Startup, game_init);
    }
}

fn game_init(mut commands: Commands, camera_targets: Res<CameraTargets>) {
    // commands.spawn(Camera2dBundle::default());

    commands
    .spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                target: RenderTarget::Image(camera_targets.floor_target.clone()),
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("main_camera"),
        FloorCamera,
    ))
    .insert(SpriteCamera);
}
