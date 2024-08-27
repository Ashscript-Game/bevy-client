use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    prelude::*,
    utils::hashbrown::HashSet,
};
use hexx::{hex, shapes, Hex};
use rand::Rng;

use crate::{
    components::OccupiesTile,
    engine::terrain::HEX_LAYOUT,
    structure::{assembler::spawn_assembler, distributor::spawn_distributor},
    unit::plugin::spawn_unit,
};

pub fn assembler_distributor_benchmark(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    occupiers: Query<&Transform, With<OccupiesTile>>,
) {
    let occupied_tiles: HashSet<Hex> = HashSet::from_iter(
        occupiers
            .iter()
            .map(|transform| HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()))
            .collect::<Vec<Hex>>(),
    );

    /* let mut occupied_tiles = HashSet::new();
    for transform in occupiers.iter() {
        occupied_tiles.insert(HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()));
    } */

    println!("occupied tiles: {:?}", occupied_tiles);

    let mut rng = rand::thread_rng();

    for hex in shapes::hexagon(hex(0, 0), 10) {
        if occupied_tiles.contains(&hex) {
            continue;
        }

        match rng.gen_range(0..=1) {
            0 => spawn_assembler(hex, &mut commands, &asset_server),
            _ => spawn_distributor(hex, &mut commands, &asset_server),
        }
    }
}

pub fn unit_benchmark(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    occupiers: Query<&Transform, With<OccupiesTile>>,
) {
    let occupied_tiles: HashSet<Hex> = HashSet::from_iter(
        occupiers
            .iter()
            .map(|transform| HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate()))
            .collect::<Vec<Hex>>(),
    );

    let mut rng = rand::thread_rng();

    for hex in shapes::hexagon(hex(0, 0), 30) {
        if occupied_tiles.contains(&hex) {
            continue;
        }

        if rng.gen_range(0..=5) == 0 {
            spawn_unit(hex, &mut commands, &mut meshes, &mut materials)
        }
    }
}
