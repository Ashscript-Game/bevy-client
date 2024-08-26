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
    utils::{hashbrown::HashMap, HashSet},
};
use bevy_inspector_egui::bevy_egui::systems::InputResources;
use bevy_magic_light_2d::prelude::{LightOccluder2D, OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use hexx::{hex, shapes, Hex};

use crate::{
    components::{Assembler, Distributor, OccupiesTile, ResourceBlob, Store, Structure},
    constants::{self, distributor, z_order, Resource, RESOURCE_INPUTS, SECONDS_PER_TICK},
    terrain::tiles::HEX_LAYOUT,
    utils::{self, find_angle},
};

use super::assembler;

pub struct DistributorPlugin;

impl Plugin for DistributorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_distributors);
    }
}

fn generate_distributors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<OccupiesTile>>,
) {
    for hex in shapes::hexagon(hex(13, -10), 2) {
        spawn_distributor(hex, &mut commands, &asset_server, &query);
    }
}

fn spawn_distributor(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    occupiers: &Query<&Transform, With<OccupiesTile>>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    // not very efficient
    for transform in occupiers.iter() {
        if transform.translation.truncate() == world_pos {
            return;
        }
    }

    println!("spawning distributor");

    let resource_options = vec![Resource::Coal, Resource::Minerals, Resource::Metal];
    let resource = utils::pick(&resource_options);

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
            resource: *resource,
            store: Store {
                resources: {
                    let mut map = HashMap::new();
                    map.insert(*resource, 1000);
                    map
                },
                allowed_inputs: Some(HashSet::from([*resource])),
                capacity: 1000,
            },
        },
        OmniLightSource2D {
            intensity: 0.2,
            color: constants::distributor::COLOR,
            falloff: Vec3::new(1.5, 10., 0.005),
            ..default()
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));

    /* distributor_light(world_pos, commands, constants::distributor::COLOR); */
}

/* fn distributor_light(world_pos: Vec2, commands: &mut Commands, color: Color) {
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
 */
