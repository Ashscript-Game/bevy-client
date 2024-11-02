use crate::{
    components::{Lava, OccupiesTile, ResourceNode, Scrap, State, Wall},
    constants::{self, resource_noise_tresholds, SIMPLEX_GENERATOR},
    engine::terrain::{hexagonal_plane, HEX_LAYOUT, HEX_SIZE},
};
use ashscript_types::terrain::Terrain;
use bevy::{ecs::world, math::Vec3, prelude::*, render::view::RenderLayers};
use bevy_magic_light_2d::{
    gi::render_layer::ALL_LAYERS,
    prelude::{LightOccluder2D, OmniLightSource2D, CAMERA_LAYER_OBJECTS, CAMERA_LAYER_WALLS},
};
use hexx::{hex, shapes, Hex};
use libnoise::Generator;

pub fn generate_resources_from_keyframe(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<State>,
) {
    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);

    let material_handles = [
        materials.add(ColorMaterial::from(constants::coal_node::COLOR)),
        materials.add(ColorMaterial::from(constants::mineral_node::COLOR)),
        materials.add(ColorMaterial::from(constants::scrap::COLOR)),
        materials.add(ColorMaterial::from(constants::wall::COLOR)),
        materials.add(ColorMaterial::from(constants::lava::COLOR)),
    ];

    for (_, chunk) in state.map.chunks.iter() {
        for (hex, node) in chunk.coal_nodes.iter() {
            generate_resource(
                &mut commands,
                &_asset_server,
                &mesh_handle,
                &material_handles[0],
                *hex,
                node.amount,
                constants::coal_node::COLOR,
            );
        }

        for (hex, node) in chunk.mineral_nodes.iter() {
            generate_resource(
                &mut commands,
                &_asset_server,
                &mesh_handle,
                &material_handles[1],
                *hex,
                node.amount,
                constants::mineral_node::COLOR,
            );
        }

        for (hex, node) in chunk.scrap.iter() {
            generate_resource(
                &mut commands,
                &_asset_server,
                &mesh_handle,
                &material_handles[2],
                *hex,
                node.amount,
                constants::scrap::COLOR,
            );
        }

        for (hex, terrain) in chunk.terrain.iter() {
            generate_terrain(
                &mut commands,
                &_asset_server,
                &mesh_handle,
                &material_handles,
                *hex,
                terrain,
            );
        }
    }
}

fn generate_resource(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    mesh: &Handle<Mesh>,
    material: &Handle<ColorMaterial>,
    hex: Hex,
    amount: u32,
    color: Color,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    commands.spawn((
        ColorMesh2dBundle {
            transform: Transform::from_xyz(
                world_pos.x,
                world_pos.y,
                constants::resource_node::Z_POS,
            ),
            mesh: mesh.clone().into(),
            material: material.clone(),
            ..default()
        },
        OccupiesTile,
        ResourceNode {
            coal_percent: 50,
            mineral_percent: 50,
            ticks_to_regen: 0,
            resource_remaining: amount,
        },
        RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
    ));

    resource_node_light(world_pos, commands, color);
}

fn generate_terrain(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    mesh: &Handle<Mesh>,
    materials: &[Handle<ColorMaterial>],
    hex: Hex,
    terrain: &Terrain,
) {
    let world_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    match terrain {
        Terrain::Wall => {
            commands.spawn((
                ColorMesh2dBundle {
                    transform: Transform::from_xyz(
                        world_pos.x,
                        world_pos.y,
                        constants::resource_node::Z_POS,
                    ),
                    mesh: mesh.clone().into(),
                    material: materials[3].clone(),
                    ..default()
                },
                Wall,
                OccupiesTile,
                RenderLayers::from_layers(CAMERA_LAYER_WALLS),
                LightOccluder2D {
                    h_size: Vec2::new(HEX_SIZE.x, HEX_SIZE.x * 0.5),
                },
            ));
        }
        Terrain::Lava => {
            commands.spawn((
                ColorMesh2dBundle {
                    transform: Transform::from_xyz(
                        world_pos.x,
                        world_pos.y,
                        constants::resource_node::Z_POS,
                    ),
                    mesh: mesh.clone().into(),
                    material: materials[3].clone(),
                    ..default()
                },
                Lava,
                OccupiesTile,
                RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
            ));

            resource_node_light(world_pos, commands, constants::lava::COLOR);
        }
        _ => {}
    }
}

fn resource_node_light(world_pos: Vec2, commands: &mut Commands, color: Color) {
    commands
        .spawn(OmniLightSource2D {
            intensity: 0.5,
            color,
            falloff: Vec3::new(20., 20., 0.005),
            jitter_intensity: 0.01,
            jitter_translation: 0.1,
        })
        .insert(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(RenderLayers::from_layers(ALL_LAYERS));
}
