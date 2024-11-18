use ashscript_types::{components::{owner::Owner, storage::Storage, tile::Tile}, constants::map::CHUNK_SIZE};
use bevy::prelude::*;
use hecs::With;

use crate::{components::{LoadChunks, State, UnloadedChunks}, structure::factory::spawn_factory};

pub fn generate_factories_from_keyframe(
    unloaded_chunks: Res<UnloadedChunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {

    for (entity, (_, tile, owner, storage)) in state.world.query::<((&ashscript_types::components::factory::Factory, &Tile, &Owner, &Storage))>().iter() {
        
        println!("new factory");
        if !unloaded_chunks.0.contains(&tile.hex.to_lower_res(CHUNK_SIZE)) {
            continue;
        }

        spawn_factory(tile.hex, &mut commands, &asset_server, owner.0, storage.clone());
    }
}