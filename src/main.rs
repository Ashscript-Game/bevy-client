use bevy::{app::App, DefaultPlugins};
use controls::camera::CameraControlsPlugin;
use game::GamePlugin;
use terrain::tiles::TilePlugin;

pub mod game;
pub mod terrain;
pub mod controls;
pub mod constants;
pub mod components;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin))
    .run();
}