use bevy::prelude::*;

use crate::{components::State, structure::factory::spawn_factory};

pub fn generate_factories_from_keyframe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    for (_, chunk) in state.map.chunks.iter() {
        for (hex, factory) in chunk.factories.iter() {
            spawn_factory(*hex, &mut commands, &asset_server, factory.owner_id);
        }
    }
}