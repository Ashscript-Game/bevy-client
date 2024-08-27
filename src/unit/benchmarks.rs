use bevy::{
    app::{App, Plugin},
    prelude::*,
    utils::hashbrown::HashSet,
};
use hexx::{hex, shapes, Hex};
use rand::Rng;

use crate::{components::OccupiesTile, engine::terrain::HEX_LAYOUT};

use super::plugin::spawn_unit;

pub struct UnitBenchmarks;

impl Plugin for UnitBenchmarks {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, unit_benchmark);
    }
}

fn unit_benchmark(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    occupiers: Query<&Transform, With<OccupiesTile>>,
) {
    let occupied_tiles: HashSet<Hex> = HashSet::from_iter(
        occupiers
            .iter()
            .map(|transform| HEX_LAYOUT.world_pos_to_hex(transform.translation.truncate())),
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
