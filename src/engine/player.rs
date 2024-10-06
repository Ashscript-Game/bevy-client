use crate::{
    ai_scripts,
    components::{GameState, PlayerState, PlayerStates, Unit},
    types::PlayerScript,
};
use bevy::{ecs::entity, prelude::*};
use std::collections::HashMap;

use super::{terrain::HEX_LAYOUT, unit::unit_move_hex};

pub fn populate_game_state(
    mut game_state: ResMut<GameState>,
    units: Query<(&Unit, &Transform, Entity)>,
) {
    // Units

    let cloned_units = units
        .iter()
        .map(|(u, t, e)| (u.clone(), *t, e))
        .collect::<Vec<(Unit, Transform, Entity)>>();

    game_state.units = cloned_units;
}

pub fn run_player_scripts(game_state: Res<GameState>, mut player_states: ResMut<PlayerStates>) {
    let mut player_scripts: HashMap<String, PlayerScript> = HashMap::new(); /*  vec![ai_scripts::basic_economy::main]; */
    player_scripts.insert(game_state.players[0].name.clone(), ai_scripts::basic_economy::main);
    player_scripts.insert(game_state.players[1].name.clone(), ai_scripts::basic_economy::main);

    player_states.0.insert(game_state.players[0].name.clone(), PlayerState::new());
    player_states.0.insert(game_state.players[1].name.clone(), PlayerState::new());

    // run player scripts

    let player_names = player_scripts.keys().cloned();
    for player_name in player_names {
        let mut player_state = player_states.0.get_mut(&player_name).unwrap();
        let player_script = player_scripts
            .get(&player_name)
            .expect("player script not found");

        player_script(&game_state, &mut player_state);
    }
}

pub fn run_move_intents(
    player_states: ResMut<PlayerStates>,
    mut units: Query<(&mut Unit, &mut Transform, Entity)>,
) {
    // temporary solution, should be replaced with better method later
    let other_units = units
        .iter_mut()
        .map(|(u, t, e)| (u.clone(), *t, e))
        .collect::<Vec<(Unit, Transform, Entity)>>();

    for (player_name, player_state) in &player_states.0 {
        for intent in player_state.intents.unit_move.iter() {
            let (mut unit, mut unit_transform, entity) = units.get_mut(intent.entity).unwrap();

            // check if there is an other_unit at the destination
            // does not work in a bevy context because units might be moving towards but not yet reached. So allows double moving to a destination
            if let Some((other_unit, other_unit_transform, other_entity)) =
                unit_at_hex(intent.to, &other_units)
            {
                continue;
            }

            // TODO: also check for units with the same intent.to and decide which gets to go, or none

            unit_move_hex(&mut unit, &mut unit_transform, intent.to);
        }
    }
}

fn unit_at_hex<'a>(
    hex: hexx::Hex,
    units: &'a Vec<(Unit, Transform, Entity)>,
) -> Option<(&'a Unit, &'a Transform, &'a Entity)> {
    for (unit, unit_transform, entity) in units.iter() {
        if hex != HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate()) {
            continue;
        }

        return Some((unit, unit_transform, entity));
    }

    None
    /* `(&'a components::Unit, &'a bevy::prelude::Transform, bevy::prelude::Entity)` value */
}
