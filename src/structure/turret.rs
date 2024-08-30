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
    components::{Assembler, Distributor, OccupiesTile, ResourceBlob, Store, Structure, Turret},
    constants::{self, distributor, turret, z_order, Resource, RESOURCE_INPUTS, SECONDS_PER_TICK},
    engine::terrain::HEX_LAYOUT,
    utils::{self, find_angle_coords},
};

use super::assembler;

pub struct DistributorPlugin;

impl Plugin for DistributorPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

pub fn spawn_turret(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(turret::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 5.0),
                scale: Vec3::new(1., 1., 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Turret {
            range: 6,
            damage: 2,
            energy_gen: 60,
            ..default()
        },
        OmniLightSource2D {
            intensity: 0.2,
            color: constants::turret::COLOR,
            falloff: Vec3::new(4., 4., 0.005),
            ..default()
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}