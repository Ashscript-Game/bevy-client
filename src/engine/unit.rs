use ashscript_types::components::{owner::Owner, tile::Tile};
use bevy::prelude::*;
use hexx::Hex;

use crate::{
    components::{intents, Actions, MappedUnits, Moving, PlayerState, State, Unit},
    constants::{self, GeneralResult, UnitPart, UNIT_PART_WEIGHTS},
    unit::plugin::create_unit,
    utils::find_angle_coords,
};

use super::terrain::HEX_LAYOUT;

pub fn generate_units_from_keyframe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut unit_map: MappedUnits,
    state: Res<State>,
) {
    for (entity, (_, tile, owner)) in state
        .world
        .query::<((&ashscript_types::unit::Unit, &Tile, &Owner))>()
        .iter()
    {
        create_unit(
            tile.hex,
            &mut commands,
            &asset_server,
            &mut unit_map,
            owner.0,
        );
    }
}

pub fn move_units_from_actions(
    mut query: Query<(&mut Unit, &mut Transform, Entity)>,
    mut unit_map: MappedUnits,
    actions: Res<Actions>,
) {
    for action in actions.0.unit_move.iter() {
        let Some(entity) = unit_map.entity(&action.from) else {
            continue;
        };
        let Ok((mut unit, mut transform, _)) = query.get_mut(*entity) else {
            continue;
        };

        unit_move_hex(&mut unit, &mut transform, action.to);
    }
}

pub fn kill_units(
    units: Query<(&Unit, &Transform, Entity)>,
    mut commands: Commands,
    mut unit_maps: MappedUnits,
) {
    for (unit, transform, entity) in units.iter() {
        if unit.age > constants::unit::MAX_AGE {
            unit_maps.remove(&HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()));

            commands.entity(entity).despawn();
            continue;
        }

        if unit.health == 0 {
            unit_maps.remove(&HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()));

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
    if unit1.energy < unit_attack_cost(unit1) {
        return GeneralResult::Fail;
    }

    let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit1_transform.translation.truncate());
    let other_unit_hex = HEX_LAYOUT.world_pos_to_hex(unit2_transform.translation.truncate());

    if unit_hex == other_unit_hex {
        return GeneralResult::Fail;
    }

    let distance = unit_hex.unsigned_distance_to(other_unit_hex);
    if distance > unit_range(unit1) {
        return GeneralResult::Fail;
    }

    let damage = unit_damage(unit1);
    if damage > unit2.health {
        unit2.health = 0
    } else {
        unit2.health -= damage
    }

    unit1.energy -= unit_attack_cost(unit1);

    GeneralResult::Success
}

pub fn unit_move_intent(entity: &Entity, to_hex: Hex, player_state: &mut PlayerState) {
    player_state.intents.unit_move.push(intents::UnitMove {
        entity: *entity,
        to: to_hex,
    });
}

pub fn unit_move_hex(
    unit: &mut Unit,
    unit_transform: &mut Transform,
    target_hex: Hex,
) -> GeneralResult {
    let target_translation_2d = HEX_LAYOUT.hex_to_world_pos(target_hex);
    let target_translation = Vec3::new(target_translation_2d.x, target_translation_2d.y, 1.);

    unit_move(unit, unit_transform, &target_translation)
}

pub fn unit_move(
    unit: &mut Unit,
    unit_transform: &mut Transform,
    target_translation: &Vec3,
) -> GeneralResult {
    if unit.energy < unit_move_cost(unit) {
        return GeneralResult::Fail;
    }

    let hex_pos = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());
    let new_hex_pos = HEX_LAYOUT.world_pos_to_hex(target_translation.truncate());

    if hex_pos.unsigned_distance_to(new_hex_pos) != 1 {
        return GeneralResult::Fail;
    }

    let angle = find_angle_coords(
        unit_transform.translation.x,
        unit_transform.translation.y,
        target_translation.x,
        target_translation.y,
    );

    unit.moving = Some(Moving {
        start_pos: unit_transform.translation,
        target_pos: *target_translation,
        angle,
    });
    unit.energy -= unit_move_cost(unit);

    unit_transform.rotation = Quat::from_rotation_z(angle);

    GeneralResult::Success
}

pub fn unit_at_hex(
    hex: hexx::Hex,
    units: &[(Unit, Transform, Entity)],
) -> Option<(&Unit, &Transform, &Entity)> {
    for (unit, unit_transform, entity) in units.iter() {
        if hex != HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate()) {
            continue;
        }

        return Some((unit, unit_transform, entity));
    }

    None
}

pub fn unit_attack_intent(entity: &Entity, target: &Entity, player_state: &mut PlayerState) {
    player_state.intents.unit_attack.push(intents::UnitAttack {
        attacker: *entity,
        target: *target,
    });
}
