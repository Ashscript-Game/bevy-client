use ashscript_types::actions::{self, UnitAttack};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use hexx::Hex;

use crate::components::{Actions, GameObjectKindComp, HealthComp, State, Unit};

pub fn unit_attack_actions(units: Query<(&Unit, &Transform, Entity)>, actions: Res<Actions>) {
    for (unit, transform, entity) in units.iter() {

    }
}

///  Charges units energy proportional to the energy they spent on attacking
pub fn charge_unit_attack_actions() {

}

/// Applies damage to entities attacked by a unit
pub fn apply_unit_attack_actions(with_health: Query<(&Transform, &mut HealthComp, Entity, &GameObjectKindComp)>, actions: Res<Actions>) {

    let entities_by_hex: HashMap<Hex, Vec<Entity>> = HashMap::new();

    for action in actions.0.unit_attack.iter() {
        let entity = entities_by_hex.get(&action.target_hex).unwrap().iter().filter(|entity| {
            let (_, _, _, kind) = with_health.get(**entity).unwrap();

            true
            /* kind.0 == action.target_kind */
        });


    }
}