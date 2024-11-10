use ashscript_types::constants::map::{CHUNK_SIZE, HEX_LAYOUT};
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        view::RenderLayers,
    },
};
use bevy_magic_light_2d::prelude::CAMERA_LAYER_FLOOR;
use hexx::{hex, shapes, Hex, HexLayout, HexOrientation, PlaneMeshBuilder};
use rand::random;

use crate::components::{LoadChunks, State, TickEvent};

pub const HEX_SIZE: Vec2 = Vec2::splat(64.0);
const COLORS: [Color; 3] = [
    /* Color::BLUE, Color::WHITE, Color::RED, */
    Color::srgba(60. / 255., 60. / 255., 60. / 255., 1.),
    Color::srgba(65. / 255., 65. / 255., 65. / 255., 1.),
    Color::srgba(55. / 255., 55. / 255., 55. / 255., 1.),
];

pub fn generate_tiles(
    trigger: Trigger<LoadChunks>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<State>,
) {
    println!("generating tiles");

    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);

    let material_handles = [
        materials.add(ColorMaterial::from(COLORS[0])),
        materials.add(ColorMaterial::from(COLORS[1])),
        materials.add(ColorMaterial::from(COLORS[2])),
    ];

    let new_chunks = &trigger.event().0;

    for chunk_hex in new_chunks.iter() {
        for hex in shapes::hexagon(chunk_hex.to_higher_res(CHUNK_SIZE), CHUNK_SIZE) {
            generate_chunk(
                chunk_hex,
                hex,
                &mut commands,
                /* &material_handles, */
                &mesh_handle,
                &mut materials,
            );
        }
    }
}

fn generate_chunk(
    chunk_hex: &Hex,
    hex: Hex,
    commands: &mut Commands,
    /* material_handles: &[Handle<ColorMaterial>], */
    mesh_handle: &Handle<Mesh>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let pos = HEX_LAYOUT.hex_to_world_pos(hex);
    /*     let color_index = (chunk_hex.x - chunk_hex.y).rem_euclid(3);
    let material_handle = material_handles[color_index as usize].clone(); */
    let offset = /* (50. + random::<f32>() * 3.) */55. / 255.;
    let material_handle = materials.add(ColorMaterial::from(Color::srgba(
        offset, offset, offset, 1.,
    )));

    // let handle = materials.add(ColorMaterial::from(COLORS[0]));

    commands.spawn((
        ColorMesh2dBundle {
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            mesh: mesh_handle.clone().into(),
            material: material_handle,
            ..default()
        },
        RenderLayers::from_layers(CAMERA_LAYER_FLOOR),
    ));
}

pub fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        // < 1 creates borders around hexes
        .with_scale(Vec3::splat(1. /* 0.95 */))
        .facing(Vec3::Z)
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
