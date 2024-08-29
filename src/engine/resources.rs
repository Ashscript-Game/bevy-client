use crate::{
    components::{OccupiesTile, ResourceNode, Scrap},
    constants::{self, resource_node, resource_noise_tresholds, SIMPLEX_GENERATOR},
    engine::terrain::{hexagonal_plane, HEX_LAYOUT, HEX_SIZE},
};
use bevy::{
    app::{App, Plugin, Startup, Update},
    math::Vec3,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_light_2d::{
    light::{PointLight2d, PointLight2dBundle},
    prelude::{LightOccluder2d, LightOccluder2dBundle, LightOccluder2dShape},
};
use hexx::{hex, shapes};
use libnoise::Generator;

pub fn generate_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("generating nodes");

    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);

    let material_handles = [
        materials.add(ColorMaterial::from(constants::coal_node::COLOR)),
        materials.add(ColorMaterial::from(constants::mineral_node::COLOR)),
        materials.add(ColorMaterial::from(constants::scrap::COLOR)),
        materials.add(ColorMaterial::from(constants::wall::COLOR)),
    ];

    for hex in shapes::hexagon(hex(0, 0), 97) {
        let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);
        let noise = SIMPLEX_GENERATOR.sample([
            world_pos.x as f64 / 25.,
            world_pos.y as f64 / 25., /* hex.x as f64, hex.y as f64 */
        ]);
        /* println!("noise: {}", noise); */

        if noise > resource_noise_tresholds::WALL.0 && noise < resource_noise_tresholds::WALL.1 {

            let occluder = commands.spawn(LightOccluder2dBundle {
                light_occluder: LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: HEX_SIZE * 2./* / 2. */,
                    },
                },
                transform: Transform::from_xyz(0., 0., 0.0),
                ..default()
            }).id();

            commands.spawn((
                ColorMesh2dBundle {
                    transform: Transform::from_xyz(
                        world_pos.x,
                        world_pos.y,
                        constants::resource_node::Z_POS,
                    ),
                    mesh: mesh_handle.clone().into(),
                    material: material_handles[3].clone(),
                    ..default()
                },
                OccupiesTile,
                /* LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: HEX_SIZE * 0.5,
                    },
                }, */
            )).add_child(occluder);

            continue;
        }

        if noise > resource_noise_tresholds::COAL.0 && noise < resource_noise_tresholds::COAL.1 {
            let light = resource_node_light(world_pos, &mut commands, constants::coal_node::COLOR);

            commands
                .spawn((
                    ColorMesh2dBundle {
                        transform: Transform::from_xyz(
                            world_pos.x,
                            world_pos.y,
                            constants::resource_node::Z_POS,
                        ),
                        mesh: mesh_handle.clone().into(),
                        material: material_handles[0].clone(),
                        ..default()
                    },
                    OccupiesTile,
                    ResourceNode {
                        coal_percent: 50,
                        mineral_percent: 50,
                        ticks_to_regen: 0,
                        resource_remaining: 1000,
                    },
                ))
                .add_child(light);

            continue;
        }

        if noise > resource_noise_tresholds::MINERALS.0
            && noise < resource_noise_tresholds::MINERALS.1
        {
            let light =
                resource_node_light(world_pos, &mut commands, constants::mineral_node::COLOR);

            commands
                .spawn((
                    ColorMesh2dBundle {
                        transform: Transform::from_xyz(
                            world_pos.x,
                            world_pos.y,
                            constants::resource_node::Z_POS,
                        ),
                        mesh: mesh_handle.clone().into(),
                        material: material_handles[1].clone(),
                        ..default()
                    },
                    OccupiesTile,
                    ResourceNode {
                        coal_percent: 50,
                        mineral_percent: 50,
                        ticks_to_regen: 0,
                        resource_remaining: 1000,
                    },
                ))
                .add_child(light);

            continue;
        }

        if noise > resource_noise_tresholds::SCRAP.0 && noise < resource_noise_tresholds::SCRAP.1 {
            let light = resource_node_light(world_pos, &mut commands, constants::scrap::COLOR);

            commands
                .spawn((
                    ColorMesh2dBundle {
                        transform: Transform::from_xyz(
                            world_pos.x,
                            world_pos.y,
                            constants::resource_node::Z_POS,
                        ),
                        mesh: mesh_handle.clone().into(),
                        material: material_handles[2].clone(),
                        ..default()
                    },
                    OccupiesTile,
                    Scrap {
                        metal: 1000,
                        ticks_to_decay: 100,
                    },
                ))
                .add_child(light);

            continue;
        }
    }
}

fn resource_node_light(world_pos: Vec2, commands: &mut Commands, color: Color) -> Entity {
    commands
        .spawn(PointLight2dBundle {
            transform: Transform::from_xyz(0., 0., 150.),
            point_light: PointLight2d {
                intensity: 5.,
                color,
                radius: 1000.,
                falloff: 1.,
                ..default()
            },
            ..default()
        })
        .id()
}
