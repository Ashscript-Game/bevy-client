use bevy::app::{App, Plugin, Startup};

use super::{assembler::setup_assembler_replenishing, distributor::setup_distributor_replenishing};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_distributor_replenishing, setup_assembler_replenishing));
    }
}