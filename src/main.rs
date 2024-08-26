use std::time::Duration;

use bevy::{
    app::{App, Startup},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_magic_light_2d::{gi::BevyMagicLight2DPlugin, prelude::*};
use game::GamePlugin;

pub mod components;
pub mod constants;
pub mod controls;
pub mod game;
pub mod lighting;
pub mod terrain;
pub mod structure;
pub mod utils;
pub mod debug;
pub mod player_script;
pub mod engine;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba_u8(0, 0, 0, 0)))
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
        .register_type::<LightOccluder2D>()
        .register_type::<OmniLightSource2D>()
        .register_type::<SkylightMask2D>()
        .register_type::<SkylightLight2D>()
        .register_type::<BevyMagicLight2DSettings>()
        .register_type::<LightPassParams>()
        .run();
}
