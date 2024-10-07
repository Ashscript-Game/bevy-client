use crate::{
    components::{OccupiesTile, ResourceNode, Scrap, Wall},
    constants::{self, resource_noise_tresholds, SIMPLEX_GENERATOR},
    engine::terrain::{hexagonal_plane, HEX_LAYOUT, HEX_SIZE},
};
use bevy::{
    math::Vec3,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_magic_light_2d::{
    gi::render_layer::ALL_LAYERS,
    prelude::{LightOccluder2D, OmniLightSource2D, CAMERA_LAYER_OBJECTS, CAMERA_LAYER_WALLS},
};
use hexx::{hex, shapes};
use libnoise::Generator;

pub fn generate_resources(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
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
            /* let secondary_occluder = commands
                .spawn((
                    Transform {
                        translation: Vec3 {
                            x: world_pos.x,
                            y: world_pos.y,
                            z: constants::resource_node::Z_POS,
                        },
                        ..default()
                    },
                    LightOccluder2D {
                        h_size: Vec2::new(1., HEX_SIZE.y),
                    },
                ))
                .id(); */

            commands
                .spawn((
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
                    Wall,
                    OccupiesTile,
                    RenderLayers::from_layers(CAMERA_LAYER_WALLS),
                    LightOccluder2D {
                        h_size: Vec2::new(HEX_SIZE.x, HEX_SIZE.x * 0.5),
                    },
                ))
                /* .add_child(secondary_occluder) */;

            continue;
        }

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
                OccupiesTile,
                ResourceNode {
                    coal_percent: 50,
                    mineral_percent: 50,
                    ticks_to_regen: 0,
                    resource_remaining: 1000,
                },
                RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
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
                OccupiesTile,
                ResourceNode {
                    coal_percent: 50,
                    mineral_percent: 50,
                    ticks_to_regen: 0,
                    resource_remaining: 1000,
                },
                RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
            ));
            resource_node_light(world_pos, &mut commands, constants::mineral_node::COLOR);
            continue;
        }

        if noise > resource_noise_tresholds::SCRAP.0 && noise < resource_noise_tresholds::SCRAP.1 {
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
                OccupiesTile,
                Scrap {
                    metal: 1000,
                    ticks_to_decay: 100,
                },
                RenderLayers::from_layers(CAMERA_LAYER_OBJECTS),
            ));
            resource_node_light(world_pos, &mut commands, constants::scrap::COLOR);
            continue;
        }
    }
}

fn resource_node_light(world_pos: Vec2, commands: &mut Commands, color: Color) {
    commands
        .spawn(OmniLightSource2D {
            intensity: 0.5,
            color,
            falloff: Vec3::new(20., 20., 0.005),
            jitter_intensity: 0.01,
            jitter_translation: 0.1
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
