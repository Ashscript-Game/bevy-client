use ashscript_types::constants::map::HEX_LAYOUT;
use bevy::{
    app::{App, Plugin}, prelude::*, render::view::RenderLayers, utils::hashbrown::HashMap
};
use bevy_magic_light_2d::prelude::CAMERA_LAYER_OBJECTS;
use uuid::Uuid;

use crate::{
    components::{Assembler, OccupiesTile, Owner, Store},
    constants::{self, assembler, Resource, RESOURCE_INPUTS},
};

pub struct AssemblerPlugin;

impl Plugin for AssemblerPlugin {
    fn build(&self, _app: &mut App) {
        //app;
    }
}

pub fn spawn_assembler(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    owner_id: Uuid,
) {

    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(assembler::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 1.0),
                scale: Vec3::new(1.2, 1.2, 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Assembler {
            output_resource: constants::Resource::Metal,
            store: Store {
                resources: HashMap::new(),
                allowed_inputs: Some(RESOURCE_INPUTS[Resource::Metal].clone()),
                capacity: 1000,
            },
            transferring: None
        },
        Owner(owner_id),
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS)
    ));
}

pub fn assembler_produce(assembler: &mut Assembler) {
    let output_resource = assembler.output_resource;

    // Ensure we have a positive amount of input resources

    for input_resource in RESOURCE_INPUTS[output_resource].iter() {

        let Some(input_amount) = assembler.store.resources.get(input_resource) else {
            return;
        };

        if *input_amount == 0 {
            return;
        }
    }

    // transform 1 of each input resource into output resource

    for input_resource in RESOURCE_INPUTS[output_resource].iter() {
        *assembler.store.resources.entry(*input_resource).or_insert(0) -= 1;
    }

    *assembler.store.resources.entry(output_resource).or_insert(0) += 1;
}

fn delete_assembler_if_not_in_keyframe() {
    // 
}