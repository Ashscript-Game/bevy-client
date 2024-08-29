use std::time::Duration;

use bevy::{
    app::{App, Startup}, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, utils::hashbrown::HashMap, DefaultPlugins
};
use bevy_magic_light_2d::{gi::BevyMagicLight2DPlugin, prelude::*};
use components::{GameSettings, ProjectileMoveEndTimer, UnitMap};
use constants::{PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK};
use game::GamePlugin;

pub mod components;
pub mod constants;
pub mod controls;
pub mod debug;
pub mod engine;
pub mod game;
pub mod lighting;
pub mod player_script;
pub mod prelude;
pub mod projectile;
pub mod structure;
pub mod unit;
pub mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgba(0., 0., 0., 0.)))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Scripter".into(),
                        ..default()
                    }),
                    ..default()
                }),
            GamePlugin,
            BevyMagicLight2DPlugin,
            FrameTimeDiagnosticsPlugin,
            /* LogDiagnosticsPlugin {
                debug: false,
                wait_duration: Duration::from_secs(1),
                filter: None,
            }, */
        ))
        .insert_resource(BevyMagicLight2DSettings {
            light_pass_params: LightPassParams {
                reservoir_size: 16,
                smooth_kernel_size: (2, 1),
                direct_light_contrib: 0.2,
                indirect_light_contrib: 0.8,
                ..default()
            },
            ..default()
        })
        .insert_resource(ProjectileMoveEndTimer(Timer::from_seconds(
            SECONDS_PER_TICK * PROJECTILE_MOVE_END_TICK_PORTION,
            TimerMode::Once,
        )))
        .insert_resource(GameSettings {
            lights: true,
        })
        .insert_resource(UnitMap(HashMap::new()))
        .register_type::<LightOccluder2D>()
        .register_type::<OmniLightSource2D>()
        .register_type::<SkylightMask2D>()
        .register_type::<SkylightLight2D>()
        .register_type::<BevyMagicLight2DSettings>()
        .register_type::<LightPassParams>()
        .run();
}
