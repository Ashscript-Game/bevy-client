use std::f32::consts::PI;

use bevy::{
    math::{Quat, Vec3}, prelude::*, render::view::RenderLayers, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, utils::HashMap
};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};

use crate::{
    components::{ResourceBlob, Store}, constants::{self, coal_node, z_order, GeneralResult, Resource, RESOURCE_INPUTS}, engine::terrain::HEX_LAYOUT, utils::find_angle_coords
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