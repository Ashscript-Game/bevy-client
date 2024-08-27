use std::f32::consts::PI;

use bevy::{prelude::*, transform::commands};

use crate::{
    components::{Moving, Unit},
    constants::{self, GeneralResult, UnitPart, UNIT_PART_WEIGHTS},
    utils::find_angle,
};

use super::terrain::HEX_LAYOUT;

pub fn age_units(mut units: Query<&mut Unit>) {
    for mut unit in units.iter_mut() {
        unit.age += 1;
    }
}

pub fn energize_units(mut units: Query<&mut Unit>) {
    for mut unit in units.iter_mut() {
        unit.energy += unit.body[UnitPart::Generate];
    }
}

pub fn kill_units(units: Query<(&Unit, Entity)>, mut commands: Commands) {
    for (unit, entity) in units.iter() {
        if unit.age > constants::unit::MAX_AGE {
            commands.entity(entity).despawn();
            continue;
        }

        if unit.health == 0 {
            commands.entity(entity).despawn();
            continue;
        }
    }
}

pub fn unit_range(unit: &Unit) -> u32 {
    unit.body[UnitPart::Ranged]
}

pub fn unit_damage(unit: &Unit) -> u32 {
    unit.body[UnitPart::Ranged]
}

pub fn unit_attack_cost(unit: &Unit) -> u32 {
    unit.body[UnitPart::Ranged]
}

pub fn unit_weight(unit: &Unit) -> u32 {
    let mut weight: u32 = 0;

    for (body_part, part_amount) in unit.body.iter() {
        weight += UNIT_PART_WEIGHTS[body_part] * part_amount;
    }

    weight
}

pub fn unit_move_cost(unit: &Unit) -> u32 {
    unit.weight / 10
}

pub fn unit_attack(
    unit1: &mut Unit,
    unit1_transform: &Transform,
    unit2: &mut Unit,
    unit2_transform: &Transform,
) -> GeneralResult {
    let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit1_transform.translation.truncate());
    let other_unit_hex = HEX_LAYOUT.world_pos_to_hex(unit2_transform.translation.truncate());

    if unit_hex == other_unit_hex {
        return GeneralResult::Fail;
    }

    let distance = unit_hex.unsigned_distance_to(other_unit_hex);
    if distance > unit_range(unit1) {
        return GeneralResult::Fail;
    }

    unit2.health = (unit2.health - unit_damage(unit1)).max(0);
    unit1.energy -= unit_attack_cost(unit1);

    GeneralResult::Success
}

pub fn unit_move(
    unit: &mut Unit,
    unit_transform: &mut Transform,
    target_translation: &Vec3,
) -> GeneralResult {
    let hex_pos = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());
    let new_hex_pos = HEX_LAYOUT.world_pos_to_hex(target_translation.truncate());

    if hex_pos.unsigned_distance_to(new_hex_pos) != 1 {
        return GeneralResult::Fail;
    }

    let angle = find_angle(
        unit_transform.translation.x,
        unit_transform.translation.y,
        target_translation.x,
        target_translation.y,
    ) + PI / 2.;

    unit.moving = Some(Moving {
        start_pos: unit_transform.translation,
        target_pos: *target_translation,
        angle,
    });
    unit.energy -= unit_move_cost(unit);

    unit_transform.rotation = Quat::from_rotation_z(angle);

    GeneralResult::Success
}
