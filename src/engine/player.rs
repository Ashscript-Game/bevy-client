use crate::{
    ai_scripts,
    components::{Factory, GameState, MappedUnits, PlayerState, PlayerStates, Unit, Wall},
    constants::GeneralResult,
    projectile::laser::create_laser,
    types::PlayerScript,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use hexx::Hex;
use std::collections::HashMap;

use super::{
    factory::factory_spawn,
    terrain::HEX_LAYOUT,
    unit::{unit_at_hex, unit_attack, unit_damage, unit_move_hex},
};

pub fn populate_game_state(
    mut game_state: ResMut<GameState>,
    units: Query<(&Unit, &Transform, Entity)>,
    factories: Query<(&Factory, &Transform, Entity)>,
    walls: Query<&Transform, With<Wall>>,
) {
    // Units

    let cloned_units = units
        .iter()
        .map(|(u, t, e)| (u.clone(), *t, e))
        .collect::<Vec<(Unit, Transform, Entity)>>();
    game_state.units = cloned_units;

    // Factories

    let cloned_factories = factories
        .iter()
        .map(|(f, t, e)| (f.clone(), *t, e))
        .collect::<Vec<(Factory, Transform, Entity)>>();
    game_state.factories = cloned_factories;

    // Occupied Tiles

    let occupied_tiles: HashSet<Hex> = HashSet::from_iter(
        walls
            .iter()
            .map(|transform| HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()))
            .collect::<Vec<Hex>>(),
    );
    game_state.walls = occupied_tiles;
}

pub fn run_player_scripts(game_state: Res<GameState>, mut player_states: ResMut<PlayerStates>) {
    let mut player_scripts: HashMap<String, PlayerScript> = HashMap::new(); /*  vec![ai_scripts::basic_economy::main]; */
    player_scripts.insert(
        game_state.players[0].name.clone(),
        ai_scripts::basic_combat::main,
    );
    player_scripts.insert(
        game_state.players[1].name.clone(),
        ai_scripts::basic_combat::main,
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
        let player_state = player_states.0.get_mut(&player_name).unwrap();
        let player_script = player_scripts
            .get(&player_name)
            .expect("player script not found");

        player_script(&game_state, player_state);
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

    for (_player_name, player_state) in &player_states.0 {
        for intent in player_state.intents.unit_move.iter() {
            let Ok((mut unit, mut unit_transform, _entity)) = units.get_mut(intent.entity) else {
                continue;
            };

            if unit.owner_id != player_state.owner_id {
                continue;
            }

            // check if there is an other_unit at the destination
            // does not work in a bevy context because units might be moving towards but not yet reached. So allows double moving to a destination
            if let Some((_other_unit, _other_unit_transform, _other_entity)) =
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
    for (_player_name, player_state) in &player_states.0 {
        for intent in player_state.intents.factory_spawn.iter() {
            let Ok((factory, _)) = factories.get(intent.entity) else {
                continue;
            };
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

pub fn run_unit_attack_intents(
    player_states: ResMut<PlayerStates>,
    mut units: Query<(&mut Unit, &Transform, Entity)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (_player_name, player_state) in &player_states.0 {
        for intent in player_state.intents.unit_attack.iter() {
            let Ok([(mut attacker, attacker_transform, _), (mut target, target_transform, _)]) =
                units.get_many_mut([intent.attacker, intent.target])
            else {
                println!("[run unit attack intents] attacker or target not found");
                continue;
            };

            if attacker.owner_id != player_state.owner_id {
                continue;
            }

            if unit_attack(
                &mut attacker,
                attacker_transform,
                &mut target,
                target_transform,
            ) != GeneralResult::Success
            {
                continue;
            }

            let laser_target_pos = {
                if let Some(moving) = &target.moving {
                    moving.target_pos
                } else {
                    target_transform.translation
                }
            };

            create_laser(
                &attacker_transform.translation,
                &laser_target_pos,
                intent.target,
                unit_damage(&attacker),
                &mut commands,
                &asset_server,
            );
        }
    }
}
