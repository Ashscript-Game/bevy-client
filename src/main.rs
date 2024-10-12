
use ashscript_solis_2d::LightPlugin;
use bevy::{
    app::App, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, utils::hashbrown::HashMap, DefaultPlugins
};
use components::{GameSettings, GameState, PlayerStates, ProjectileMoveEndTimer};
use constants::{PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK};
use game::GamePlugin;
use rust_socketio::{ClientBuilder, Payload, Socket, RawClient};

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
pub mod ai_scripts;
pub mod types;

fn main() {
    /* test_socket(); */

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
            FrameTimeDiagnosticsPlugin,
            LightPlugin,
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
        .insert_resource(GameSettings {
            lights: true,
        })
        .insert_resource(GameState::new())
        .insert_resource(PlayerStates(HashMap::new()))
        .run();
}

fn test_socket() {
    // get a socket that is connected to the admin namespace
    let socket = ClientBuilder::new("http://localhost:3000")
        .namespace("/client")
        .on("keyframe", keyframe_callback)
        .on("action", action_callback)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");
}

fn keyframe_callback(payload: Payload, raw_client: RawClient) {
    println!("received keyframe: {:?}", payload);
}

fn action_callback(payload: Payload, raw_client: RawClient) {
    println!("received action: {:?}", payload);
}