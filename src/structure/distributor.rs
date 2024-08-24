use std::{f32::consts::PI, time::Instant};

use bevy::{
    app::{App, Plugin, Startup},
    gizmos,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        mesh::{self, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::hashbrown::HashMap,
};
use bevy_inspector_egui::bevy_egui::systems::InputResources;
use bevy_magic_light_2d::prelude::{LightOccluder2D, OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use hexx::{hex, shapes, Hex};

use crate::{
    components::{Assembler, Distributor, OccupiesTile, ResourceBlob, Structure},
    constants::{self, distributor, z_order, Resource, RESOURCE_INPUTS, SECONDS_PER_TICK},
    terrain::tiles::HEX_LAYOUT,
    utils::{self, find_angle},
};

use super::assembler;

pub struct DistributorPlugin;

impl Plugin for DistributorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_distributors));
    }
}

fn generate_distributors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<OccupiesTile>>,
) {
    for hex in shapes::hexagon(hex(15, -12), 2) {
        spawn_distributor(hex, &mut commands, &asset_server, &query);
    }
}

fn spawn_distributor(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: &Query<&Transform, With<OccupiesTile>>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    // not very efficient
    for transform in query.iter() {
        if transform.translation.truncate() == world_pos {
            return;
        }
    }

    println!("spawning distributor");

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(distributor::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 5.0),
                scale: Vec3::new(0.08, 0.08, 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Structure,
        Distributor {
            resource: constants::Resource::Metal,
            store: HashMap::new(),
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));

    distributor_light(world_pos, commands, constants::distributor::COLOR);
}

fn distributor_light(world_pos: Vec2, commands: &mut Commands, color: Color) {
    commands
        .spawn((
            OmniLightSource2D {
                intensity: 0.2,
                color,
                falloff: Vec3::new(1.5, 10., 0.005),
                ..default()
            },
            Distributor {
                resource: constants::Resource::Coal,
                store: {
                    let mut map = HashMap::new();
                    map.insert(constants::Resource::Coal, 1000);
                    map
                },
            },
        ))
        .insert(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(RenderLayers::all());
}
