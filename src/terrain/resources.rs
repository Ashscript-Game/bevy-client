use crate::{
    components::ResourceNode,
    constants::{self, resource_node, resource_noise_tresholds, SIMPLEX_GENERATOR},
    terrain::tiles::{hexagonal_plane, HEX_LAYOUT},
};
use bevy::{
    app::{App, Plugin, Startup, Update},
    math::Vec3,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_magic_light_2d::prelude::OmniLightSource2D;
use hexx::{hex, shapes};
use libnoise::Generator;

use super::tiles::TilePlugin;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_nodes, generate_scrap));
    }
}

fn generate_nodes(
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
    ];

    for hex in shapes::hexagon(hex(0, 0), 97) {
        let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);
        let noise = SIMPLEX_GENERATOR.sample([world_pos.x as f64, world_pos.y as f64]);

        println!("noise: {}", noise);

        if noise > resource_noise_tresholds::COAL.0 && noise < resource_noise_tresholds::COAL.1 {
            commands.spawn((
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
                ResourceNode {
                    coal_percent: 50,
                    mineral_percent: 50,
                    ticks_to_regen: 0,
                    resource_remaining: 1000,
                },
            ));

            resource_node_light(world_pos, &mut commands, constants::coal_node::COLOR);

            continue;
        }

        if noise > resource_noise_tresholds::MINERALS.0
            && noise < resource_noise_tresholds::MINERALS.1
        {
            commands.spawn((
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
                ResourceNode {
                    coal_percent: 50,
                    mineral_percent: 50,
                    ticks_to_regen: 0,
                    resource_remaining: 1000,
                },
            ));
            resource_node_light(world_pos, &mut commands, constants::mineral_node::COLOR);
            continue;
        }

        if noise > resource_noise_tresholds::SCRAP.0
            && noise < resource_noise_tresholds::SCRAP.1
        {
            commands.spawn((
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
                ResourceNode {
                    coal_percent: 50,
                    mineral_percent: 50,
                    ticks_to_regen: 0,
                    resource_remaining: 1000,
                },
            ));
            resource_node_light(world_pos, &mut commands, constants::scrap::COLOR);
            continue;
        }
    }
}

fn generate_scrap() {}

fn generate_mineral_nodes() {}

fn generate_coal_nodes() {}

fn resource_node_light(world_pos: Vec2, commands: &mut Commands, color: Color) {
    commands
        .spawn(OmniLightSource2D {
            intensity: 0.8,
            color,
            falloff: Vec3::new(1.5, 10.0, 0.005),
            ..default()
        })
        .insert(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(RenderLayers::all());
}
