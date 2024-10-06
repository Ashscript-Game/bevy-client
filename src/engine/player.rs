use crate::{
    ai_scripts,
    components::{Factory, GameState, MappedUnits, PlayerState, PlayerStates, Unit},
    types::PlayerScript,
};
use bevy::{ecs::entity, prelude::*};
use std::collections::HashMap;

use super::{
    factory::factory_spawn,
    terrain::HEX_LAYOUT,
    unit::{unit_at_hex, unit_move_hex},
};

pub fn populate_game_state(
    mut game_state: ResMut<GameState>,
    units: Query<(&Unit, &Transform, Entity)>,
    factories: Query<(&Factory, &Transform, Entity)>,
) {
    // Units

    let cloned_units = units
        .iter()
        .map(|(u, t, e)| (u.clone(), *t, e))
        .collect::<Vec<(Unit, Transform, Entity)>>();
    game_state.units = cloned_units;

    let cloned_factories = factories
        .iter()
        .map(|(f, t, e)| (f.clone(), *t, e))
        .collect::<Vec<(Factory, Transform, Entity)>>();
    game_state.factories = cloned_factories;
}

pub fn run_player_scripts(game_state: Res<GameState>, mut player_states: ResMut<PlayerStates>) {
    let mut player_scripts: HashMap<String, PlayerScript> = HashMap::new(); /*  vec![ai_scripts::basic_economy::main]; */
    player_scripts.insert(
        game_state.players[0].name.clone(),
        ai_scripts::basic_economy::main,
    );
    player_scripts.insert(
        game_state.players[1].name.clone(),
        ai_scripts::basic_economy::main,
    );

    player_states
        .0
        .insert(game_state.players[0].name.clone(), PlayerState::new(0));
    player_states
        .0
        .insert(game_state.players[1].name.clone(), PlayerState::new(1));

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

            if unit.owner_id != player_state.owner_id {
                continue;
            }

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

pub fn run_factory_spawn_intents(
    player_states: ResMut<PlayerStates>,
    mut factories: Query<(&mut Factory, &Transform)>,
    units: Query<(&Unit, &Transform, Entity)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mapped_units: MappedUnits,
) {
    for (player_name, player_state) in &player_states.0 {
        for intent in player_state.intents.factory_spawn.iter() {
            let (factory, _) = factories.get(intent.entity).expect("Factory not found");
            if factory.owner_id != player_state.owner_id {
                continue;
            }

            factory_spawn(
                intent,
                &mut factories,
                &units,
                &mut commands,
                &asset_server,
                &mut mapped_units,
            );
        }
    }
}
