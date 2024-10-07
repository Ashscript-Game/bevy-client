use std::u32;

use bevy::{prelude::*, utils::hashbrown::HashSet};
use hexx::Hex;

use crate::{
    components::{GameState, PlayerState, Unit},
    engine::{
        factory::factory_spawn_intent,
        terrain::HEX_LAYOUT,
        unit::{unit_attack_intent, unit_move_intent, unit_range},
    },
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

    let unit_hexes = game_state
        .units
        .iter()
        .map(|(u, t, _)| HEX_LAYOUT.world_pos_to_hex(t.translation.truncate()))
        .collect::<HashSet<Hex>>();

    for (unit, unit_transform, entity) in &game_state.units {
        if unit.owner_id != player_state.owner_id {
            continue;
        }

        let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());

        let mut closest_enemy: Option<Entity> = None;
        let mut lowest_distance = u32::MAX;
        let mut closest_enemy_hex = Hex::new(0, 0);

        for (enemy_unit, enemy_transform, enemy_entity) in enemy_units.iter() {
            let enemy_hex = HEX_LAYOUT.world_pos_to_hex(enemy_transform.translation.truncate());

            let distance = unit_hex.unsigned_distance_to(enemy_hex);
            if distance >= lowest_distance {
                continue;
            }

            closest_enemy = Some(*enemy_entity);
            lowest_distance = distance;
            closest_enemy_hex = enemy_hex;
        }

        let Some(closest_enemy) = closest_enemy else {
            continue;
        };

        unit_attack_intent(&entity, &closest_enemy, player_state);

        // If we are sufficiently out of range, move closer
        if lowest_distance >= unit_range(unit) {
            let path = hexx::algorithms::a_star(unit_hex, closest_enemy_hex, |_, bhex| {
                if bhex == closest_enemy_hex || bhex == unit_hex {
                    return Some(1)
                }

                if game_state.walls.contains(&bhex) {
                    return None
                }

                if unit_hexes.contains(&bhex) {
                    return Some(5)
                }

                Some(1)
                /* (bhex != closest_enemy_hex &&/* bhex != closest_enemy_hex && ahex != unit_hex && */game_state.occupied_tiles.contains(&bhex)).then_some(1) */
            });

            if let Some(path) = path {
                if let Some(hex) = path.get(1) {
                    unit_move_intent(entity, *hex, player_state);
                }
            }
            else {
                println!("[basic combat ai] no path found");
            }
        }
    }
}

fn use_factories(game_state: &GameState, player_state: &mut PlayerState) {
    for (_, _, entity) in game_state.factories.iter() {
        factory_spawn_intent(entity, player_state);
    }
}
