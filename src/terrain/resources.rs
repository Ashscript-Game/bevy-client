use bevy::{
    app::{App, Plugin, Startup, Update},
    math::Vec3,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_magic_light_2d::prelude::OmniLightSource2D;
use hexx::hex;

use crate::{
    components::ResourceNode,
    constants::{self, resource_node},
    terrain::tiles::{hexagonal_plane, HEX_LAYOUT},
};

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

    /*let mut point = hex(8, -12);
    let mut world_pos = HEX_LAYOUT.hex_to_world_pos(point);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(resource_node::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, resource_node::Z_POS),
                scale: Vec3::new(0.1, 0.1, 1.),
                ..default()
            },
            ..default()
        },
        ResourceNode {
            coal_percent: 50,
            mineral_percent: 50,
            ticks_to_regen: 0,
            resource_remaining: 1000,
        },
    ));

    point = hex(-8, 12);
    world_pos = HEX_LAYOUT.hex_to_world_pos(point);

     commands.spawn((
        SpriteBundle {
            texture: asset_server.load(resource_node::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(world_pos.x, world_pos.y, resource_node::Z_POS),
                scale: Vec3::new(0.1, 0.1, 1.),
                ..default()
            },
            ..default()
        },
        ResourceNode {
            coal_percent: 50,
            mineral_percent: 50,
            ticks_to_regen: 0,
            resource_remaining: 1000,
        },
    )); */

    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(ColorMaterial::from(constants::resource_node::COLOR));

    let node_hex_positions = [
        hex(8, -12),
        hex(-8, 12),
        hex(8, 12),
        hex(-8, -12),
        hex(20, 12),
        hex(-20, -12),
        hex(20, -12),
        hex(-20, 12),
        hex(20, 0),
        hex(-20, 0),
    ];

    for hex_pos in node_hex_positions {
        let world_pos = HEX_LAYOUT.hex_to_world_pos(hex_pos);

        commands.spawn((
            ColorMesh2dBundle {
                transform: Transform::from_xyz(
                    world_pos.x,
                    world_pos.y,
                    constants::resource_node::Z_POS,
                ),
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                ..default()
            },
            ResourceNode {
                coal_percent: 50,
                mineral_percent: 50,
                ticks_to_regen: 0,
                resource_remaining: 1000,
            },
        ));

        resource_node_light(world_pos, &mut commands);
    }
}

fn resource_node_light(world_pos: Vec2, commands: &mut Commands) {
    commands
        .spawn(OmniLightSource2D {
            intensity: 0.8,
            color: constants::resource_node::COLOR,
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

fn generate_scrap() {}
