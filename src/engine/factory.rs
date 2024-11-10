use ashscript_types::{components::{owner::Owner, tile::Tile}, constants::map::CHUNK_SIZE, structures::factory::Factory};
use bevy::prelude::*;
use hecs::With;

use crate::{components::{LoadChunks, State}, structure::factory::spawn_factory};

pub fn generate_factories_from_keyframe(
    trigger: Trigger<LoadChunks>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    let new_chunks = &trigger.event().0;

    for (entity, (_, tile, owner)) in state.world.query::<((&Factory, &Tile, &Owner))>().iter() {
        
        println!("factory");
        if !new_chunks.contains(&tile.hex.to_lower_res(CHUNK_SIZE)) {
            continue;
        }

        spawn_factory(tile.hex, &mut commands, &asset_server, owner.0);
    }
}