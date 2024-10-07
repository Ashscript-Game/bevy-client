use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    prelude::*,
    utils::hashbrown::HashSet,
};
use hexx::{hex, shapes, Hex};
use rand::Rng;

use crate::{
    components::{MappedUnits, OccupiesTile},
    engine::terrain::HEX_LAYOUT,
    structure::{assembler::spawn_assembler, distributor::spawn_distributor, factory::spawn_factory, turret::spawn_turret},
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
    asset_server: Res<AssetServer>,
    occupiers: Query<&Transform, With<OccupiesTile>>,
    mut units: MappedUnits,
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
            spawn_unit(hex, &mut commands, &asset_server, &mut units, 0);
        }
    }
}

pub fn turret_benchmark(
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

    let mut rng = rand::thread_rng();

    for hex in shapes::hexagon(hex(0, 0), 10) {
        if occupied_tiles.contains(&hex) {
            continue;
        }

        if rng.gen_range(0..=5) == 0 { spawn_turret(hex, &mut commands, &asset_server) }
    }
}

pub fn factory_combat_benchmark(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // spawn 2 factories somewhat away from each other
    // each factory is owned by a different player
    // each factory produces units for free
    // units will try to attack the other factory

    spawn_factory(hex(8, 6), &mut commands, &asset_server, 0);
    spawn_factory(hex(-8, -3), &mut commands, &asset_server, 1);
}