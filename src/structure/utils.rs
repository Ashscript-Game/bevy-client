use std::f32::consts::PI;

use bevy::{
    math::{Quat, Vec3}, prelude::*, render::view::RenderLayers, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, utils::HashMap
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};

use crate::{
    components::{ResourceBlob, Store},
    constants::{self, coal_node, z_order, GeneralResult, Resource, RESOURCE_INPUTS},
    terrain::tiles::HEX_LAYOUT, utils::find_angle,
};

pub fn transfer(
    out_pos: &Vec3,
    out_store: &mut Store,
    in_pos: &Vec3,
    in_store: &mut Store,
    resource: &Resource,
    amount: u32,
) -> GeneralResult {

    // check amount

    let clamped_amount = amount.min(in_store.capacity);
    if clamped_amount == 0 {
        return GeneralResult::Fail;
    }

    // check distance

    let in_hex = HEX_LAYOUT.world_pos_to_hex(out_pos.truncate());
    let out_hex = HEX_LAYOUT.world_pos_to_hex(in_pos.truncate());

    let distance = in_hex.unsigned_distance_to(out_hex);
    if distance > constants::distributor::RANGE {
        return GeneralResult::Fail;
    }

    // check allowed inputs

    if let Some(allowed_inputs) = &in_store.allowed_inputs {
        if !allowed_inputs.contains(resource) {
            return GeneralResult::Fail;
        }
    }

    *in_store.resources.entry(*resource).or_insert(0) += clamped_amount;
    *out_store.resources.entry(*resource).or_insert(0) -= clamped_amount;

    /* *in_store.resources.get_mut(resource).unwrap_or(&mut 0) += clamped_amount;
    *out_store.resources.get_mut(resource).unwrap_or(&mut 0) -= clamped_amount; */

    GeneralResult::Success
}

pub fn create_resource_blob(
    start_pos: &Vec3,
    target_pos: &Vec3,
    resource: &Resource,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    /* println!("creating resource blob {:?}", resource); */

    let mesh = Mesh2dHandle(meshes.add(Circle::new(10.)));

    let angle = find_angle(
        start_pos.x,
        start_pos.y,
        target_pos.x,
        target_pos.y,
    ) + PI / 2.;
    
    let color_resource_map = HashMap::from([(Resource::Coal, coal_node::COLOR), (Resource::Minerals, constants::mineral_node::COLOR), (Resource::Metal, constants::metal::COLOR)]);
    let color = *color_resource_map.get(resource).unwrap_or(&constants::scrap::COLOR);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(
                    start_pos.x,
                    start_pos.y,
                    z_order::PROJECTILE,
                ),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            ..default()
        },
        OmniLightSource2D {
            intensity: 0.3,
            color: Color::WHITE,
            falloff: Vec3::new(10., 10., 0.005),
            ..Default::default()
        },
        ResourceBlob {
            resource: *resource,
            target_pos: *target_pos,
            start_pos: *start_pos,
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));
}
