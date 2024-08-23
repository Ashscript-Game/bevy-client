use std::{f32::consts::PI, time::Instant};

use bevy::{
    app::{App, Plugin, Startup},
    gizmos,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        mesh::{self, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::hashbrown::HashMap,
};
use bevy_inspector_egui::bevy_egui::systems::InputResources;
use bevy_magic_light_2d::prelude::{LightOccluder2D, OmniLightSource2D, CAMERA_LAYER_OBJECTS};
use hexx::{hex, shapes, Hex};

use crate::{
    components::{Assembler, Distributor, OccupiesTile, ResourceBlob, Structure},
    constants::{self, distributor, z_order, Resource, RESOURCE_INPUTS, SECONDS_PER_TICK},
    terrain::tiles::HEX_LAYOUT,
    utils::{self, find_angle},
};

use super::assembler;

pub struct DistributorPlugin;

impl Plugin for DistributorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_distributors, setup_distributor_replenishing))
            .add_systems(Update, distributor_ai);
    }
}

fn generate_distributors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<OccupiesTile>>,
) {
    for hex in shapes::hexagon(hex(15, -12), 2) {
        spawn_distributor(hex, &mut commands, &asset_server, &query);
    }
}

fn spawn_distributor(
    hex: hexx::Hex,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: &Query<&Transform, With<OccupiesTile>>,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    // not very efficient
    for transform in query.iter() {
        if transform.translation.truncate() == world_pos {
            return;
        }
    }

    println!("spawning distributor");

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(distributor::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 5.0),
                scale: Vec3::new(0.08, 0.08, 1.0),
                ..default()
            },
            ..default()
        },
        OccupiesTile,
        Structure,
        Distributor {
            resource: constants::Resource::Metal,
            store: HashMap::new(),
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));

    distributor_light(world_pos, commands, constants::distributor::COLOR);
}

fn distributor_light(world_pos: Vec2, commands: &mut Commands, color: Color) {
    commands
        .spawn((
            OmniLightSource2D {
                intensity: 0.2,
                color,
                falloff: Vec3::new(1.5, 10., 0.005),
                ..default()
            },
            Distributor {
                resource: constants::Resource::Coal,
                store: {
                    let mut map = HashMap::new();
                    map.insert(constants::Resource::Coal, 1000);
                    map
                },
            },
        ))
        .insert(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(RenderLayers::all());
}

#[derive(Resource)]
pub struct ReplenishDistributors(Timer);

fn setup_distributor_replenishing(mut commands: Commands) {
    commands.insert_resource(ReplenishDistributors(Timer::from_seconds(SECONDS_PER_TICK, TimerMode::Repeating)));
}

fn distributor_ai(
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
