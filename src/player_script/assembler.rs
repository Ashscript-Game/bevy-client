use bevy::prelude::*;
use hexx::Hex;
use rand::distributions::DistIter;

use crate::{
    components::{Assembler, Distributor}, constants::{self, GeneralResult}, engine::{terrain::HEX_LAYOUT}, projectile::resource_blob::create_resource_blob, structure::{
        assembler::assembler_produce,
        utils::transfer,
    }
};

pub fn assemblers_produce(
    mut assemblers: Query<&mut Assembler>,
) {

    for mut assembler in assemblers.iter_mut() {
        assembler_produce(&mut assembler);
    }
}

pub fn assembler_ai(
    mut assemblers: Query<(&Transform, &mut Assembler)>,
    mut distributors: Query<(&Transform, &mut Distributor, Entity)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    // for each distributor:

    // convert input resources into 1 output resource if possible

    // if there is > 0 output resource in store,
    // find a distributor to that matches output resource transfer to

    for (assembler_transform, mut assembler) in assemblers.iter_mut() {
        let output_resource = assembler.output_resource;
        let amount = {
            let Some(amount) = assembler.store.resources.get(&output_resource) else {
                continue;
            };

            if *amount == 0 {
                continue;
            }

            *amount
        };

        let assembler_hex = HEX_LAYOUT.world_pos_to_hex(assembler_transform.translation.truncate());

        /* let Some((distributor_transform, distributor)) =
            find_distributor(&assembler_hex, &mut distributors)
        else {
            continue;
        }; */

        let Some((distributor_transform, mut distributor, entity)) =
            distributors.iter_mut().find(|(transform, distributor, entity)| {
                let distributor_hex = HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate());

                distributor.resource == output_resource
                    && assembler_hex.unsigned_distance_to(distributor_hex)
                        <= constants::assembler::RANGE
            })
        else {
            continue;
        };

        if transfer(
            &assembler_transform.translation,
            &mut assembler.store,
            &distributor_transform.translation,
            &mut distributor.store,
            &output_resource,
            amount,
        ) == GeneralResult::Fail
        {
            continue;
        };

        create_resource_blob(
            &assembler_transform.translation,
            &distributor_transform.translation,
            entity,
            &output_resource,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

/* fn find_distributor<'a>(
    assembler_hex: &'a Hex,
    distributors: &'a mut Query<'a, 'a, (&'a Transform, &'a mut Distributor)>,
) -> Option<(&'a Transform, &'a mut Distributor)> {
    for (distributor_transform, mut distributor) in distributors.iter_mut() {
        let distributor_hex =
            HEX_LAYOUT.world_pos_to_hex(distributor_transform.translation.truncate());

        let distance = assembler_hex.unsigned_distance_to(distributor_hex);
        if distance > constants::assembler::RANGE {
            continue;
        }

        return Some((distributor_transform, &mut distributor));
    }

    None
}
 */
