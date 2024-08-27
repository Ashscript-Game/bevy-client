use bevy::prelude::*;

use crate::{
    components::Unit,
    constants::GeneralResult,
    engine::{
        terrain::HEX_LAYOUT,
        unit::{unit_attack, unit_attack_cost, unit_damage, unit_range},
    },
    projectile::laser::create_laser,
};

pub fn unit_ai(
    mut units: Query<(&mut Unit, &mut Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    // temporary solution, cloning probably voids ability to deal damage
    let mut other_units = units.iter_mut().map(|(u, t)| (u.clone(), t.clone())).collect::<Vec<(Unit, Transform)>>();

    for (mut unit, unit_transform) in units.iter_mut() {

        if unit_attack_cost(&unit) > unit.energy {
            continue;
        }

        let unit_hex = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());

        for (other_unit, other_unit_transform) in other_units.iter_mut() {

            let other_unit_hex =
                HEX_LAYOUT.world_pos_to_hex(other_unit_transform.translation.truncate());

            let distance = unit_hex.unsigned_distance_to(other_unit_hex);
            if distance > unit_range(&unit) {
                continue;
            }

            if unit_attack(
                &mut unit,
                &unit_transform,
                other_unit,
                other_unit_transform,
            ) == GeneralResult::Fail
            {
                continue;
            }

            create_laser(
                &unit_transform.translation,
                &other_unit_transform.translation,
                unit_damage(&unit),
                &mut commands,
                &mut meshes,
                &mut materials,
            );
            break;
        }
    }
}
