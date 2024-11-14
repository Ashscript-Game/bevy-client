// TODO: remove these eventually, once there's less warnings in between the actual
// error messages.
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::net::{SocketAddr, UdpSocket};

use ashscript_types::{actions::ActionsByKind, global::Global, map::Map};
use bevy::{
    app::App, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, tasks::TaskPoolBuilder,
    utils::hashbrown::HashMap, DefaultPlugins,
};
use bevy_eventwork::EventworkRuntime;
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};
use bevy_magic_light_2d::{gi::BevyMagicLight2DPlugin, prelude::*};
use components::{
    Actions, DebugUI, GameSettings, GameState, LoadedChunks, PlayerStates, ProjectileMoveEndTimer, SelectedGameObjects, State, UnloadedChunks
};
use constants::{PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK};
use game::GamePlugin;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod constants;
pub mod controls;
pub mod debug;
pub mod engine;
pub mod game;
pub mod lighting;
pub mod networker;
pub mod prelude;
pub mod projectile;
pub mod structure;
pub mod types;
pub mod unit;
pub mod utils;
pub mod ui;

fn main() {
    let network_info = networker::create_network_resource();

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
            bevy_egui::EguiPlugin,
            FrameTimeDiagnosticsPlugin,
            /* LogDiagnosticsPlugin {
                debug: false,
                wait_duration: Duration::from_secs(1),
                filter: None,
            }, */
        ))
        .insert_resource(BevyMagicLight2DSettings {
            light_pass_params: LightPassParams {
                reservoir_size: 1, /* 16 */
                smooth_kernel_size: (3, 3),
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
        .insert_resource(GameSettings { lights: true })
        .insert_resource(State {
            map: Map::new(),
            global: Global::new(),
            world: hecs::World::new(),
        })
        .insert_resource(Actions(ActionsByKind::new()))
        .insert_resource(GameState::new())
        .insert_resource(PlayerStates(HashMap::new()))
        .insert_resource(network_info)
        .register_type::<LightOccluder2D>()
        .register_type::<OmniLightSource2D>()
        .register_type::<SkylightMask2D>()
        .register_type::<SkylightLight2D>()
        .register_type::<BevyMagicLight2DSettings>()
        .register_type::<LightPassParams>()
        .insert_resource(NetworkSettings::default())
        .insert_resource(LoadedChunks::default())
        .insert_resource(UnloadedChunks::default())
        .insert_resource(SelectedGameObjects::default())
        .insert_resource(DebugUI::default())
        .run();
}
