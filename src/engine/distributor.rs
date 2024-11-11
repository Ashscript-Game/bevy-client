use ashscript_types::{components::{owner::Owner, tile::Tile}, constants::map::CHUNK_SIZE};
use bevy::prelude::*;

use crate::{components::{LoadChunks, State, UnloadedChunks}, structure::distributor::spawn_distributor};

pub fn generate_distributors_from_keyframe(
    unloaded_chunks: Res<UnloadedChunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {

    for (entity, (_, tile, owner)) in state.world.query::<(&ashscript_types::components::distributor::Distributor, &Tile, &Owner)>().iter() {
        if !unloaded_chunks.0.contains(&tile.hex.to_lower_res(CHUNK_SIZE)) {
            continue;
        }

        spawn_distributor(tile.hex, &mut commands, &asset_server, owner.0);
    }
}