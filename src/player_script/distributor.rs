use std::f32::consts::PI;

use bevy::{prelude::*, render::view::RenderLayers, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};

use crate::{components::{Assembler, Distributor, ResourceBlob}, constants::{self, z_order, RESOURCE_INPUTS}, engine::distributor::ReplenishDistributors, terrain::tiles::HEX_LAYOUT, utils::find_angle};

pub fn distributor_ai(
    mut distributors: Query<(&Transform, &mut Distributor)>,
    mut assemblers: Query<(&Transform, &mut Assembler)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut replenish_distributors: ResMut<ReplenishDistributors>,
) {
    // replenish distributors periodically

    replenish_distributors.0.tick(time.delta());

    if replenish_distributors.0.finished() {
        for (_, mut distributor) in distributors.iter_mut() {

            let resource = distributor.resource;

            *distributor
                .store
                .get_mut(&resource)
                .unwrap_or(&mut 0) += 1000;
        }
    }

    // find assemblers in range

    // distribute to assemblers such that all their stores are as equal as possiblex

    for (distributor_transform, mut distributor) in distributors.iter_mut() {

        let distributor_resource = distributor.resource;
        let mut distributor_resource_amount = {
            let Some(mut distributor_resource_amount) =
                distributor.store.get(&distributor_resource)
            else {
                continue;
            };

            if distributor_resource_amount == &0 {
                continue;
            }

            *distributor_resource_amount
        };

        let distributor_hex =
            HEX_LAYOUT.world_pos_to_hex(distributor_transform.translation.truncate());

        for (assembler_transform, mut assembler) in assemblers.iter_mut() {
            let assembler_hex =
                HEX_LAYOUT.world_pos_to_hex(assembler_transform.translation.truncate());

            let distance = distributor_hex.unsigned_distance_to(assembler_hex);

            if distance > constants::distributor::RANGE {
                continue;
            }

            let input_resources = &RESOURCE_INPUTS[assembler.output_resource];

            if !input_resources.contains(&distributor.resource) {
                continue;
            }

            *assembler
                .store
                .get_mut(&distributor_resource)
                .unwrap_or(&mut 0) += distributor_resource_amount;

            *distributor
                .store
                .get_mut(&distributor_resource)
                .unwrap_or(&mut 0) -= distributor_resource_amount;
            distributor_resource_amount -= distributor_resource_amount;

            let mesh = Mesh2dHandle(meshes.add(Circle::new(10.)));

            let angle = find_angle(
                distributor_transform.translation.x,
                distributor_transform.translation.y,
                assembler_transform.translation.x,
                assembler_transform.translation.y,
            ) + PI / 2.;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh,
                    material: materials.add(Color::WHITE),
                    transform: Transform {
                        translation: Vec3::new(
                            distributor_transform.translation.x,
                            distributor_transform.translation.y,
                            z_order::PROJECTILE,
                        ),
                        rotation: Quat::from_rotation_z(angle),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                    },
                    ..default()
                },
                OmniLightSource2D {
                    intensity: 0.3,
                    color: Color::WHITE,
                    falloff: Vec3::new(10., 10., 0.005),
                    ..Default::default()
                },
                ResourceBlob {
                    resource: distributor_resource,
                    target_pos: assembler_transform.translation,
                    start_pos: distributor_transform.translation,
                },
                RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
            ));

            println!(
                "transferring {} from {} to {}",
                distributor_resource_amount,
                distributor_transform.translation.truncate(),
                assembler_transform.translation.truncate()
            );
        }
    }
}
