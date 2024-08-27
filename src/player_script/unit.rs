use bevy::{prelude::*, utils::hashbrown::HashSet};
use hexx::Hex;

use crate::{
    components::{OccupiesTile, Unit},
    constants::GeneralResult,
    engine::{
        terrain::HEX_LAYOUT,
        unit::{unit_attack, unit_attack_cost, unit_damage, unit_move, unit_move_cost, unit_range},
    },
    projectile::laser::create_laser,
    utils::pick,
};

pub fn unit_ai(
    mut units: Query<(&mut Unit, &mut Transform)>,
    occupiers: Query<&Transform, (With<OccupiesTile>, Without<Unit>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let unit_tiles: HashSet<Hex> = HashSet::from_iter(
        units
            .iter()
            .map(|(_, transform)| HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()))
            .collect::<Vec<Hex>>(),
    );

    let occupied_tiles: HashSet<Hex> = HashSet::from_iter(
        occupiers
            .iter()
            .map(|transform| HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()))
            .collect::<Vec<Hex>>(),
    );

    let q_offsets = [-1, 0, 1];
    let t_offsets = [-1, 0, 1];

    // temporary solution, cloning probably voids ability to deal damage
    let mut other_units = units
        .iter_mut()
        .map(|(u, t)| (u.clone(), *t))
        .collect::<Vec<(Unit, Transform)>>();

    for (mut unit, mut unit_transform) in units.iter_mut() {
        
        if let Some(moving) = &unit.moving {
            unit_transform.translation.x = moving.target_pos.x;
            unit_transform.translation.y = moving.target_pos.y;
            unit.moving = None;
        };
        
        if unit_move_cost(&unit) > unit.energy {
            continue;
        }

        let unit_hex: Hex = HEX_LAYOUT.world_pos_to_hex(unit_transform.translation.truncate());

        let mut tried_moves: u8 = 0;
        while tried_moves < 6 {
            let translation_hex = Hex::new(
                unit_hex.x + *pick(&q_offsets),
                unit_hex.y + *pick(&t_offsets),
            );
            let translation2d = HEX_LAYOUT.hex_to_world_pos(translation_hex);
            let translation = Vec3::new(translation2d.x, translation2d.y, 0.0);

            if translation.x + translation.y == 0. {
                tried_moves += 1;
                continue;
            }

            if unit_tiles.contains(&translation_hex) {
                tried_moves += 1;
                continue;
            }

            if occupied_tiles.contains(&translation_hex) {
                tried_moves += 1;
                continue;
            }

            if unit_move(&mut unit, &mut unit_transform, &translation) == GeneralResult::Success {
                break;
            }
        }

        if unit_attack_cost(&unit) > unit.energy {
            continue;
        }

        for (other_unit, other_unit_transform) in other_units.iter_mut() {
            let other_unit_hex =
                HEX_LAYOUT.world_pos_to_hex(other_unit_transform.translation.truncate());

            let distance = unit_hex.unsigned_distance_to(other_unit_hex);
            if distance > unit_range(&unit) {
                continue;
            }

            if unit_attack(&mut unit, &unit_transform, other_unit, other_unit_transform)
                == GeneralResult::Fail
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
