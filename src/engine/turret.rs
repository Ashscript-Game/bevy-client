use bevy::prelude::Transform;

use crate::{components::{Turret, Unit}, constants::GeneralResult};

use super::terrain::HEX_LAYOUT;

pub fn turret_attack_cost(turret: &Turret) -> u32 {
    turret.range + turret.damage
}

pub fn turret_attack(
    turret: &mut Turret,
    turret_transform: &Transform,
    unit: &mut Unit,
    unit_transform: &Transform,
) -> GeneralResult {
    let turret_hex = HEX_LAYOUT.world_pos_to_hex(turret_transform.translation.truncate());
    let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());

    if turret_hex == unit_hex {
        return GeneralResult::Fail;
    }

    let distance = turret_hex.unsigned_distance_to(unit_hex);
    if distance > turret.range {
        return GeneralResult::Fail;
    }

    unit.health = (unit.health - turret.damage).max(0);
    turret.energy -= turret_attack_cost(turret);

    GeneralResult::Success
}
