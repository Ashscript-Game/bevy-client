use ashscript_types::{components::{owner::Owner, tile::Tile}, constants::map::CHUNK_SIZE};
use bevy::prelude::*;

use crate::{components::{LoadChunks, State}, structure::distributor::spawn_distributor};

pub fn generate_distributors_from_keyframe(
    trigger: Trigger<LoadChunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    let new_chunks = &trigger.event().0;

    for (entity, (_, tile, owner)) in state.world.query::<(&ashscript_types::components::distributor::Distributor, &Tile, &Owner)>().iter() {
        if !new_chunks.contains(&tile.hex.to_lower_res(CHUNK_SIZE)) {
            continue;
        }

        spawn_distributor(tile.hex, &mut commands, &asset_server, owner.0);
    }
}