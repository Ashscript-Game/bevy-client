
use bevy::prelude::*;

use crate::{
    components::{Assembler, Distributor}, constants::{self, GeneralResult, RESOURCE_INPUTS}, engine::{terrain::HEX_LAYOUT}, projectile::resource_blob::create_resource_blob, structure::utils::transfer
};

pub fn distributor_ai(
    mut distributors: Query<(&Transform, &mut Distributor)>,
    mut assemblers: Query<(&Transform, &mut Assembler, Entity)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // find assemblers in range

    // distribute to assemblers such that all their stores are as equal as possiblex

    for (distributor_transform, mut distributor) in distributors.iter_mut() {
        let distributor_resource = distributor.resource;

        *distributor
            .store
            .resources
            .entry(distributor_resource)
            .or_insert(0) += 1000;

        let distributor_hex =
            HEX_LAYOUT.world_pos_to_hex(distributor_transform.translation.truncate());

        for (assembler_transform, mut assembler, entity) in assemblers.iter_mut() {
            let distributor_resource_amount = {
                *distributor
                    .store
                    .resources
                    .get(&distributor_resource)
                    .unwrap_or(&0)
                    .min(&10)
            };

            let input_resources = &RESOURCE_INPUTS[assembler.output_resource];
            if !input_resources.contains(&distributor.resource) {
                continue;
            }

            let assembler_hex =
                HEX_LAYOUT.world_pos_to_hex(assembler_transform.translation.truncate());

            let distance = distributor_hex.unsigned_distance_to(assembler_hex);
            if distance > constants::distributor::RANGE {
                continue;
            }

            if transfer(
                &distributor_transform.translation,
                &mut distributor.store,
                &assembler_transform.translation,
                &mut assembler.store,
                &distributor_resource,
                distributor_resource_amount,
            ) == GeneralResult::Fail
            {
                continue;
            }

            create_resource_blob(
                &distributor_transform.translation,
                &assembler_transform.translation,
                entity,
                &distributor_resource,
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }
}
