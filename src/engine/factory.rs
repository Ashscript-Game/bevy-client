use ashscript_types::{components::{owner::Owner, tile::Tile}, structures::factory::Factory};
use bevy::prelude::*;
use hecs::With;

use crate::{components::State, structure::factory::spawn_factory};

pub fn generate_factories_from_keyframe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    for (entity, (_, tile, owner)) in state.world.query::<((&Factory, &Tile, &Owner))>().iter() {
        spawn_factory(tile.hex, &mut commands, &asset_server, owner.0);
    }
}