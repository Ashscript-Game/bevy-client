use bevy::{app::App, DefaultPlugins};
use game::GamePlugin;

pub mod game;
pub mod tiles;
pub mod controls;
pub mod constants;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin))
    .run();
}