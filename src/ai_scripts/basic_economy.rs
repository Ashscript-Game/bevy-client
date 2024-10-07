use bevy::prelude::*;
use hexx::Hex;

use crate::{
    components::{GameState, PlayerState},
    engine::{factory::factory_spawn_intent, terrain::HEX_LAYOUT, unit::unit_move_intent},
    utils::pick,
};

pub fn main(game_state: &Res<GameState>, player_state: &mut PlayerState) {
    move_units_randomly(game_state, player_state);
    use_factories(game_state, player_state);
}

fn move_units_randomly(game_state: &GameState, player_state: &mut PlayerState) {
    let q_offsets = [-1, 0, 1];
    let t_offsets = [-1, 0, 1];

    for (_unit, unit_transform, entity) in game_state.units.iter() {
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