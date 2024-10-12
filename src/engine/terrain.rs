use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        view::RenderLayers,
    },
};
use hexx::{hex, shapes, HexLayout, HexOrientation, PlaneMeshBuilder};

pub const HEX_SIZE: Vec2 = Vec2::splat(64.0);
pub const CHUNK_SIZE: u32 = 5;
const COLORS: [Color; 3] = [
    /* Color::BLUE, Color::WHITE, Color::RED, */
    Color::srgba(60. / 255., 60. / 255., 60. / 255., 1.),
    Color::srgba(65. / 255., 65. / 255., 65. / 255., 1.),
    Color::srgba(55. / 255., 55. / 255., 55. / 255., 1.),
];

pub const HEX_LAYOUT: HexLayout = HexLayout {
    hex_size: HEX_SIZE,
    orientation: HexOrientation::Pointy,
    origin: Vec2::new(0., 0.),
    invert_x: false,
    invert_y: false,
};

pub fn generate_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("generating tiles");

    let mesh = hexagonal_plane(&HEX_LAYOUT);
    let mesh_handle = meshes.add(mesh);

    let handles = [
        materials.add(ColorMaterial::from(COLORS[0])),
        materials.add(ColorMaterial::from(COLORS[1])),
        materials.add(ColorMaterial::from(COLORS[2])),
    ];

    for hex in shapes::hexagon(hex(0, 0), 97) {
        let pos = HEX_LAYOUT.hex_to_world_pos(hex);
        let hex_mod = hex.to_lower_res(CHUNK_SIZE);
        let color_index = (hex_mod.x - hex_mod.y).rem_euclid(3);
        let material_handle = handles[color_index as usize].clone();

        // let handle = materials.add(ColorMaterial::from(COLORS[0]));

        commands
            .spawn((ColorMesh2dBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                mesh: mesh_handle.clone().into(),
                material: material_handle,
                ..default()
            }));
    }
}

pub fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        // < 1 creates borders around hexes
        .with_scale(Vec3::splat(1./* 0.95 */))
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
