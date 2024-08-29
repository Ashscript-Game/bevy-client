use bevy::prelude::*;
use hexx::Hex;

use crate::{components::{Turret, Unit}, constants::GeneralResult, engine::{terrain::HEX_LAYOUT, turret::{turret_attack, turret_attack_cost}}, projectile::laser::create_laser};

pub fn turret_ai(
    mut turrets: Query<(&mut Turret, &Transform)>,
    mut units: Query<(&mut Unit, &Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    for (mut turret, turret_transform) in turrets.iter_mut() {

        turret.energy += turret.energy_gen;

        if turret_attack_cost(&turret) > turret.energy {
            continue;
        }

        let turret_hex: Hex = HEX_LAYOUT.world_pos_to_hex(turret_transform.translation.truncate());

        for (mut unit, unit_transform) in units.iter_mut() {
            let unit_hex =
                HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());

            let distance = turret_hex.unsigned_distance_to(unit_hex);
            if distance > turret.range {
                continue;
            }

            if turret_attack(&mut turret, turret_transform, &mut unit, unit_transform) == GeneralResult::Fail {
                continue;
            }

            let laser_target_pos = {
                if let Some(moving) = &unit.moving {
                    moving.target_pos
                } else {
                    unit_transform.translation
                }
            };

            create_laser(
                &turret_transform.translation,
                &laser_target_pos,
                turret.damage,
                &mut commands,
                &mut meshes,
                &mut materials,
            );
            break;
        }
    }
}
