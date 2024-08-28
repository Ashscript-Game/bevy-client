use bevy::{
    app::{App, Plugin, Startup}, core_pipeline::bloom::BloomSettings, prelude::*, render::{camera::RenderTarget, view::RenderLayers}
};
use bevy_magic_light_2d::{
    prelude::{CameraTargets, CAMERA_LAYER_FLOOR, CAMERA_LAYER_OBJECTS, CAMERA_LAYER_WALLS},
    FloorCamera, ObjectsCamera, SpriteCamera, WallsCamera,
};

use crate::{
    components::ResourceBlob, constants::{self, resource_blob, SECONDS_PER_TICK}, controls::{camera::CameraControlsPlugin, plugin::ControlsPlugin}, debug::plugin::DebugPlugin, engine::plugin::EnginePlugin, lighting::plugin::LightingPlugin, player_script::plugin::PlayerScriptPlugin, projectile::plugin::ProjectilePlugin, structure::plugin::StructuresPlugin, unit::plugin::UnitPlugin, utils::signed_distance
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ControlsPlugin,
            LightingPlugin,
            DebugPlugin,
            PlayerScriptPlugin,
            EnginePlugin,
            ProjectilePlugin,
            UnitPlugin,
            StructuresPlugin,
        ))
        .add_systems(Startup, game_init);
    }
}

fn game_init(mut commands: Commands, camera_targets: Res<CameraTargets>) {
    // commands.spawn(Camera2dBundle::default());

    let projection: OrthographicProjection = OrthographicProjection {
        scale: constants::camera::MIN_SCALE,
        near: -2000.0,
        far: 2000.0,
        ..default()
    };

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                target: RenderTarget::Image(camera_targets.floor_target.clone()),
                ..Default::default()
            },
            projection: projection.clone(),
            ..Default::default()
        },
        Name::new("floor_camera"),
        FloorCamera,
        SpriteCamera,
        RenderLayers::from_layers(CAMERA_LAYER_FLOOR),
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                target: RenderTarget::Image(camera_targets.walls_target.clone()),
                ..Default::default()
            },
            projection: projection.clone(),
            ..Default::default()
        },
        Name::new("walls_camera"),
        WallsCamera,
        SpriteCamera,
        RenderLayers::from_layers(CAMERA_LAYER_WALLS),
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                target: RenderTarget::Image(camera_targets.objects_target.clone()),
                ..Default::default()
            },
            projection: projection.clone(),
            ..Default::default()
        },
        /* BloomSettings::NATURAL, */
        Name::new("obejcts_camera"),
        ObjectsCamera,
        SpriteCamera,
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}
