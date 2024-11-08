use ashscript_types::{components::{owner::Owner, tile::Tile}, structures::distributor::Distributor};
use bevy::prelude::*;

use crate::{components::State, structure::assembler::spawn_assembler};

pub fn generate_assemblers_from_keyframe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    for (entity, (_, tile, owner)) in state.world.query::<(&Distributor, &Tile, &Owner)>().iter() {
        spawn_assembler(tile.hex, &mut commands, &asset_server, owner.0);
    }
}