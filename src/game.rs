use bevy::{
    app::{App, Plugin, Startup}, core_pipeline::bloom::BloomSettings, prelude::*, render::{camera::RenderTarget, view::RenderLayers}
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

fn game_init(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());

    let projection: OrthographicProjection = OrthographicProjection {
        scale: constants::camera::MIN_SCALE,
        near: -1000.,
        far: 1000.,
        ..default()
    };

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            projection,
            ..Default::default()
        },
        /* BloomSettings::NATURAL, */
        Name::new("main_camera"),
    ));
}
