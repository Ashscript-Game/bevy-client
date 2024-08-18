use bevy::{
    app::{App, Plugin, Startup},
    prelude::*,
};

use crate::{
    controls::{camera::CameraControlsPlugin, plugin::ControlsPlugin},
    terrain::{plugin::TerrainPlugin, tiles::TilePlugin},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TerrainPlugin, ControlsPlugin))
            .add_systems(Startup, game_init);
    }
}

fn game_init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
