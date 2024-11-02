use bevy::prelude::*;

use crate::{components::State, structure::assembler::spawn_assembler};

pub fn generate_assemblers_from_keyframe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    for (_, chunk) in state.map.chunks.iter() {
        for (hex, assembler) in chunk.assemblers.iter() {
            spawn_assembler(*hex, &mut commands, &asset_server, assembler.owner_id);
        }
    }
}