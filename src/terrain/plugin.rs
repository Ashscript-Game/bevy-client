use bevy::app::{App, Plugin, Update};

use super::{resources::ResourcesPlugin, tiles::TilePlugin};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TilePlugin, ResourcesPlugin));
    }
}