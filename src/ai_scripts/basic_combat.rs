use std::u32;

use bevy::prelude::*;
use hexx::Hex;

use crate::{
    components::{GameState, PlayerState, Unit},
    engine::{
        factory::factory_spawn_intent,
        terrain::HEX_LAYOUT,
        unit::{unit_attack_intent, unit_move_intent},
    },
    utils::{self, pick},
};

pub fn main(game_state: &Res<GameState>, player_state: &mut PlayerState) {
    move_and_attack_units(game_state, player_state);
    use_factories(game_state, player_state);
}

fn move_and_attack_units(game_state: &GameState, player_state: &mut PlayerState) {
    let enemy_units = game_state
        .units
        .iter()
        .filter(|(u, _, _)| u.owner_id != player_state.owner_id)
        .cloned()
        .collect::<Vec<(Unit, Transform, Entity)>>();

    for (unit, unit_transform, entity) in &game_state.units {
        if unit.owner_id != player_state.owner_id {
            continue;
        }

        let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());

        let mut closest_enemy: Option<Entity> = None;
        let mut lowest_distance = u32::MAX;

        for (enemy_unit, enemy_transform, enemy_entity) in enemy_units.iter() {
            let enemy_hex = HEX_LAYOUT.world_pos_to_hex(enemy_transform.translation.truncate());

            let distance = unit_hex.unsigned_distance_to(enemy_hex);
            if distance >= lowest_distance {
                continue;
            }

            closest_enemy = Some(*enemy_entity);
            lowest_distance = distance;
        }

        let Some(closest_enemy) = closest_enemy else {
            continue;
        };

        unit_attack_intent(&entity, &closest_enemy, player_state);
    }

    let q_offsets = [-1, 0, 1];
    let t_offsets = [-1, 0, 1];

    for (unit, unit_transform, entity) in game_state.units.iter() {
        let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());
        let to_hex = Hex::new(
            unit_hex.x + *pick(&q_offsets),
            unit_hex.y + *pick(&t_offsets),
        );

        unit_move_intent(entity, to_hex, player_state);
    }
}

fn use_factories(game_state: &GameState, player_state: &mut PlayerState) {
    for (_, _, entity) in game_state.factories.iter() {
        factory_spawn_intent(entity, player_state);
    }
}
