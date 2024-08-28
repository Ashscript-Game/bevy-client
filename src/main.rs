use std::time::Duration;

use bevy::{
    app::{App, Startup},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_light_2d::plugin::Light2dPlugin;
use components::ProjectileMoveEndTimer;
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
        .add_plugins((
            DefaultPlugins
                /* .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }) */
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Scripter".into(),
                        ..default()
                    }),
                    ..default()
                }),
            GamePlugin,
            Light2dPlugin,
            FrameTimeDiagnosticsPlugin,
            /* LogDiagnosticsPlugin {
                debug: false,
                wait_duration: Duration::from_secs(1),
                filter: None,
            }, */
        ))
        .insert_resource(ProjectileMoveEndTimer(Timer::from_seconds(
            SECONDS_PER_TICK * PROJECTILE_MOVE_END_TICK_PORTION,
            TimerMode::Once,
        )))
        .run();
}
