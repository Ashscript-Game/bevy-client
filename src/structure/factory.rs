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
    components::{Assembler, Distributor, Factory, OccupiesTile, ResourceBlob, Store, Structure},
    constants::{self, distributor, factory, z_order, Resource, RESOURCE_INPUTS, SECONDS_PER_TICK},
    engine::terrain::HEX_LAYOUT,
    utils::{self, find_angle_coords},
};

pub fn spawn_factory(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(factory::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 5.0),
                scale: Vec3::new(0.08, 0.08, 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Factory {
            tick_last_produced: 0,
            store: Store {
                resources: {
                    let mut map = HashMap::new();
                    map.insert(Resource::Metal, 1000);
                    map
                },
                allowed_inputs: Some(HashSet::from([Resource::Metal])),
                capacity: 1000,
            },
        },
        OmniLightSource2D {
            intensity: 0.2,
            color: factory::COLOR,
            falloff: Vec3::new(4., 4., 0.005),
            ..default()
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}