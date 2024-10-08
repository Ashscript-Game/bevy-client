use std::f32::consts::PI;

use bevy::{math::Quat, prelude::Transform};

use crate::{components::{Turret, Unit}, constants::GeneralResult, utils::find_angle};

use super::terrain::HEX_LAYOUT;

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
