use bevy::app::{App, Plugin, Startup};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(Startup, game_init);
    }
}

fn game_init() {

}