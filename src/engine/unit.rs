use bevy::{prelude::*, transform::commands};

use crate::{
    components::Unit,
    constants::{self, GeneralResult, UnitPart},
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
