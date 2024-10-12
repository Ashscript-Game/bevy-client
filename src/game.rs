use ashscript_solis_2d::prelude::RadianceCameraBundle;
use bevy::{
    app::{App, Plugin, Startup}, core_pipeline::tonemapping::Tonemapping, prelude::*, render::{camera::RenderTarget, view::RenderLayers}, utils::hashbrown::HashMap
};

use crate::{
    components::{OccupyStructuresMap, UnitMap}, constants::{self}, controls::plugin::ControlsPlugin, debug::plugin::DebugPlugin, engine::plugin::EnginePlugin, projectile::plugin::ProjectilePlugin, structure::plugin::StructuresPlugin, unit::plugin::UnitPlugin
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ControlsPlugin,
            DebugPlugin,
            /* PlayerScriptPlugin, */
            EnginePlugin,
            ProjectilePlugin,
            UnitPlugin,
            StructuresPlugin,
        ))
        .add_systems(Startup, (game_init, spawn_unit_map, spawn_structures_map));
    }
}

fn game_init(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());

    let projection: OrthographicProjection = OrthographicProjection {
        scale: constants::camera::MIN_SCALE,
        // near: -2000.0,
        // far: 2000.0,
        near: -1000.0,
        far: 1000.0,
        ..default()
    };

    commands.spawn(RadianceCameraBundle {
        camera_bundle: Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera: Camera {
                clear_color: Color::BLACK.into(),
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::AcesFitted,
            ..default()
        },
        ..default()
    });
}

fn spawn_unit_map(mut commands: Commands) {

    commands.spawn(UnitMap(HashMap::new()));
}

fn spawn_structures_map(mut commands: Commands) {
    commands.spawn(OccupyStructuresMap(HashMap::new()));
}