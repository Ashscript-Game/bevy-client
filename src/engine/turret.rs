use std::f32::consts::PI;

use ashscript_types::{components::{owner::Owner, tile::Tile}, constants::map::{CHUNK_SIZE, HEX_LAYOUT}};
use bevy::{math::Quat, prelude::*};

use crate::{
    components::{LoadChunks, State, Turret, Unit},
    constants::GeneralResult,
    structure::turret::spawn_turret,
    utils::find_angle,
};

pub fn generate_turrets_from_keyframe(
    trigger: Trigger<LoadChunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    let new_chunks = &trigger.event().0;

    for (_, (_, tile, owner)) in state
        .world
        .query::<((&ashscript_types::components::turret::Turret, &Tile, &Owner))>()
        .iter()
    {
        println!("turret");

        if !new_chunks.contains(&tile.hex.to_lower_res(CHUNK_SIZE)) {
            continue;
        }

        spawn_turret(tile.hex, &mut commands, &asset_server, owner.0);
    }
}

pub fn turret_attack_cost(turret: &Turret) -> u32 {
    turret.range + turret.damage
}

pub fn turret_attack(
    turret: &mut Turret,
    turret_transform: &mut Transform,
    unit: &mut Unit,
    unit_transform: &Transform,
) -> GeneralResult {
    if turret.energy < turret_attack_cost(turret) {
        return GeneralResult::Fail;
    }

    let turret_hex = HEX_LAYOUT.world_pos_to_hex(turret_transform.translation.truncate());
    let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());

    if turret_hex == unit_hex {
        return GeneralResult::Fail;
    }

    let distance = turret_hex.unsigned_distance_to(unit_hex);
    if distance > turret.range {
        return GeneralResult::Fail;
    }

    if turret.damage > unit.health {
        unit.health = 0
    } else {
        unit.health -= turret.damage
    }

    let angle = find_angle(&turret_transform.translation, &unit_transform.translation) + PI / 2.;
    turret_transform.rotation = Quat::from_rotation_z(angle);

    turret.energy -= turret_attack_cost(turret);

    GeneralResult::Success
}
