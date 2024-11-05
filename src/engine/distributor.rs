use bevy::prelude::*;

use crate::{components::State, structure::distributor::spawn_distributor};

pub fn generate_distributors_from_keyframe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    for (_, chunk) in state.map.chunks.iter() {
        for (hex, distributor) in chunk.distributors.iter() {
            spawn_distributor(*hex, &mut commands, &asset_server, distributor.owner_id);
        }
    }
}