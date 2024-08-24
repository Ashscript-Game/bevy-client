use bevy::{
    app::{App, Plugin, Startup},
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
};
use bevy_magic_light_2d::{
    prelude::{CameraTargets, CAMERA_LAYER_FLOOR, CAMERA_LAYER_OBJECTS, CAMERA_LAYER_WALLS},
    FloorCamera, ObjectsCamera, SpriteCamera, WallsCamera,
};

use crate::{
    components::ResourceBlob, constants::{self, resource_blob, SECONDS_PER_TICK}, controls::{camera::CameraControlsPlugin, plugin::ControlsPlugin}, debug::plugin::DebugPlugin, lighting::plugin::LightingPlugin, player_script::plugin::PlayerScriptPlugin, structure::plugin::StructuresPlugin, terrain::{plugin::TerrainPlugin, tiles::TilePlugin}, utils::signed_distance
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TerrainPlugin,
            ControlsPlugin,
            LightingPlugin,
            StructuresPlugin,
            DebugPlugin,
            PlayerScriptPlugin,
        ))
        .add_systems(Startup, game_init)
        .add_systems(Update, update_resource_blobs);
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
        Name::new("obejcts_camera"),
        ObjectsCamera,
        SpriteCamera,
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}

fn update_resource_blobs(
    mut resource_blobs: Query<(&mut Transform, &ResourceBlob, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {

    for (mut blob_transform, blob, entity) in resource_blobs.iter_mut() {

        // the initial sign is important to detect which way we pass the target, negative or positive 

        let horizontal_sign = (blob.target_pos.x - blob.start_pos.x).signum();
        let vertical_sign = (blob.target_pos.y - blob.start_pos.y).signum();
        
        // if we have passed or reached the target, despawn the blob
        if (blob_transform.translation.x - blob.target_pos.x) * horizontal_sign >= 0.
            && (blob_transform.translation.y - blob.target_pos.y) * vertical_sign >= 0.
        {
            println!("despawning resource blob");
            commands.entity(entity).despawn();
            continue;
        }

        // translate the position of the blob to move linearly (relative x to y) towards the target
        // this should move the blob at a constant time of SECONDS_PER_TICK, no matter the distance or tick rate, it should reach the destination at the speed of the tick rate

        let distance = signed_distance(blob.start_pos, blob.target_pos);

        let direction = blob_transform.rotation * Vec3::Y;
        let translation_delta = distance / SECONDS_PER_TICK * time.delta_seconds() * direction;

        blob_transform.translation += translation_delta;
    }
}
