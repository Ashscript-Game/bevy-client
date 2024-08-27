use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    prelude::*, utils::hashbrown::HashSet,
};
use hexx::{hex, shapes, Hex};
use rand::Rng;

use crate::{components::OccupiesTile, engine::terrain::HEX_LAYOUT};

use super::{assembler::spawn_assembler, distributor::spawn_distributor};

pub struct StructureBenchmarks;

impl Plugin for StructureBenchmarks {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, assembler_distributor_benchmark);
    }
}

fn assembler_distributor_benchmark(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    occupiers: Query<&Transform, With<OccupiesTile>>,
) {
    let occupied_tiles: HashSet<Hex> = HashSet::from_iter(
        occupiers
            .iter()
            .map(|transform| HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate())).collect::<Vec<Hex>>(),
    );

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
