use bevy::prelude::*;

use crate::{
    components::{intents, Factory, MappedUnits, PlayerState, Unit},
    constants,
    unit::plugin::spawn_unit,
};

use super::{terrain::HEX_LAYOUT, unit::unit_at_hex};

pub fn factory_spawn(
    intent: &intents::FactorySpawn,
    factories: &mut Query<(&mut Factory, &Transform)>,
    units: &Query<(&Unit, &Transform, Entity)>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mapped_units: &mut MappedUnits,
) {
    let units_vec = units
        .iter()
        .map(|(u, t, e)| (u.clone(), *t, e))
        .collect::<Vec<(Unit, Transform, Entity)>>();

    let (mut factory, factory_transform) =
        factories.get_mut(intent.entity).expect("Factory not found");

    if factory.production_progress < 100 {
        return;
    }

    let factory_hex = HEX_LAYOUT.world_pos_to_hex(factory_transform.translation.truncate());

    for neighbour_hex in factory_hex.all_neighbors() {
        if let Some(unit) = unit_at_hex(neighbour_hex, &units_vec) {
            continue;
        };

        spawn_unit(
            neighbour_hex,
            commands,
            asset_server,
            mapped_units,
            factory.owner_id,
        );
        factory.production_progress = 0;
        break;
    }
}

pub fn progress_factories(mut factories: Query<&mut Factory>) {
    for mut factory in factories.iter_mut() {
        factory.production_progress = (factory.production_progress
            + (1. / constants::factory::DEFAULT_PRODUCTION_SPEED as f32 * 100.) as u8)
            .min(100);
    }
}

pub fn factory_spawn_intent(entity: &Entity, player_state: &mut PlayerState) {
    player_state
        .intents
        .factory_spawn
        .push(intents::FactorySpawn {
            entity: *entity,
            out: None,
        });
}
