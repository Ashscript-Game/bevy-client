use bevy::app::{App, Plugin, Update};

use super::distributor::distributor_ai;

pub struct PlayerScriptPlugin;

impl Plugin for PlayerScriptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, distributor_ai);
    }
}