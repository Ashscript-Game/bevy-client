use std::f32::consts::PI;

use bevy::{math::Quat, prelude::*};

use crate::{components::{State, Turret, Unit}, constants::GeneralResult, structure::turret::spawn_turret, utils::find_angle};

use super::terrain::HEX_LAYOUT;

pub fn generate_turrets_from_keyframe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    for (_, chunk) in state.map.chunks.iter() {
        for (hex, turret) in chunk.turrets.iter() {
            spawn_turret(*hex, &mut commands, &asset_server, turret.owner_id);
        }
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
        return GeneralResult::Fail
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
