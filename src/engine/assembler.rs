use ashscript_types::{components::{owner::Owner, tile::Tile}, constants::map::CHUNK_SIZE};
use bevy::prelude::*;

use crate::{components::{LoadChunks, State, UnloadedChunks}, structure::assembler::spawn_assembler};

pub fn generate_assemblers_from_keyframe(
    unloaded_chunks: Res<UnloadedChunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {

    for (entity, (_, tile, owner)) in state.world.query::<(&ashscript_types::components::assembler::Assembler, &Tile, &Owner)>().iter() {
        if !unloaded_chunks.0.contains(&tile.hex.to_lower_res(CHUNK_SIZE)) {
            continue;
        }

        spawn_assembler(tile.hex, &mut commands, &asset_server, owner.0);
    }
}