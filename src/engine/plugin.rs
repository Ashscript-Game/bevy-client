use bevy::app::{App, Plugin, Startup, Update};

use super::{assembler::setup_assembler_replenishing, distributor::setup_distributor_replenishing, resource_blob::update_resource_blobs};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_distributor_replenishing, setup_assembler_replenishing)).add_systems(Update, update_resource_blobs);
    }
}