use ashscript_types::{
    components::{energy::Energy, health::Health, owner::Owner, tile::Tile},
    constants::map::{CHUNK_SIZE, HEX_LAYOUT},
    objects::GameObjectKind,
};
use bevy::prelude::*;
use hexx::Hex;

use crate::{
    components::{
        intents, Actions, HealthComp, LoadChunks, MappedGameObjects, Moving, PlayerState, State, TickEvent, Unit, UnloadedChunks
    },
    constants::{self, GeneralResult, UnitPart, UNIT_PART_WEIGHTS},
    unit::plugin::create_unit,
    utils::find_angle_coords,
};

pub fn generate_units_on_chunkload(
    unloaded_chunks: Res<UnloadedChunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_object_map: MappedGameObjects,
    state: Res<State>,
) {
    for (entity, (_, tile, owner, health, body, energy)) in state
        .world
        .query::<((
            &ashscript_types::components::unit::Unit,
            &Tile,
            &Owner,
            &ashscript_types::components::health::Health,
            &ashscript_types::components::body::UnitBody,
            &Energy,
        ))>()
        .iter()
    {
        if !unloaded_chunks
            .0
            .contains(&tile.hex.to_lower_res(CHUNK_SIZE))
        {
            continue;
        }

        create_unit(
            tile.hex,
            &mut commands,
            &asset_server,
            &mut game_object_map,
            *health,
            owner.0,
            *body,
            *energy,
        );
    }
}

pub fn generate_units_from_factory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_object_map: MappedGameObjects,
    actions: Res<Actions>,
) {
    println!("spawning units from factories");

    for action in actions.0.factory_spawn_unit.iter() {
        create_unit(
            action.out,
            &mut commands,
            &asset_server,
            &mut game_object_map,
            ashscript_types::components::health::Health::for_unit(&action.body),
            action.owner,
            action.body,
            Energy::for_unit_body(&action.body),
        );
    }
}

pub fn force_units_move(
    mut game_object_map: MappedGameObjects,
    mut query: Query<(&mut Unit, &mut Transform, Entity)>,
) {
    println!("force moving units");

    for (mut unit, mut transform, _) in query.iter_mut() {
        let Some(moving) = &unit.moving else {
            continue;
        };

        let from_hex = HEX_LAYOUT.world_pos_to_hex(moving.start_pos.truncate());
        let target_hex = HEX_LAYOUT.world_pos_to_hex(moving.target_pos.truncate());

        transform.translation = moving.target_pos;
        unit.moving = None;

        game_object_map.move_to(&from_hex, target_hex, GameObjectKind::Unit);
    }
}

pub fn move_units_from_actions(
    mut query: Query<(&mut Unit, &mut Transform, Entity)>,
    game_object_map: MappedGameObjects,
    actions: Res<Actions>,
) {
    println!("moving units");

    for action in actions.0.unit_move.iter() {
        let Some(entity) = game_object_map.entity(&action.from, GameObjectKind::Unit) else {
            continue;
        };
        let Ok((mut unit, mut transform, _)) = query.get_mut(*entity) else {
            continue;
        };

        unit_move_hex(&mut unit, &mut transform, action.to);
    }
}

pub fn units_attack_from_actions(
    actions: Res<Actions>,
    mut commands: Commands,
    mut targets: Query<(&Transform, &mut HealthComp)>,
    mut game_object_map: MappedGameObjects,
) {
    for action in actions.0.unit_attack.iter() {
        let Some(target_entity) = game_object_map.entity(&action.target_hex, action.target_kind)
        else {
            continue;
        };

        let Ok((_, mut target_health)) = targets.get_mut(*target_entity) else {
            continue;
        };

        if action.damage >= target_health.0.current {
            target_health.0.current = 0;
            continue;
        }

        target_health.0.current = target_health.0.current.saturating_sub(action.damage);
    }
}

pub fn kill_units(
    units: Query<(&Unit, &Transform, &HealthComp, Entity)>,
    mut commands: Commands,
    mut game_object_map: MappedGameObjects,
) {
    for (unit, transform, health, entity) in units.iter() {
        /* if unit.age > constants::unit::MAX_AGE {
            game_object_map.remove(
                &HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()),
                GameObjectKind::Unit,
            );

            commands.entity(entity).despawn();
            continue;
        } */

        if health.0.current == 0 {
            game_object_map.remove(
                &HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()),
                GameObjectKind::Unit,
            );

            commands.entity(entity).despawn();
            continue;
        }
    }
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
    let hex_pos = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());
    let new_hex_pos = HEX_LAYOUT.world_pos_to_hex(target_translation.truncate());

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
    /* unit.energy -= unit_move_cost(unit); */

    unit_transform.rotation = Quat::from_rotation_z(angle);

    GeneralResult::Success
}

pub fn unit_attack_intent(entity: &Entity, target: &Entity, player_state: &mut PlayerState) {
    player_state.intents.unit_attack.push(intents::UnitAttack {
        attacker: *entity,
        target: *target,
    });
}
