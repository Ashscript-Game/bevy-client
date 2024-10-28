
use std::net::{SocketAddr, UdpSocket};

use ashscript_types::{actions::ActionsByKind, global::Global, map::Map};
use bevy::{
    app::App, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, utils::hashbrown::HashMap, DefaultPlugins
};
use bevy_magic_light_2d::{gi::BevyMagicLight2DPlugin, prelude::*};
use bevy_simple_networking::{ClientPlugin, SocketAddrResource, UdpSocketResource};
use components::{Actions, DebugSettings, GameSettings, GameState, PlayerStates, ProjectileMoveEndTimer, State};
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
pub mod ai_scripts;
pub mod types;
pub mod networker;

fn main() {

    let address: SocketAddr = "127.0.0.1:3000".parse().expect("could not parse socket address");
    let socket = UdpSocket::bind("[::]:0").expect("unable to bind socket");

    socket.connect(address).expect("unable to connect to server");
    socket.set_nonblocking(true)
    .expect("unable to set socket to nonblocking");

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
            ClientPlugin,
            BevyMagicLight2DPlugin,
            bevy_egui::EguiPlugin,
            FrameTimeDiagnosticsPlugin,
            /* LogDiagnosticsPlugin {
                debug: false,
                wait_duration: Duration::from_secs(1),
                filter: None,
            }, */
        ))
        .insert_resource(SocketAddrResource::new(address))
        .insert_resource(UdpSocketResource::new(socket))
        .insert_resource(BevyMagicLight2DSettings {
            light_pass_params: LightPassParams {
                reservoir_size: 1/* 16 */,
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
        .insert_resource(GameSettings {
            lights: true,
        })
        .insert_resource(DebugSettings {
            hightlight_chunks: false,
        })
        .insert_resource(State {
            map: Map::new(),
            global: Global::new(),
        })
        .insert_resource(Actions(ActionsByKind::new()))
        .insert_resource(GameState::new())
        .insert_resource(PlayerStates(HashMap::new()))
        .register_type::<LightOccluder2D>()
        .register_type::<OmniLightSource2D>()
        .register_type::<SkylightMask2D>()
        .register_type::<SkylightLight2D>()
        .register_type::<BevyMagicLight2DSettings>()
        .register_type::<LightPassParams>()
        .run();
}